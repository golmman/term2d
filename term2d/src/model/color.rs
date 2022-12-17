use std::fmt::Display;

use super::rgba::Rgba;

#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    pub bg: Rgba,
    pub fg: Rgba,
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
    pub fn text() -> Self {
        Self {
            bg: Rgba::default(),
            fg: Rgba {
                r: 200,
                g: 200,
                b: 200,
                a: 255,
            },
        }
    }
}

impl From<&Color> for String {
    fn from(color: &Color) -> Self {
        let Rgba {
            r: fg_r,
            g: fg_g,
            b: fg_b,
            a: _,
        } = color.fg;
        let Rgba {
            r: bg_r,
            g: bg_g,
            b: bg_b,
            a: _,
        } = color.bg;
        format!("\x1b[38;2;{fg_r};{fg_g};{fg_b};48;2;{bg_r};{bg_g};{bg_b}m")
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
