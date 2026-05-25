use crate::wallpaper::color::Color;

struct Image {
    pixels: Vec<Color>,
    width: u32,
    height: u32,
}

impl Image {
    fn new(width: u32, height: u32) -> Image {
        let size = (width * height) as usize;
        Image {
            pixels: vec![Color::black(); size],
            width,
            height,
        }
    }
}

// TODO: Implement the access to pixels by (x, y) coordinates
