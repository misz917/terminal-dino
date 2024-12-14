const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);
const UP: (i32, i32) = (0, -1);
const DOWN: (i32, i32) = (0, 1);

pub fn upscale(rgba_values: Vec<Vec<bool>>, strength: u8) -> Vec<Vec<bool>> {
    let mut current_img = rgba_values;
    for _ in 0..strength {
        current_img = upscale_boilerplate(&current_img);
    }
    current_img
}

fn upscale_boilerplate(img: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let height = img.len();
    let width = if height > 0 { img[0].len() } else { 0 };

    let mut modified_img = vec![vec![false; width * 2]; height * 2];

    for y in 0..height {
        for x in 0..width {
            modified_img[y * 2][x * 2] = get_new_pixel(&[LEFT, UP], x, y, img);
            modified_img[y * 2][x * 2 + 1] = get_new_pixel(&[UP, RIGHT], x, y, img);
            modified_img[y * 2 + 1][x * 2 + 1] = get_new_pixel(&[RIGHT, DOWN], x, y, img);
            modified_img[y * 2 + 1][x * 2] = get_new_pixel(&[DOWN, LEFT], x, y, img);
        }
    }

    modified_img
}

fn get_new_pixel(directions: &[(i32, i32); 2], x: usize, y: usize, img: &Vec<Vec<bool>>) -> bool {
    let (xi, yi) = directions[0];
    let (xii, yii) = directions[1];
    let first_pixel: bool;

    match safe_pixel_grab((x as i32 + xi) as usize, (y as i32 + yi) as usize, img) {
        Some(pixel) => first_pixel = pixel,
        None => return img[y][x],
    }

    match safe_pixel_grab((x as i32 + xii) as usize, (y as i32 + yii) as usize, img) {
        Some(pixel) => {
            if first_pixel == pixel {
                return pixel;
            } else {
                return img[y][x];
            }
        }
        None => return img[y][x],
    }
}

fn safe_pixel_grab(x: usize, y: usize, img: &Vec<Vec<bool>>) -> Option<bool> {
    if y < img.len() && x < img[0].len() {
        Some(img[y][x])
    } else {
        None
    }
}
