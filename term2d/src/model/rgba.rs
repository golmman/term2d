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
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Constructs an Rgba value from a hex str.
    pub const fn from_hex(hex: &str) -> Self {
        let bytes = hex.as_bytes();
        let r = 16 * char_to_hex(bytes[0]) + char_to_hex(bytes[1]);
        let g = 16 * char_to_hex(bytes[2]) + char_to_hex(bytes[3]);
        let b = 16 * char_to_hex(bytes[4]) + char_to_hex(bytes[5]);
        Self { r, g, b, a: 255 }
    }

    pub const fn transparent() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }

    pub const fn black() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    pub const fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }

    pub const fn red() -> Self {
        Self {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    pub const fn green() -> Self {
        Self {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        }
    }

    pub const fn blue() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        }
    }

    pub const fn yellow() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 0,
            a: 255,
        }
    }

    pub const fn cyan() -> Self {
        Self {
            r: 0,
            g: 255,
            b: 255,
            a: 255,
        }
    }

    pub const fn violet() -> Self {
        Self {
            r: 255,
            g: 0,
            b: 255,
            a: 255,
        }
    }

    pub const fn orange() -> Self {
        Self {
            r: 255,
            g: 128,
            b: 0,
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

const fn char_to_hex(ascii_char: u8) -> u8 {
    match ascii_char {
        48 => 0,
        49 => 1,
        50 => 2,
        51 => 3,
        52 => 4,
        53 => 5,
        54 => 6,
        55 => 7,
        56 => 8,
        57 => 9,
        65 | 97 => 10,
        66 | 98 => 11,
        67 | 99 => 12,
        68 | 100 => 13,
        69 | 101 => 14,
        70 | 102 => 15,
        _ => panic!("Not a valid hex string."),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_constructs_rgba_from_hex_string() {
        let color = Rgba::from_hex("012345");
        assert_eq!(
            color,
            Rgba {
                r: 1,
                g: 35,
                b: 69,
                a: 255
            }
        );

        let color = Rgba::from_hex("678901");
        assert_eq!(
            color,
            Rgba {
                r: 103,
                g: 137,
                b: 01,
                a: 255
            }
        );

        let color = Rgba::from_hex("abcdef");
        assert_eq!(
            color,
            Rgba {
                r: 171,
                g: 205,
                b: 239,
                a: 255
            }
        );

        let color = Rgba::from_hex("ABCDEF");
        assert_eq!(
            color,
            Rgba {
                r: 171,
                g: 205,
                b: 239,
                a: 255
            }
        );
    }
}
