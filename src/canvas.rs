//! Canvas type
//!
//! TRTC chapter 2

use crate::color::{self, Color};

const PPM_MAX_COLOR_VALUE: i32 = 255;
const PPM_MAX_CHAR_PER_LINE: usize = 70;

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

    /// Set all pixels to specified color.
    pub fn fill(&mut self, color: &Color) {
        for c in &mut self.pixels {
            *c = color.clone();
        }
    }

    /// Convert this canvas to PPM format.
    /// TODO: return sequence of u8 instead?
    pub fn to_ppm(&self) -> String {
        format!("{}{}", self.ppm_header(), self.ppm_data())
    }

    fn ppm_header(&self) -> String {
        format!(
            "P3\n{} {}\n{}\n",
            self.width, self.height, PPM_MAX_COLOR_VALUE
        )
    }

    fn ppm_data(&self) -> String {
        let mut ppm = String::new();

        // Number of characters on current line.
        let mut nb_chars = 0;

        // Number of Color instances on current line.
        let mut nb_colors = 0;

        for color in &self.pixels {
            if nb_colors == self.width && nb_chars > 0 {
                // We've emitted a full row, go to the next line.
                nb_colors = 0;
                nb_chars = 0;
                ppm.push('\n');
            }
            nb_colors += 1;

            for i in 0..3 {
                let channel = color.channel(i);
                let s = format!("{}", scale_and_clamp_color(channel));
                if nb_chars + s.len() >= PPM_MAX_CHAR_PER_LINE {
                    // Adding the current channel would overflow, go to next line.
                    nb_chars = 0;
                    ppm.push('\n');
                } else if nb_chars > 0 {
                    // Not first channel on line.
                    nb_chars += 1;
                    ppm.push(' ');
                }
                nb_chars += s.len();
                ppm += s.as_str();
            }
        }

        if nb_chars > 0 {
            ppm.push('\n');
        }

        ppm
    }
}

/// Convert a `Color` component to a PPM color value.
fn scale_and_clamp_color(n: f64) -> i32 {
    let scaled = (n * PPM_MAX_COLOR_VALUE as f64).round() as i32;
    if scaled < 0 {
        0
    } else if scaled > PPM_MAX_COLOR_VALUE {
        PPM_MAX_COLOR_VALUE
    } else {
        scaled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_canvas() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width(), 10);
        assert_eq!(canvas.height(), 20);

        // All pixels are black
        assert_eq!(canvas.get(0, 0), &color::BLACK);
        assert_eq!(canvas.get(9, 19), &color::BLACK);
    }

    #[test]
    fn writing_pixel() {
        let mut canvas = Canvas::new(10, 20);
        canvas.set(1, 2, &color::RED);
        assert_eq!(*canvas.get(1, 2), color::RED);
    }

    #[test]
    fn constructing_ppm_header() {
        let canvas = Canvas::new(5, 3);
        let want = "P3\n5 3\n255\n";
        assert_eq!(canvas.ppm_header(), want);
    }

    #[test]
    fn constructing_ppm_data() {
        let mut canvas = Canvas::new(5, 3);
        canvas.set(0, 0, &Color::new(1.5, 0.0, 0.0));
        canvas.set(2, 1, &Color::new(0.0, 0.5, 0.0));
        canvas.set(4, 2, &Color::new(-0.5, 0.0, 1.0));
        let want = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
                    0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
                    0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";
        assert_eq!(canvas.ppm_data(), want);
    }

    #[test]
    fn splitting_long_ppm_lines() {
        let mut canvas = Canvas::new(10, 2);
        canvas.fill(&Color::new(1.0, 0.8, 0.6));
        let want = "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
                    153 255 204 153 255 204 153 255 204 153 255 204 153\n\
                    255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
                    153 255 204 153 255 204 153 255 204 153 255 204 153\n";
        assert_eq!(canvas.ppm_data(), want);
    }
}
