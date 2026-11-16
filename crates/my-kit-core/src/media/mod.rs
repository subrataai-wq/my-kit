//! Media, image dimension calculation, and aspect ratio toolkit.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

impl Dimensions {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn aspect_ratio(&self) -> f64 {
        if self.height == 0 {
            0.0
        } else {
            self.width as f64 / self.height as f64
        }
    }

    pub fn is_portrait(&self) -> bool {
        self.height > self.width
    }

    pub fn is_landscape(&self) -> bool {
        self.width > self.height
    }

    pub fn is_square(&self) -> bool {
        self.width == self.height && self.width > 0
    }

    pub fn scale_by(&self, factor: f64) -> Self {
        let w = (self.width as f64 * factor).round().max(1.0) as u32;
        let h = (self.height as f64 * factor).round().max(1.0) as u32;
        Self::new(w, h)
    }
}

/// Standardized aspect ratios for digital media, galleries, and products.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectRatio {
    Ratio16x9,
    Ratio4x3,
    Ratio1x1,
    Ratio9x16,
    Custom(u32, u32),
}

impl AspectRatio {
    pub fn as_f64(&self) -> f64 {
        match self {
            Self::Ratio16x9 => 16.0 / 9.0,
            Self::Ratio4x3 => 4.0 / 3.0,
            Self::Ratio1x1 => 1.0,
            Self::Ratio9x16 => 9.0 / 16.0,
            Self::Custom(w, h) => {
                if *h == 0 {
                    0.0
                } else {
                    *w as f64 / *h as f64
                }
            }
        }
    }

    pub fn to_dimensions_width(&self, width: u32) -> Dimensions {
        let ratio = self.as_f64();
        let height = if ratio == 0.0 {
            0
        } else {
            (width as f64 / ratio).round().max(1.0) as u32
        };
        Dimensions::new(width, height)
    }

    pub fn to_dimensions_height(&self, height: u32) -> Dimensions {
        let ratio = self.as_f64();
        let width = (height as f64 * ratio).round().max(1.0) as u32;
        Dimensions::new(width, height)
    }
}

/// Fits source dimensions within a bounding box, preserving aspect ratio (letterbox / contain).
pub fn fit_contain(src: Dimensions, max_box: Dimensions) -> Dimensions {
    if src.width == 0 || src.height == 0 || max_box.width == 0 || max_box.height == 0 {
        return Dimensions::new(0, 0);
    }

    let w_ratio = max_box.width as f64 / src.width as f64;
    let h_ratio = max_box.height as f64 / src.height as f64;
    let scale = w_ratio.min(h_ratio);

    src.scale_by(scale)
}

/// Scales source dimensions so that it completely covers the target box, preserving aspect ratio (cover / fill).
pub fn fit_cover(src: Dimensions, target_box: Dimensions) -> Dimensions {
    if src.width == 0 || src.height == 0 || target_box.width == 0 || target_box.height == 0 {
        return Dimensions::new(0, 0);
    }

    let w_ratio = target_box.width as f64 / src.width as f64;
    let h_ratio = target_box.height as f64 / src.height as f64;
    let scale = w_ratio.max(h_ratio);

    src.scale_by(scale)
}

/// Standard EXIF orientation values for cameras and photo studios.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Normal = 1,
    FlipHorizontal = 2,
    Rotate180 = 3,
    FlipVertical = 4,
    Transpose = 5,
    Rotate90 = 6,
    Transverse = 7,
    Rotate270 = 8,
}

impl Orientation {
    pub fn from_exif(value: u16) -> Self {
        match value {
            2 => Self::FlipHorizontal,
            3 => Self::Rotate180,
            4 => Self::FlipVertical,
            5 => Self::Transpose,
            6 => Self::Rotate90,
            7 => Self::Transverse,
            8 => Self::Rotate270,
            _ => Self::Normal,
        }
    }

    pub fn rotation_degrees(&self) -> u32 {
        match self {
            Self::Normal | Self::FlipHorizontal => 0,
            Self::Rotate90 | Self::Transpose => 90,
            Self::Rotate180 | Self::FlipVertical => 180,
            Self::Rotate270 | Self::Transverse => 270,
        }
    }

    pub fn is_mirrored(&self) -> bool {
        matches!(
            self,
            Self::FlipHorizontal | Self::FlipVertical | Self::Transpose | Self::Transverse
        )
    }
}

pub mod prelude {
    pub use super::{fit_contain, fit_cover, AspectRatio, Dimensions, Orientation};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimensions_calculations() {
        let landscape = Dimensions::new(1920, 1080);
        assert!(landscape.is_landscape());
        assert!(!landscape.is_portrait());
        assert!((landscape.aspect_ratio() - 1.77777777).abs() < 0.001);

        let square = Dimensions::new(500, 500);
        assert!(square.is_square());

        let scaled = square.scale_by(0.5);
        assert_eq!(scaled, Dimensions::new(250, 250));
    }

    #[test]
    fn test_aspect_ratio_to_dimensions() {
        let ar = AspectRatio::Ratio16x9;
        let dim = ar.to_dimensions_width(1920);
        assert_eq!(dim, Dimensions::new(1920, 1080));

        let dim_h = ar.to_dimensions_height(1080);
        assert_eq!(dim_h, Dimensions::new(1920, 1080));
    }

    #[test]
    fn test_fit_contain() {
        let src = Dimensions::new(2000, 1000); // 2:1
        let max_box = Dimensions::new(1000, 1000);
        let fitted = fit_contain(src, max_box);
        assert_eq!(fitted, Dimensions::new(1000, 500));
    }

    #[test]
    fn test_fit_cover() {
        let src = Dimensions::new(2000, 1000); // 2:1
        let target_box = Dimensions::new(1000, 1000);
        let fitted = fit_cover(src, target_box);
        assert_eq!(fitted, Dimensions::new(2000, 1000));
    }

    #[test]
    fn test_orientation() {
        assert_eq!(Orientation::from_exif(1), Orientation::Normal);
        assert_eq!(Orientation::from_exif(6).rotation_degrees(), 90);
        assert!(Orientation::from_exif(2).is_mirrored());
    }
}

