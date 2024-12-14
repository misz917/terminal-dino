use std::{
    collections::HashMap,
    sync::mpsc::Receiver,
    thread::sleep,
    time::{Duration, SystemTime},
};

use crate::{
    bitmap_utils::{bitmap_buffer::BufferManager, bitmap_printer::Printer},
    drawable_objects::{drawable_object::DrawableObject, label::Label},
    game_states::game_state::GameStateEnum,
    graphical_interface::graphical_screen::GraphicalScreen,
    task_scheduler::TaskScheduler,
    utils::{Value, XY},
    view::View,
    window_setup::terminal_screen::Screen,
    FPS_LIMIT, WINDOW_RESOLUTION,
};

pub struct GameController<B: BufferManager> {
    frame_counter: u32,
    view: View,
    screen: GraphicalScreen<B>,
    rx: Receiver<char>,
    active_state: GameStateEnum,
    task_scheduler: TaskScheduler,
    resources: HashMap<String, Value>,
}
impl<B: BufferManager> GameController<B> {
    pub fn new(
        view: View,
        screen: GraphicalScreen<B>,
        rx: Receiver<char>,
        default_game_state: GameStateEnum,
        task_scheduler: TaskScheduler,
    ) -> Self {
        GameController {
            frame_counter: 0,
            view,
            screen,
            rx,
            active_state: default_game_state,
            task_scheduler,
            resources: HashMap::new(),
        }
    }

    fn display_frame_counter(&mut self) {
        let label = DrawableObject::Label(Label::new(&format!("{}", self.frame_counter)));
        self.view.remove_object("frame_count");
        self.view.insert_object(
            "frame_count",
            i16::max_value(),
            false,
            label,
            XY::new(
                (WINDOW_RESOLUTION.x - 1 - (f32::log10(self.frame_counter as f32)) as usize) as i32,
                (WINDOW_RESOLUTION.y - 1) as i32,
            ),
            None,
        );
    }

    pub fn run(&mut self) {
        let mut state_change_listener: Option<GameStateEnum> = None;
        self.active_state.as_state().on_enter(
            &mut self.view,
            &mut self.task_scheduler,
            &mut self.resources,
        );

        loop {
            let timer = SystemTime::now();
            self.display_frame_counter();

            self.screen.printer.read_keys();

            if let Ok(input) = self.rx.try_recv() {
                self.active_state.as_state().handle_input(
                    &mut self.view,
                    input,
                    &mut state_change_listener,
                    &mut self.task_scheduler,
                    &mut self.resources,
                );
            }

            if let Some(task) = self.task_scheduler.get_task() {
                if task.game_state.is_none()
                    || task.game_state.is_some()
                        && self
                            .active_state
                            .variant_eq(&task.game_state.clone().unwrap())
                {
                    let follow_up_task = task.execute(&mut self.view);
                    if let Some(new_task) = follow_up_task {
                        self.task_scheduler.schedule(new_task);
                    }
                }
            }

            self.active_state.as_state().each_frame(
                &mut self.view,
                &mut state_change_listener,
                &mut self.task_scheduler,
                &mut self.resources,
            );

            if let Some(ref new_state) = state_change_listener {
                self.change_state(new_state.clone());
                state_change_listener = None;
            }

            self.screen.schedule_frame(self.view.compile());
            self.screen.display_frame();

            Self::enforce_fps(timer);
            self.frame_counter += 1;
        }
    }

    fn change_state(&mut self, new_state: GameStateEnum) {
        self.active_state.as_state().on_exit(
            &mut self.view,
            &mut self.task_scheduler,
            &mut self.resources,
        );
        self.active_state = new_state;
        self.active_state.as_state().on_enter(
            &mut self.view,
            &mut self.task_scheduler,
            &mut self.resources,
        );
    }

    fn enforce_fps(timer: SystemTime) {
        let sleep_duration = 1.0 / FPS_LIMIT;
        if let Ok(elapsed) = timer.elapsed() {
            if Duration::from_secs_f32(sleep_duration) > elapsed {
                sleep(Duration::from_secs_f32(sleep_duration) - elapsed);
            }
        }
    }
}
