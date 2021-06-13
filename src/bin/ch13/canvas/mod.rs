use std::{
    fs::File,
    io::{self, BufWriter},
};

use io::Write;

use super::tuple::Color;
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::default(); width * height],
        }
    }

    pub fn write(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[(self.width * y) + x] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[(self.width * y) + x]
    }

    pub fn to_ppm(&self, filename: &str) -> io::Result<()> {
        let f = File::create(filename)?;
        let mut w = BufWriter::new(f);
        let header = format!("P3\n{} {}\n255", self.width, self.height);
        w.write_all(header.as_bytes())?;
        for (i, pixel) in self.pixels.iter().enumerate() {
            if i % self.width == 0 {
                w.write_all(b"\n")?;
            }
            w.write_all((*pixel * 255f64).to_string().as_bytes())?;
            w.write_all(b" ")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_creation() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert!(c.pixels.iter().all(|&c| c == Color::default()));
    }

    #[test]
    fn write_pixel() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        let x = 2;
        let y = 3;
        c.write(x, y, red);
        assert_eq!(c.pixel_at(x, y), red);
    }
}
