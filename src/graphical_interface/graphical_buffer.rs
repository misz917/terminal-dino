use crate::{
    bitmap_utils::{bitmap::Bitmap, bitmap_buffer::BufferManager},
    utils::XY,
};

#[derive(Clone)]
pub struct GraphicalBuffer {
    active_frame: Box<Bitmap<char>>,
    following_frame: Option<Box<Bitmap<char>>>,
    resolution: XY<usize>,
}
impl GraphicalBuffer {
    pub fn new(first_frame: &Bitmap<char>) -> Self {
        GraphicalBuffer {
            active_frame: Box::new(first_frame.clone()),
            following_frame: None,
            resolution: first_frame.resolution,
        }
    }
}

impl BufferManager for GraphicalBuffer {
    fn insert_frame(&mut self, new_frame: Box<Bitmap<char>>) {
        if new_frame.resolution != self.resolution {
            panic!()
        }

        if let Some(bitmap) = self.following_frame.take() {
            self.active_frame = bitmap;
        }

        self.following_frame = Some(new_frame);
    }

    fn get_frame(&mut self) -> Option<Box<Bitmap<char>>> {
        if let Some(following_frame) = &self.following_frame {
            let mut differences = *self.active_frame.clone();
            for row in 0..self.resolution.y {
                for col in 0..self.resolution.x {
                    if self.active_frame.matrix[row][col] != following_frame.matrix[row][col] {
                        differences.matrix[row][col] = following_frame.matrix[row][col];
                    }
                }
            }
            return Some(Box::new(differences));
        }
        return None;
    }
}
