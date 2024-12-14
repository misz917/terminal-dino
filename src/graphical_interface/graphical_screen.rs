use crate::{
    bitmap_utils::{bitmap::Bitmap, bitmap_buffer::BufferManager, bitmap_printer::Printer},
    utils::XY,
    window_setup::terminal_screen::Screen,
};

use super::graphical_printer::GraphicalPrinter;

pub struct GraphicalScreen<B: BufferManager> {
    buffer: B,
    pub printer: GraphicalPrinter,
    border_width: XY<usize>,
}
impl<B: BufferManager> Screen for GraphicalScreen<B> {
    fn schedule_frame(&mut self, new_frame: Box<Bitmap<char>>) {
        self.buffer.insert_frame(new_frame);
    }

    fn display_frame(&mut self) {
        if let Some(frame) = self.buffer.get_frame() {
            self.printer.print(&frame, &self.border_width);
        }
    }

    fn prepare() {}
}

impl<B: BufferManager> GraphicalScreen<B> {
    pub fn new(buffer: B, printer: GraphicalPrinter, border_width: XY<usize>) -> Self {
        GraphicalScreen {
            buffer,
            printer,
            border_width,
        }
    }
}
