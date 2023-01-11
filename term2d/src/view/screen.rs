use std::io::stdout;
use std::io::Stdout;
use std::io::Write;

use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

use crate::model::ansiesc::cursor_goto;
use crate::model::color::Color;
use crate::model::point::Point;
use crate::model::rgba::Rgba;

#[derive(Debug, Clone)]
pub struct Pixel {
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

pub struct RawTerminalScreen {
    drop_strings: Vec<String>,
    main_display: RawTerminal<Stdout>,
    pixel_buffer: Vec<Pixel>,
    size: Point,
}

pub trait Screen {
    fn get_pixel(&self, index: usize) -> &Pixel;
    fn set_pixel(&mut self, index: usize, pixel: &Pixel);
    fn flush_pixels(&mut self, raw_pixels: &[u8]);
    fn get_size(&self) -> &Point;
    fn resize(&mut self) -> &Point;
    fn clear(&mut self);

    fn draw_pixel(&mut self, p: &Point, rgba: &Rgba) {
        let index = (self.get_size().width() * p.y + p.x) as usize;

        let old_rgba = &self.get_pixel(index).color.bg;
        let new_rgba = rgba.blend(old_rgba);

        let new_color = Color {
            fg: Rgba::black(),
            bg: new_rgba,
        };

        self.set_pixel(
            index,
            &Pixel {
                ch: ' ',
                color: new_color,
            },
        );
    }

    fn draw_char(&mut self, p: &Point, color: &Color, ch: char) {
        let index = (self.get_size().width() * p.y + p.x) as usize;
        let pixel = &self.get_pixel(index);

        let old_bg = &pixel.color.bg;
        let new_bg = color.bg.blend(old_bg);
        let old_fg = &pixel.color.fg;
        let new_fg = color.fg.blend(old_fg);

        let new_color = Color {
            bg: new_bg,
            fg: new_fg,
        };

        self.set_pixel(
            index,
            &Pixel {
                ch,
                color: new_color,
            },
        );
    }

    fn draw_text(&mut self, p: &Point, color: &Color, text: &str) {
        for (i, ch) in text.chars().enumerate() {
            self.draw_char(&Point::new(p.x + i as i32, p.y), color, ch);
        }
    }

    fn get_color(&self, p: &Point) -> Color {
        let index = (self.get_size().width() * p.y + p.x) as usize;
        self.get_pixel(index).color.clone()
    }

    fn display(&mut self) {
        let mut s = String::new();
        let Point {
            x: width,
            y: height,
        } = *self.get_size();

        for y in 0..height {
            let row = y + 1;
            s.push_str(&cursor_goto(1, row));

            let mut i = (width * y) as usize;
            let i_max = (width * (y + 1)) as usize;
            let mut last_color = &self.get_pixel(i).color;
            s.push_str(&format!("{last_color}"));

            while i < i_max {
                let Pixel { color, ch } = &self.get_pixel(i);

                if color.bg != last_color.bg && color.fg != last_color.fg {
                    s.push_str(&format!("{color}"));
                } else if color.bg != last_color.bg && color.fg == last_color.fg {
                    s.push_str(&color.bg.bg_ansi());
                } else if color.bg == last_color.bg && color.fg != last_color.fg {
                    s.push_str(&color.fg.fg_ansi());
                }

                last_color = color;
                s.push(*ch);
                i += 1;
            }
        }

        self.flush_pixels(s.as_bytes());
    }
}

impl Screen for RawTerminalScreen {
    fn get_pixel(&self, index: usize) -> &Pixel {
        &self.pixel_buffer[index]
    }

    fn set_pixel(&mut self, index: usize, pixel: &Pixel) {
        self.pixel_buffer[index] = pixel.clone();
    }

    fn flush_pixels(&mut self, raw_pixels: &[u8]) {
        self.main_display.write_all(raw_pixels).unwrap();
        self.main_display.flush().unwrap();
    }

    fn get_size(&self) -> &Point {
        &self.size
    }

    fn resize(&mut self) -> &Point {
        let (cols, rows) = termion::terminal_size().unwrap();
        self.size = Point::new(cols as i32, rows as i32);
        &self.size
    }

