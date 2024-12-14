use graphical_interface::{
    graphical_buffer::GraphicalBuffer, graphical_printer::GraphicalPrinter,
    graphical_screen::GraphicalScreen,
};

use crate::{
    bitmap_utils::bitmap::Bitmap,
    game_controller::GameController,
    game_states::{game_state::GameStateEnum, menu::Menu},
    task_scheduler::TaskScheduler,
    utils::XY,
    view::View,
    window_setup::{
        terminal_screen::TerminalHelper,
        window::{GnomeTerminal, Terminal, WindowCreator},
    },
};
use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
    thread::{self},
};

pub mod asset_server;
pub mod bitmap_utils;
pub mod collision_detector;
pub mod drawable_objects;
pub mod game_controller;
pub mod game_states;
pub mod graphical_interface;
pub mod task_scheduler;
pub mod utils;
pub mod view;
pub mod window_setup;

lazy_static::lazy_static! {
    static ref SPEED: Mutex<f32> = Mutex::new(1.0);
}

const BORDER_WIDTH: XY<usize> = XY::new(2, 1);
const WINDOW_RESOLUTION: XY<usize> = XY::new(160, 40);
const FPS_LIMIT: f32 = 60.0; // may be buggy above ~46
const SPEEDUP_RATE: f32 = 1.0003;

fn main() {
    let gnome_window = GnomeTerminal::new();

    let (tx, rx): (Sender<char>, Receiver<char>) = mpsc::channel();
    thread::spawn(move || loop {
        let input = gnome_window.read_key();
        if let Some(pressed_key) = input {
            tx.send(pressed_key).unwrap();
        }
    });

    let asset_path = "/home/user/Codes/GithubRepos/uni-console-dino/src/assets/";
    let buffer = GraphicalBuffer::new(&Bitmap::new(WINDOW_RESOLUTION, ' '));
    let printer = GraphicalPrinter::new(XY::new(WINDOW_RESOLUTION.x * 8, WINDOW_RESOLUTION.y * 18));
    let view = View::new(asset_path, ' ');
    let screen = GraphicalScreen::new(buffer, printer, BORDER_WIDTH);
    let task_scheduler = TaskScheduler::new();
    let mut game_controller = GameController::new(
        view,
        screen,
        rx,
        GameStateEnum::Menu(Box::new(Menu)),
        task_scheduler,
    );
    game_controller.run();
}
