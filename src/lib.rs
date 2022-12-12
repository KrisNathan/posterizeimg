fn posterize(n: u8, level: u8, max: u8) -> u8 {
    let m = max / level;
    let mut minimum_delta = max;
    let mut target = 0;
    for i in 0..level {
        let ceil = (i + 1) * m;
        let floor = i * m;
        let dc = n.abs_diff(ceil);
        let df = n.abs_diff(floor);
        if minimum_delta > dc {
            minimum_delta = dc;
            target = ceil;
        }
        if minimum_delta > df {
            minimum_delta = df;
            target = floor;
        }
    }
    target
}

#[derive(Debug)]
pub enum PosterizeErrorType {
    MinimumLevel,
}

#[derive(Debug)]
pub struct PosterizeError(PosterizeErrorType);

impl std::fmt::Display for PosterizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            PosterizeErrorType::MinimumLevel => {
                write!(f, "expected level higher than or equal to 2")
            }
        }
    }
}

impl std::error::Error for PosterizeError {
    fn description(&self) -> &str {
        match self.0 {
            PosterizeErrorType::MinimumLevel => "expected level higher than or equal to 2",
        }
    }
}

use image::{ImageBuffer, Rgba};

pub fn posterize_img_rgba8(
    img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    level: u8,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, PosterizeError> {
    if level < 2 {
        return Err(PosterizeError(PosterizeErrorType::MinimumLevel));
    }
    let mut copy = img.clone();
    for pix in copy.pixels_mut() {
        for i in 0..4 {
            pix.0[i] = posterize(pix.0[i], level - 1, 255);
        }
    }
    Ok(copy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};

    #[test]
    fn test_posterize() {
        assert_eq!(posterize(128, 1, u8::MAX), u8::MAX)
    }

    #[test]
    fn test_posterize_img_rgba8() -> Result<(), Box<dyn std::error::Error>> {
        let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(1, 1);
        img.get_pixel_mut(0, 0).0[0] = 128;
        let mut expectation: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(1, 1);
        expectation.get_pixel_mut(0, 0).0[0] = 255;
        let result = posterize_img_rgba8(&img, 2)?;
        assert_eq!(result, expectation);
        Ok(())
    }
}
