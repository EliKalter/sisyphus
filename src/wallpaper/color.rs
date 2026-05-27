use std::io::LineWriter;

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn black() -> Color {
        Color::new(0, 0, 0)
    }

    fn reverse(&mut self) {
        let reversed: Color = self.get_reverse();
        self.r = reversed.r;
        self.g = reversed.g;
        self.b = reversed.b;
    }

    fn get_reverse(&self) -> Color {
        Color {
            r: u8::MAX - self.r,
            g: u8::MAX - self.g,
            b: u8::MAX - self.b,
        }
    }

    // The brightness adgusted for human perception
    // Normalized
    pub fn relative_luminanace(&self) -> f32 {
        let r: f32 = self.r as f32;
        let g: f32 = self.g as f32;
        let b: f32 = self.b as f32;

        (0.241 * r.powi(2) + 0.691 * g.powi(2) + 0.068 * b.powi(2)).sqrt() / 255.0
    }
}
