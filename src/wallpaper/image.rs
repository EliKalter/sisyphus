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

    fn pix_at(&self, x: u32, y: u32) -> Result<Color, String> {
        // Make sure the the row and col are in the bounds
        // // TODO: Create 2D struct that will represent a point and use it to take the row and col
        if x >= self.height || y >= self.width {
            return Err("X and Y must be the index of a row and column in the image".to_string());
        };

        // Return the pix at that position
        Ok(self.pixels[(x * self.width + y) as usize])
    }

    // TODO: Have function that takes in a point of the same type as pix coordinates are, and checks
    // it is in the boounds of the image
}

// TODO: Implement the access to pixels by (x, y) coordinates
