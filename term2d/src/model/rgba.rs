use std::cmp::max;
use std::cmp::min;

#[derive(Clone, Debug, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for Rgba {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

impl Rgba {
    pub fn transparent() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }

    pub fn black() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    pub fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }

    pub fn red() -> Self {
        Self {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    pub fn green() -> Self {
        Self {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        }
    }

    pub fn blue() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        }
    }

    pub fn yellow() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 0,
            a: 255,
        }
    }

    pub fn cyan() -> Self {
        Self {
            r: 0,
            g: 255,
            b: 255,
            a: 255,
        }
    }

    pub fn violet() -> Self {
        Self {
            r: 255,
            g: 0,
            b: 255,
            a: 255,
        }
    }

    pub fn bg_ansi(&self) -> String {
        let Rgba { r, g, b, a: _ } = *self;
        format!("\x1b[48;2;{r};{g};{b}m")
    }

    pub fn fg_ansi(&self) -> String {
        let Rgba { r, g, b, a: _ } = *self;
        format!("\x1b[38;2;{r};{g};{b}m")
    }

    pub fn blend(&self, other: &Rgba) -> Rgba {
        if self.a == 0 {
            return other.clone();
        }

        if self.a == 255 {
            return self.clone();
        }

        let a = self.a as i32;
        let r = ((a * self.r as i32 + (255 - a) * other.r as i32) / 255) as u8;
        let g = ((a * self.g as i32 + (255 - a) * other.g as i32) / 255) as u8;
        let b = ((a * self.b as i32 + (255 - a) * other.b as i32) / 255) as u8;
        Rgba { r, g, b, a: 255 }
    }

    pub fn fade(&mut self, target: &Rgba, fading_speed: i32) {
        let f_r = Rgba::calc_real_fading_speed(fading_speed, target.r as i32 - self.r as i32);
        let f_g = Rgba::calc_real_fading_speed(fading_speed, target.g as i32 - self.g as i32);
        let f_b = Rgba::calc_real_fading_speed(fading_speed, target.b as i32 - self.b as i32);
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
