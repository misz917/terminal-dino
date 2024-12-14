use crate::{
    bitmap_utils::{bitmap::Bitmap, bitmap_buffer::BufferManager, bitmap_printer::Printer},
    utils::{ESC, RGB, XY},
};
use std::io::{self, Write};

pub struct TerminalHelper;
impl TerminalHelper {
    pub fn prepare_terminal() {
        Self::set_character_color(RGB::new(255, 255, 255));
        Self::set_background_color(RGB::new(10, 50, 150));
        Self::disable_cursor_visibility();
        Self::enable_bold_mode();
        Self::move_cursor_home();
    }

    pub fn flush_terminal_buffer() {
        io::stdout().flush().unwrap();
    }

    fn move_cursor_home() {
        print!("{}[H", ESC);
    }

    pub fn set_background_color(rgb: RGB) {
        print!("{}[48;2;{};{};{}m", ESC, rgb.r, rgb.g, rgb.b);
    }

    pub fn set_character_color(rgb: RGB) {
        print!("{}[38;2;{};{};{}m", ESC, rgb.r, rgb.g, rgb.b);
    }

    fn disable_cursor_visibility() {
        print!("{}[?25l", ESC);
    }

    fn enable_bold_mode() {
        print!("{}[1m", ESC);
    }
}

pub trait Screen {
    fn schedule_frame(&mut self, new_frame: Box<Bitmap<char>>);
    fn display_frame(&mut self);
    fn prepare();
}

pub struct TerminalScreen<B: BufferManager, P: Printer> {
    buffer: B,
    printer: P,
    border_width: XY<usize>,
}
impl<B: BufferManager, P: Printer> Screen for TerminalScreen<B, P> {
    fn schedule_frame(&mut self, new_frame: Box<Bitmap<char>>) {
        self.buffer.insert_frame(new_frame);
    }

    fn display_frame(&mut self) {
        if let Some(frame) = self.buffer.get_frame() {
            self.printer.print(&frame, &self.border_width);
        }
        TerminalHelper::flush_terminal_buffer();
    }

    fn prepare() {
        TerminalHelper::prepare_terminal();
    }
}

impl<B: BufferManager, P: Printer> TerminalScreen<B, P> {
    pub fn new(buffer: B, printer: P, border_width: XY<usize>) -> Self {
        TerminalScreen {
            buffer,
            printer,
            border_width,
        }
    }
}
