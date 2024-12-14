use std::{
    sync::mpsc::Sender,
    time::{SystemTime, UNIX_EPOCH},
};

use super::{char_to_pixels::char_to_5x5, upscaler::upscale};
use crate::{
    asset_server::TRANSPARENT_CHAR,
    bitmap_utils::{bitmap::Bitmap, bitmap_printer::Printer},
    utils::XY,
    WINDOW_RESOLUTION,
};
use minifb::{Key, Window, WindowOptions};

pub struct GraphicalPrinter {
    window: Window,
    tx: Sender<char>,
}
impl Printer for GraphicalPrinter {
    fn print(&mut self, bitmap: &Bitmap<char>, _border_width: &XY<usize>) {
        let pixels = &chars_to_pixels(&bitmap);
        let upscaled_pixels = upscale(pixels.matrix.clone(), 0);
        let bitmap = Bitmap {
            resolution: XY::new(upscaled_pixels[0].len(), upscaled_pixels.len()),
            matrix: upscaled_pixels,
        };
        let rgb = bools_to_rgb(&bitmap);
        let _ = self
            .window
            .update_with_buffer(&rgb, bitmap.resolution.x, bitmap.resolution.y);
    }
}

impl GraphicalPrinter {
    pub fn new(resolution: XY<usize>, tx: Sender<char>) -> Self {
        GraphicalPrinter {
            window: Window::new("", resolution.x, resolution.y, WindowOptions::default()).unwrap(),
            tx,
        }
    }

    pub fn read_keys(&mut self) {
        let keys = self.window.get_keys_pressed(minifb::KeyRepeat::No);
        if keys.len() == 0 {
            return;
        };
        match keys[0] {
            Key::W => {
                let _ = self.tx.send('w');
            }
            Key::S => {
                let _ = self.tx.send('s');
            }
            Key::D => {
                let _ = self.tx.send('d');
            }
            _ => {}
        }
    }
}

fn chars_to_pixels(bitmap: &Bitmap<char>) -> Bitmap<bool> {
    let mut output: Bitmap<bool> = Bitmap::new(
        XY::new(bitmap.resolution.x * 5, bitmap.resolution.y * 5),
        false,
    );
    for (i, row) in bitmap.matrix.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            if *item == TRANSPARENT_CHAR || *item == ' ' {
                continue;
            }
            if let Some(arr) = char_to_5x5(*item) {
                for dx in 0..5 {
                    for dy in 0..5 {
                        output.matrix[i * 5 + dx][j * 5 + dy] = arr[dx][dy];
                    }
                }
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
                let d = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as f64
                    / 1000.0;
                (0xFF0000 + (d.sin() * 255.0) as u32) as u32
            } else {
                0x000000 // Empty space
            };
        }
    }

    output
}
