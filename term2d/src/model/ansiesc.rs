pub const COLOR_RESET: &'static str = "\x1b[0m";
pub const CLEAR_ALL: &'static str = "\x1b[2J";
pub const CURSOR_GOTO_1_1: &'static str = "\x1b[1;1H";
pub const CURSOR_SHOW: &'static str = "\x1b[?25h";

pub fn cursor_goto(col: i32, row: i32) -> String {
    format!("\x1b[{row};{col}H")
}