    fn clear(&mut self) {
        let buffer_size = (self.size.width() * self.size.height()) as usize;
        self.pixel_buffer = vec![Pixel::default(); buffer_size];
    }
}

impl Drop for RawTerminalScreen {
    fn drop(&mut self) {
        write!(self.main_display, "{}", self.drop_strings.join("")).unwrap();
        self.main_display.flush().unwrap();
    }
}

impl RawTerminalScreen {
    pub fn new(drop_strings: Vec<String>) -> Self {
        let mut main_display = stdout().into_raw_mode().unwrap();

        write!(
            main_display,
            "{}{}{}",
            termion::cursor::Hide,
            termion::cursor::Goto(1, 1),
            termion::clear::All,
        )
        .unwrap();

        main_display.flush().unwrap();

        let (cols, rows) = termion::terminal_size().unwrap();
        let buffer_size = (cols * rows) as usize;

        let pixel_buffer = vec![Pixel::from(' '); buffer_size];

        Self {
            drop_strings,
            main_display,
            pixel_buffer,
            size: Point::new(cols as i32, rows as i32),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestScreen {
        main_display: Vec<u8>,
        pixel_buffer: Vec<Pixel>,
        size: Point,
    }

    impl TestScreen {
        fn new() -> Self {
            Self {
                main_display: Vec::new(),
                pixel_buffer: vec![Pixel::default(); 81],
                size: Point::new(9, 9),
            }
        }
    }

    impl Screen for TestScreen {
        fn get_pixel(&self, index: usize) -> &Pixel {
            &self.pixel_buffer[index]
        }

        fn set_pixel(&mut self, index: usize, pixel: &Pixel) {
            self.pixel_buffer[index] = pixel.clone();
        }

        fn flush_pixels(&mut self, raw_pixels: &[u8]) {
            self.main_display.write_all(raw_pixels).unwrap();
            self.main_display.flush().unwrap();
        }

        fn get_size(&self) -> &Point {
            &self.size
        }

        fn resize(&mut self) -> &Point {
            let (cols, rows) = termion::terminal_size().unwrap();
            self.size = Point::new(cols as i32, rows as i32);
            &self.size
        }

        fn clear(&mut self) {
            let buffer_size = (self.size.width() * self.size.height()) as usize;
            self.pixel_buffer = vec![Pixel::default(); buffer_size];
        }
    }

    #[test]
    fn it_displays_an_empty_screen() {
        let black_black = format!(
            "{}",
            Color {
                bg: Rgba::black(),
                fg: Rgba::black()
            }
        );
        let mut screen = TestScreen::new();
        screen.display();

        let s = String::from_utf8(screen.main_display).unwrap();

        let mut t = String::new();
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 1)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 2)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 3)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 4)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 5)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 6)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 7)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 8)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 9)));

        assert_eq!(s, t);
    }

    #[test]
    fn it_displays_a_screen_with_two_pixels_and_only_bg_color_changes() {
        let black_black = format!(
            "{}",
            Color {
                bg: Rgba::black(),
                fg: Rgba::black()
            }
        );
        let green_bg = Rgba::green().bg_ansi();
        let black_bg = Rgba::black().bg_ansi();

        let mut screen = TestScreen::new();
        screen.draw_pixel(&Point::new(3, 3), &Rgba::green());
        screen.draw_pixel(&Point::new(4, 3), &Rgba::green());
        screen.display();

        let s = String::from_utf8(screen.main_display).unwrap();

        let mut t = String::new();
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 1)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 2)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 3)));
        t.push_str(&format!(
            "{}{black_black}   {green_bg}  {black_bg}    ",
            cursor_goto(1, 4)
        ));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 5)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 6)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 7)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 8)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 9)));

        assert_eq!(s, t);
    }

    #[test]
    fn it_displays_a_screen_with_two_chars_and_fg_and_bg_color_changes() {
        let black_black = format!(
            "{}",
            Color {
                bg: Rgba::black(),
                fg: Rgba::black()
            }
        );
        let green_red = format!(
            "{}",
            Color {
                bg: Rgba::green(),
                fg: Rgba::red()
            }
        );

        let mut screen = TestScreen::new();
        screen.draw_char(
            &Point::new(3, 3),
            &Color {
                bg: Rgba::green(),
                fg: Rgba::red(),
            },
            '#',
        );
        screen.draw_char(
            &Point::new(4, 3),
            &Color {
                bg: Rgba::green(),
                fg: Rgba::red(),
            },
            '#',
        );
        screen.display();

        let s = String::from_utf8(screen.main_display).unwrap();

        let mut t = String::new();
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 1)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 2)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 3)));
        t.push_str(&format!(
            "{}{black_black}   {green_red}##{black_black}    ",
            cursor_goto(1, 4)
        ));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 5)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 6)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 7)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 8)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 9)));

        assert_eq!(s, t);
    }

    #[test]
    fn it_displays_a_screen_with_three_differently_colored_chars() {
        let black_black = format!(
            "{}",
            Color {
                bg: Rgba::black(),
                fg: Rgba::black()
            }
        );
        let green_red = format!(
            "{}",
            Color {
                bg: Rgba::green(),
                fg: Rgba::red()
            }
        );
        let blue_fg = Rgba::blue().fg_ansi();
        let yellow_bg = Rgba::yellow().bg_ansi();

        let mut screen = TestScreen::new();
        screen.draw_char(
            &Point::new(3, 3),
            &Color {
                bg: Rgba::green(),
                fg: Rgba::red(),
            },
            '#',
        );
        screen.draw_char(
            &Point::new(4, 3),
            &Color {
                bg: Rgba::green(),
                fg: Rgba::blue(),
            },
            '#',
        );
        screen.draw_char(
            &Point::new(5, 3),
            &Color {
                bg: Rgba::yellow(),
                fg: Rgba::blue(),
            },
            '#',
        );
        screen.display();

        let s = String::from_utf8(screen.main_display).unwrap();

        let mut t = String::new();
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 1)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 2)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 3)));
        t.push_str(&format!(
            "{}{black_black}   {green_red}#{blue_fg}#{yellow_bg}#{black_black}   ",
            cursor_goto(1, 4)
        ));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 5)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 6)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 7)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 8)));
        t.push_str(&format!("{}{black_black}         ", cursor_goto(1, 9)));

        assert_eq!(s, t);
    }
}
