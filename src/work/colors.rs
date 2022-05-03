use serde::{Deserialize, Serialize};

pub const BACKGROUND_COLOR: &str = "#000000";

pub const DEFAULT_HUE: f32 = 0.0;
pub const DEFAULT_SATURATION: f32 = 1.0;
pub const DEFAULT_LIGHTNESS: f32 = 0.5;

pub const HUE_RANGE: f32 = 300.0;

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Direction {
    Positive,
    Negative,
}

#[inline]
fn range_percent(start: f32, end: f32, dir: Direction, max: f32, percent: f32) -> f32 {
    debug_assert!(percent <= 1.0 && percent >= 0.0);
    let tmp = if start == end {
        start
    } else if start < end {
        match dir {
            Direction::Positive => start + (end - start) * percent,
            Direction::Negative => {
                let hi_start = start + max;
                (hi_start - (hi_start - end) * percent) % max
            }
        }
    } else {
        match dir {
            Direction::Positive => {
                let hi_end = end + max;
                (start + (hi_end - start) * percent) % max
            }
            Direction::Negative => start - (start - end) * percent,
        }
    };
    debug_assert!(tmp >= 0.0);
    tmp
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct HslRange {
    dir: Direction,
    start: HslColor,
    end: HslColor,
}

impl HslRange {
    #[allow(dead_code)]
    pub fn new(start: HslColor, end: HslColor) -> Self {
        Self {
            dir: Direction::Positive,
            start,
            end,
        }
    }

    #[allow(dead_code)]
    pub fn new_with_dir(start: HslColor, end: HslColor, dir: Direction) -> Self {
        Self { dir, start, end }
    }

    #[inline]
    pub fn percent_of(&self, percent: f32) -> HslColor {
        HslColor {
            hue: range_percent(self.start.hue, self.end.hue, self.dir, 360.0, percent),
            saturation: range_percent(
                self.start.saturation,
                self.end.saturation,
                self.dir,
                1.0,
                percent,
            ),
            lightness: range_percent(
                self.start.lightness,
                self.end.lightness,
                self.dir,
                1.0,
                percent,
            ),
        }
    }
}

impl Default for HslRange {
    fn default() -> Self {
        HslRange {
            start: HslColor::new(DEFAULT_HUE, DEFAULT_SATURATION, DEFAULT_LIGHTNESS),
            end: HslColor::new(
                DEFAULT_HUE + HUE_RANGE,
                DEFAULT_SATURATION,
                DEFAULT_LIGHTNESS,
            ),
            dir: Direction::Positive,
        }
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct RgbRange {
    dir: Direction,
    start: RgbColor,
    end: RgbColor,
}

impl RgbRange {
    #[allow(dead_code)]
    pub fn new(start: RgbColor, end: RgbColor) -> Self {
        Self {
            start,
            end,
            dir: Direction::Positive,
        }
    }

    #[allow(dead_code)]
    pub fn new_with_dir(start: RgbColor, end: RgbColor, dir: Direction) -> Self {
        Self { start, end, dir }
    }

    #[inline]
    pub fn percent_of(&self, percent: f32) -> RgbColor {
        RgbColor {
            red: range_percent(
                self.start.red as f32,
                self.end.red as f32,
                self.dir,
                256.0,
                percent,
            )
            .floor() as u8,
            green: range_percent(
                self.start.green as f32,
                self.end.green as f32,
                self.dir,
                256.0,
                percent,
            )
            .floor() as u8,
            blue: range_percent(
                self.start.blue as f32,
                self.end.blue as f32,
                self.dir,
                256.0,
                percent,
            )
            .floor() as u8,
        }
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum ColorRange {
    Hsl(HslRange),
    Rgb(RgbRange),
}

impl Default for ColorRange {
    fn default() -> Self {
        ColorRange::Hsl(HslRange::default())
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct HslColor {
    hue: f32,
    saturation: f32,
    lightness: f32,
}

impl HslColor {
    pub fn new(hue: f32, saturation: f32, lightness: f32) -> Self {
        Self {
            hue: hue % 360.0,
            saturation: saturation % 100.0,
            lightness: lightness % 100.0,
        }
    }

    #[allow(
        clippy::many_single_char_names,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    pub fn to_rgb(&self) -> RgbColor {
        // see: https://www.rapidtables.com/convert/color/hsl-to-rgb.html

        assert!((0.0..=1.0).contains(&self.saturation));
        assert!((0.0..=1.0).contains(&self.lightness));

        let safe_hue = if self.hue >= 360.0 {
            self.hue % 360.0
        } else {
            self.hue
        };

        let c = (1.0 - f32::abs(2.0 * self.lightness - 1.0)) * self.saturation;
        let x = c * (1.0 - ((safe_hue / 60.0) % 2.0 - 1.0).abs());
        let m = self.lightness - c / 2.0;
        let (r, g, b) = match safe_hue as u32 {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            300..=359 => (c, 0.0, x),
            _ => {
                panic!("invalid hue value: {}", safe_hue);
            }
        };

        let (r, g, b) = (
            f32::floor((r + m) * 255.0).abs() as u32,
            f32::floor((g + m) * 255.0).abs() as u32,
            f32::floor((b + m) * 255.0).abs() as u32,
        );

        RgbColor::new((r & 0xFF) as u8, (g & 0xFF) as u8, (b & 0xFF) as u8)
    }
}
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct RgbColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl RgbColor {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn to_string(&self) -> String {
        format!("#{:0>2X}{:0>2X}{:0>2X}", self.red, self.green, self.blue)
    }
}

#[cfg(test)]
mod test {
    use super::{HslColor, DEFAULT_LIGHTNESS, DEFAULT_SATURATION};
    use crate::work::colors::{Direction, HslRange, DEFAULT_HUE, HUE_RANGE};

    #[test]
    fn test_hsl_to_rgb() {
        let color = HslColor::new(0.0, DEFAULT_SATURATION, DEFAULT_LIGHTNESS);
        assert_eq!(color.to_rgb().to_string().as_str(), "#FF0000");
        let color = HslColor::new(60.0, DEFAULT_SATURATION, DEFAULT_LIGHTNESS);
        assert_eq!(color.to_rgb().to_string().as_str(), "#FFFF00");
        let color = HslColor::new(120.0, DEFAULT_SATURATION, DEFAULT_LIGHTNESS);
        assert_eq!(color.to_rgb().to_string().as_str(), "#00FF00");
        let color = HslColor::new(180.0, DEFAULT_SATURATION, DEFAULT_LIGHTNESS);
        assert_eq!(color.to_rgb().to_string().as_str(), "#00FFFF");
        let color = HslColor::new(240.0, DEFAULT_SATURATION, DEFAULT_LIGHTNESS);
        assert_eq!(color.to_rgb().to_string().as_str(), "#0000FF");
        let color = HslColor::new(300.0, DEFAULT_SATURATION, DEFAULT_LIGHTNESS);
        assert_eq!(color.to_rgb().to_string().as_str(), "#FF00FF");
        let color = HslColor::new(360.0, DEFAULT_SATURATION, DEFAULT_LIGHTNESS);
        assert_eq!(color.to_rgb().to_string().as_str(), "#FF0000");
        let color = HslColor::new(340.0, DEFAULT_SATURATION, DEFAULT_LIGHTNESS);
        assert_eq!(color.to_rgb().to_string().as_str(), "#FF0055");
    }

    #[test]
    fn test_hsl_range() {
        let range = HslRange::default();
        eprintln!("HSL Default Range: {:?}", range);
        assert_eq!(range.start.hue, DEFAULT_HUE);
        assert_eq!(range.start.saturation, DEFAULT_SATURATION);
        assert_eq!(range.start.lightness, DEFAULT_LIGHTNESS);
        assert_eq!(range.end.hue, DEFAULT_HUE + HUE_RANGE);
        assert_eq!(range.end.saturation, DEFAULT_SATURATION);
        assert_eq!(range.end.lightness, DEFAULT_LIGHTNESS);
        assert_eq!(range.dir, Direction::Positive);

        let color = range.percent_of(0.5);
        eprintln!("HSL 50%: {:?}", color);
        assert_eq!(color.hue, DEFAULT_HUE + HUE_RANGE * 0.5);
        assert_eq!(color.saturation, DEFAULT_SATURATION);
        assert_eq!(color.lightness, DEFAULT_LIGHTNESS);
    }
}
