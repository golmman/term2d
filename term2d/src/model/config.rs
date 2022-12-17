use super::ansiesc::CLEAR_ALL;
use super::ansiesc::COLOR_RESET;
use super::ansiesc::CURSOR_GOTO_1_1;
use super::ansiesc::CURSOR_SHOW;

pub struct Config {
    pub fps: u16,
    pub screen_drop_strings: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fps: 10,
            screen_drop_strings: vec![
                COLOR_RESET.to_string(),
                CLEAR_ALL.to_string(),
                CURSOR_GOTO_1_1.to_string(),
                CURSOR_SHOW.to_string(),
            ],
        }
    }
}
