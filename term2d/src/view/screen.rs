use std::io::stdout;
use std::io::Stdout;
use std::io::Write;

use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

use crate::model::ansiesc::CLEAR_ALL;
use crate::model::ansiesc::COLOR_RESET;
use crate::model::ansiesc::CURSOR_GOTO_1_1;
use crate::model::ansiesc::CURSOR_SHOW;
use crate::model::color::Color;
use crate::model::point::Point;
use crate::model::rgba::Rgba;

pub type DefaultScreen = Screen<RawTerminal<Stdout>>;

#[derive(Debug, Clone)]
struct Pixel {
    pub ch: char,
    pub color: Color,
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            color: Default::default(),
            ch: ' ',
        }
    }
}

impl From<char> for Pixel {
    fn from(ch: char) -> Self {
        Self {
            color: Color::text(),
            ch,
        }
    }
}

pub struct Screen<W: Write> {
    drop_strings: Vec<String>,
    main_display: W,
    pixel_buffer: Vec<Pixel>,
    prelude_buffer: String,

    pub size: Point,
}

impl DefaultScreen {
    pub fn new(drop_strings: Vec<String>) -> Self {
        let mut screen = Screen::from(stdout().into_raw_mode().unwrap());
        screen.drop_strings = drop_strings;
        screen
    }

    pub fn resize(&mut self) -> Point {
        let (cols, rows) = termion::terminal_size().unwrap();

        self.size = Point::new(cols as i32, rows as i32);

        self.size.clone()
    }

    pub fn clear(&mut self) {
        let buffer_size = (self.size.width() * self.size.height()) as usize;
        self.prelude_buffer = String::new();
        self.pixel_buffer = vec![Pixel::default(); buffer_size];
    }

    pub fn draw_pixel(&mut self, p: &Point, rgb: &Rgba) {
        let index = (self.size.width() * p.y + p.x) as usize;

        let old_rgba = &self.pixel_buffer[index].color.bg;
        let new_rgba = rgb.blend(old_rgba);

        let new_color = Color {
            fg: Rgba::black(),
            bg: new_rgba,
        };

        self.pixel_buffer[index] = Pixel {
            ch: ' ',
            color: new_color,
        };
    }

    pub fn draw_char(&mut self, p: &Point, color: &Color, ch: char) {
        let index = (self.size.width() * p.y + p.x) as usize;

        let old_bg = &self.pixel_buffer[index].color.bg;
        let new_bg = color.bg.blend(old_bg);
        let old_fg = &self.pixel_buffer[index].color.fg;
        let new_fg = color.fg.blend(old_fg);

        let new_color = Color {
            bg: new_bg,
            fg: new_fg,
        };

        self.pixel_buffer[index] = Pixel {
            ch,
            color: new_color,
        };
    }

    pub fn draw_text(&mut self, p: &Point, color: &Color, text: &str) {
        for (i, ch) in text.chars().enumerate() {
            self.draw_char(&Point::new(p.x + i as i32, p.y), color, ch);
        }
    }

    pub fn get_color(&self, p: &Point) -> Color {
        let index = (self.size.width() * p.y + p.x) as usize;
        self.pixel_buffer[index].color.clone()
    }

    pub fn display(&mut self) {
        let mut s = String::new();

        s.push_str(&self.prelude_buffer);

        for y in 0..self.size.height() {
            let row = y + 1;
            s.push_str(&format!("\x1b[{row};1H")); // goto (row, 1)

            let mut x = 0;

            while x < self.size.width() {
                let mut i = (self.size.width() * y + x) as usize;
                let mut last_color = &self.pixel_buffer[i].color;
                s.push_str(&format!("{last_color}"));

                while x < self.size.width() && *last_color == self.pixel_buffer[i].color {
                    last_color = &self.pixel_buffer[i].color;
                    s.push(self.pixel_buffer[i].ch);

                    x += 1;
                    i = (self.size.width() * y + x) as usize;
                }
            }
        }

        self.main_display.write_all(s.as_bytes()).unwrap();
        self.main_display.flush().unwrap();
    }
}

impl<W: Write> From<W> for Screen<W> {
    fn from(mut buffer: W) -> Self {
        write!(
            buffer,
            "{}{}{}",
            termion::cursor::Hide,
            termion::cursor::Goto(1, 1),
            termion::clear::All,
        )
        .unwrap();

        buffer.flush().unwrap();

        let (cols, rows) = termion::terminal_size().unwrap();
        let buffer_size = (cols * rows) as usize;

        let prelude_buffer = String::new();
        let pixel_buffer = vec![Pixel::from(' '); buffer_size];

        Self {
            drop_strings: vec![
                COLOR_RESET.to_string(),
                CLEAR_ALL.to_string(),
                CURSOR_GOTO_1_1.to_string(),
                CURSOR_SHOW.to_string(),
            ],
            main_display: buffer,
            pixel_buffer,
            prelude_buffer,
            size: Point::new(cols as i32, rows as i32),
        }
    }
}

impl<W: Write> Drop for Screen<W> {
    fn drop(&mut self) {
        write!(self.main_display, "{}", self.drop_strings.join("")).unwrap();

        self.main_display.flush().unwrap();
    }
}
