//! Canvas type
//!
//! TRTC chapter 2

use crate::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> &Color {
        assert!(x < self.width);
        assert!(y < self.height);
        &self.pixels[y * self.width + x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width(), 10);
        assert_eq!(c.height(), 20);

        // All pixels are black
        assert_eq!(*c.get(0, 0), Color::new(0.0, 0.0, 0.0));
        assert_eq!(*c.get(9, 19), Color::new(0.0, 0.0, 0.0));
    }
}
