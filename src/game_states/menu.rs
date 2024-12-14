use super::{
    game_state::{GameState, GameStateEnum},
    main_game_loop::MainGameLoop,
};
use crate::{
    drawable_objects::{drawable_object::DrawableObject, label::Label, rectangle::Rectangle},
    task_scheduler::{Task, TaskScheduler},
    utils::{Value, XY},
    view::View,
    FPS_LIMIT, SMOOTHING_POWER, WINDOW_RESOLUTION,
};
use std::{collections::HashMap, process::exit, time::Duration};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Menu;
impl GameState for Menu {
    fn handle_input(
        &mut self,
        _view: &mut View,
        input: char,
        state_changer: &mut Option<GameStateEnum>,
        _task_scheduler: &mut TaskScheduler,
        _resources: &mut HashMap<String, Value>,
    ) {
        let y;
        let mut change_colorscheme = false;
        if let Some(Value::I32(value)) = _resources.get_mut("selected_option") {
            match input {
                'w' => *value += 2,
                's' => *value += 1,
                'd' => match *value {
                    0 => *state_changer = Some(GameStateEnum::MainGameLoop(Box::new(MainGameLoop))),
                    1 => change_colorscheme = true,
                    2 => exit(0),
                    _ => (),
                },
                _ => (),
            }
            *value = *value % 3;
            y = *value;
        } else {
            panic!("Expected `selected_option` to be an I32 value.");
        }

        if change_colorscheme {
            if let Some(Value::I32(color)) = _resources.get_mut("active_color_scheme") {
                *color = (*color + 1) % 3;
                set_color_scheme(*color);
            }
            _view.insert_object(
                "rectangle",
                100,
                false,
                DrawableObject::Rectangle(Rectangle::new(WINDOW_RESOLUTION, '.')),
                XY::new(0, 0),
                None,
            );

            _task_scheduler.schedule(Task::new(
                forced_screen_redraw,
                Duration::from_secs_f32(2.0 / FPS_LIMIT),
                None,
            ));
        }

        _view.remove_object("pointer");
        _view.insert_object(
            "pointer",
            -1,
            false,
            DrawableObject::Label(Label::new(">---------------------<")),
            XY::new(
                (WINDOW_RESOLUTION.x - 24) as i32 / 2,
                (WINDOW_RESOLUTION.y - (14 - y * 2) as usize) as i32,
            ),
            None,
        );
    }

    fn on_enter(
        &mut self,
        view: &mut View,
        _task_scheduler: &mut TaskScheduler,
        _resources: &mut HashMap<String, Value>,
    ) {
        _resources.insert("selected_option".to_owned(), Value::I32(0));
        _resources.insert("active_color_scheme".to_owned(), Value::I32(0));

        view.insert_object(
            "pointer",
            -1,
            false,
            DrawableObject::Label(Label::new(">---------------------<")),
            XY::new(
                (WINDOW_RESOLUTION.x - 24) as i32 / 2,
                (WINDOW_RESOLUTION.y - 14) as i32,
            ),
            None,
        );

        view.insert_asset(
            "title_sign",
            1,
            false,
            "welcome_screen.txt",
            XY::new(
                ((WINDOW_RESOLUTION.x - 73) / 2) as i32,
                (WINDOW_RESOLUTION.y / 2 - 8) as i32,
            ),
            None,
        );

        let text = "Start  game";
        view.insert_object(
            "start_game_label",
            2,
            false,
            DrawableObject::Label(Label::new(text)),
            XY::new(
                (WINDOW_RESOLUTION.x - text.len()) as i32 / 2,
                (WINDOW_RESOLUTION.y - 14) as i32,
            ),
            None,
        );

        let text = "Change  smoothing";
        view.insert_object(
            "change_color_scheme_label",
            2,
            false,
            DrawableObject::Label(Label::new(text)),
            XY::new(
                (WINDOW_RESOLUTION.x - text.len()) as i32 / 2,
                (WINDOW_RESOLUTION.y - 12) as i32,
            ),
            None,
        );

        let text = "Exit game";
        view.insert_object(
            "exit_game_label",
            2,
            false,
            DrawableObject::Label(Label::new(text)),
            XY::new(
                (WINDOW_RESOLUTION.x - text.len()) as i32 / 2,
                (WINDOW_RESOLUTION.y - 10) as i32,
            ),
            None,
        );

        let text = "W = up, S = down, D = select";
        view.insert_object(
            "exit_game_label",
            2,
            false,
            DrawableObject::Label(Label::new(text)),
            XY::new(
                (WINDOW_RESOLUTION.x - text.len()) as i32 / 2,
                (WINDOW_RESOLUTION.y - 2) as i32,
            ),
            None,
        );
    }

    fn on_exit(
        &mut self,
        _view: &mut View,
        _task_scheduler: &mut TaskScheduler,
        _resources: &mut HashMap<String, Value>,
    ) {
        _resources.remove("selected_option");
        _resources.remove("active_color_scheme");
        _view.remove_object("*");
    }

    fn each_frame(
        &mut self,
        _view: &mut View,
        _state_changer: &mut Option<GameStateEnum>,
        _task_scheduler: &mut TaskScheduler,
        _resources: &mut HashMap<String, Value>,
    ) {
        return;
    }
}

fn set_color_scheme(num: i32) {
    match num {
        0 => {
            let mut smoothing = SMOOTHING_POWER.lock().unwrap();
            *smoothing = 0;
        }
        1 => {
            let mut smoothing = SMOOTHING_POWER.lock().unwrap();
            *smoothing = 1;
        }
        2 => {
            let mut smoothing = SMOOTHING_POWER.lock().unwrap();
            *smoothing = 2;
        }
        _ => (),
    }
}

fn forced_screen_redraw(view: &mut View) -> Option<Task> {
    view.remove_object("rectangle");
    return None;
}
