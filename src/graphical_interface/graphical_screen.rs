use crate::{
    bitmap_utils::{bitmap::Bitmap, bitmap_buffer::BufferManager, bitmap_printer::Printer},
    utils::XY,
};

pub struct GraphicalScreen<B: BufferManager, P: Printer> {
    buffer: B,
    printer: P,
    border_width: XY<usize>,
}
impl<B: BufferManager, P: Printer> GraphicalScreen<B, P> {
    pub fn new(buffer: B, printer: P, border_width: XY<usize>) -> Self {
        GraphicalScreen {
            buffer,
            printer,
            border_width,
        }
    }

    pub fn schedule_frame(&mut self, new_frame: Box<Bitmap<char>>) {
        self.buffer.insert_frame(new_frame);
    }

    pub fn display_frame(&mut self) {
        if let Some(frame) = self.buffer.get_frame() {
            self.printer.print(&frame, &self.border_width);
        }
    }

    pub fn prepare() {}
}
