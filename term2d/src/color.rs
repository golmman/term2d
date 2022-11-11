// usage of termions 'Color' module is awkward
// see https://gitlab.redox-os.org/redox-os/termion/-/issues/123

// see
// https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
// https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797

use std::{
    cmp::{max, min},
    fmt::Display,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for Rgb {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}

impl Rgb {
    pub fn black() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    pub fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    pub fn red() -> Self {
        Self { r: 255, g: 0, b: 0 }
    }

    pub fn green() -> Self {
        Self { r: 0, g: 255, b: 0 }
    }

    pub fn blue() -> Self {
        Self { r: 0, g: 0, b: 255 }
    }

    pub fn yellow() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 0,
        }
    }

    pub fn cyan() -> Self {
        Self {
            r: 0,
            g: 255,
            b: 255,
        }
    }

    pub fn violet() -> Self {
        Self {
            r: 255,
            g: 0,
            b: 255,
        }
    }

    pub fn blend(&self, other: &Rgb, alpha: u8) -> Rgb {
        let a = alpha as i32;
        let r = ((a * self.r as i32 + (255 - a) * other.r as i32) / 255) as u8;
        let g = ((a * self.g as i32 + (255 - a) * other.g as i32) / 255) as u8;
        let b = ((a * self.b as i32 + (255 - a) * other.b as i32) / 255) as u8;
        Rgb { r, g, b }
    }

    pub fn fade(&mut self, target: &Rgb, fading_speed: i32) {
        let f_r = Rgb::calc_real_fading_speed(fading_speed, target.r as i32 - self.r as i32);
        let f_g = Rgb::calc_real_fading_speed(fading_speed, target.g as i32 - self.g as i32);
        let f_b = Rgb::calc_real_fading_speed(fading_speed, target.b as i32 - self.b as i32);
        self.r = (self.r as i32 + f_r) as u8;
        self.g = (self.g as i32 + f_g) as u8;
        self.b = (self.b as i32 + f_b) as u8;
    }

    fn calc_real_fading_speed(fading_speed: i32, color_delta: i32) -> i32 {
        if color_delta < 0 {
            return max(-fading_speed, color_delta);
        } else if color_delta > 0 {
            return min(fading_speed, color_delta);
        }

        0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub bg: Rgb,
    pub fg: Rgb,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            bg: Default::default(),
            fg: Default::default(),
        }
    }
}

impl Color {
    pub const RESET: &'static str = "\x1b[0m";

    pub fn text() -> Self {
        Self {
            bg: Rgb::default(),
            fg: Rgb {
                r: 200,
                g: 200,
                b: 200,
            },
        }
    }
}

impl From<&Color> for String {
    fn from(color: &Color) -> Self {
        let Rgb {
            r: fg_r,
            g: fg_g,
            b: fg_b,
        } = color.fg;
        let Rgb {
            r: bg_r,
            g: bg_g,
            b: bg_b,
        } = color.bg;
        format!("\x1b[38;2;{fg_r};{fg_g};{fg_b};48;2;{bg_r};{bg_g};{bg_b}m")
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}