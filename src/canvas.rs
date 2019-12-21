//! Canvas type
//!
//! TRTC chapter 2

use crate::color::{self, Color};

/// A 2D grid of pixels.
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    /// Create a black canvas of the specified size.
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![color::BLACK; width * height],
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

    pub fn set(&mut self, x: usize, y: usize, color: &Color) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.pixels[y * self.width + x] = color.clone();
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
        assert_eq!(*c.get(0, 0), color::BLACK);
        assert_eq!(*c.get(9, 19), color::BLACK);
    }

    #[test]
    fn writing_pixel() {
        let mut c = Canvas::new(10, 20);
        c.set(1, 2, &color::RED);
        assert_eq!(*c.get(1, 2), color::RED);
    }
}
