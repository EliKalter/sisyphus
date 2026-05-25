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
}
