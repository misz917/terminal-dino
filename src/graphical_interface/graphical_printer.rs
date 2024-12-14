use crate::{
    asset_server::TRANSPARENT_CHAR,
    bitmap_utils::bitmap::Bitmap,
    utils::{ESC, XY},
    WINDOW_RESOLUTION,
};
use minifb::Window;

pub trait Printer {
    fn print(&self, bitmap: &Bitmap<char>, border_width: &XY<usize>);
}

pub struct GraphicalPrinter {
    window: Window,
}
impl Printer for GraphicalPrinter {
    fn print(&self, bitmap: &Bitmap<char>, border_width: &XY<usize>) {
        for (i, row) in bitmap.matrix.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if *item == TRANSPARENT_CHAR {
                    continue;
                }
                print!(
                    "{}[{};{}H{}",
                    ESC,
                    i + 1 + border_width.y,
                    j + 1 + border_width.x,
                    item
                );
            }
        }
    }
}

fn chars_to_pixels(bitmap: &Bitmap<char>) -> Bitmap<bool> {
    let mut output: Bitmap<bool> = Bitmap::new(WINDOW_RESOLUTION, false);
    for (i, row) in bitmap.matrix.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            if *item == TRANSPARENT_CHAR || *item == ' ' {
                continue;
            } else {
                output.matrix[i][j] = true;
            }
        }
    }
    return output;
}
