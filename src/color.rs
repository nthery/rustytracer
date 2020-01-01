//! Color type
//!
//! TRTC chapter 2

use crate::util;

#[derive(Debug, Clone)]
pub struct Color {
    rgb: [f64; 3],
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { rgb: [r, g, b] }
    }

    pub fn red(&self) -> f64 {
        self.rgb[0]
    }

    pub fn green(&self) -> f64 {
        self.rgb[1]
    }

    pub fn blue(&self) -> f64 {
        self.rgb[2]
    }

    pub fn channel(&self, i: usize) -> f64 {
        self.rgb[i]
    }
}

impl PartialEq for Color {
    fn eq(&self, o: &Self) -> bool {
        util::nearly_equal(self.rgb[0], o.rgb[0])
            && util::nearly_equal(self.rgb[1], o.rgb[1])
            && util::nearly_equal(self.rgb[2], o.rgb[2])
    }
}

impl std::ops::Add for &Color {
    type Output = Color;
    fn add(self, o: Self) -> Self::Output {
        Color::new(
            self.rgb[0] + o.rgb[0],
            self.rgb[1] + o.rgb[1],
            self.rgb[2] + o.rgb[2],
        )
    }
}

impl std::ops::Sub for &Color {
    type Output = Color;
    fn sub(self, o: Self) -> Self::Output {
        Color::new(
            self.rgb[0] - o.rgb[0],
            self.rgb[1] - o.rgb[1],
            self.rgb[2] - o.rgb[2],
        )
    }
}

impl std::ops::Mul<f64> for &Color {
    type Output = Color;
    fn mul(self, o: f64) -> Self::Output {
        Color::new(self.rgb[0] * o, self.rgb[1] * o, self.rgb[2] * o)
    }
}

impl std::ops::Mul for &Color {
    type Output = Color;
    fn mul(self, o: Self) -> Self::Output {
        Color::new(
            self.rgb[0] * o.rgb[0],
            self.rgb[1] * o.rgb[1],
            self.rgb[2] * o.rgb[2],
        )
    }
}

pub const BLACK: Color = Color {
    rgb: [0.0, 0.0, 0.0],
};
pub const WHITE: Color = Color {
    rgb: [1.0, 1.0, 1.0],
};
pub const RED: Color = Color {
    rgb: [1.0, 0.0, 0.0],
};
pub const GREEN: Color = Color {
    rgb: [0.0, 1.0, 0.0],
};
pub const BLUE: Color = Color {
    rgb: [0.0, 0.0, 1.0],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_rgb_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.red(), -0.5);
        assert_eq!(c.green(), 0.4);
        assert_eq!(c.blue(), 1.7);
    }

    #[test]
    fn adding_colors() {
        assert_eq!(
            &Color::new(0.9, 0.6, 0.75) + &Color::new(0.7, 0.1, 0.25),
            Color::new(1.6, 0.7, 1.0)
        );
    }

    #[test]
    fn subtracting_colors() {
        assert_eq!(
            &Color::new(0.9, 0.6, 0.75) - &Color::new(0.7, 0.1, 0.25),
            Color::new(0.20000000000000007, 0.5, 0.5)
        );
    }

    #[test]
    fn multiplying_color_by_scalar() {
        assert_eq!(&Color::new(0.2, 0.3, 0.4) * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colors() {
        assert_eq!(
            &Color::new(1.0, 0.2, 0.4) * &Color::new(0.9, 1.0, 0.1),
            Color::new(0.9, 0.2, 0.04000000000000001)
        );
    }
}
