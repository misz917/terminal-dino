use crate::{
    asset_server::TRANSPARENT_CHAR, bitmap_utils::bitmap::Bitmap, utils::XY, WINDOW_RESOLUTION,
};
use minifb::{Window, WindowOptions};

pub trait Printer {
    fn print(&mut self, bitmap: &Bitmap<char>, border_width: &XY<usize>);
}

pub struct GraphicalPrinter {
    window: Window,
}
impl Printer for GraphicalPrinter {
    fn print(&mut self, bitmap: &Bitmap<char>, _border_width: &XY<usize>) {
        let buffer = bools_to_rgb(&chars_to_pixels(&bitmap));
        self.window
            .update_with_buffer(&buffer, bitmap.resolution.x, bitmap.resolution.y);
    }
}

impl GraphicalPrinter {
    pub fn new(resolution: XY<usize>) -> Self {
        GraphicalPrinter {
            window: Window::new("", resolution.x, resolution.y, WindowOptions::default()).unwrap(),
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

fn bools_to_rgb(bitmap: &Bitmap<bool>) -> Vec<u32> {
    let mut output: Vec<u32> = vec![0; bitmap.resolution.x * bitmap.resolution.y];

    for y in 0..bitmap.resolution.y {
        for x in 0..bitmap.resolution.x {
            let index = y * bitmap.resolution.x + x;

            output[index] = if bitmap.matrix[y][x] {
                0xFFFFFF // Pixel (white)
            } else {
                0x000000 // Empty space (black)
            };
        }
    }

    output
}
