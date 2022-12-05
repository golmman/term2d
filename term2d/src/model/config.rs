pub struct Config {
    pub fps: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self { fps: 10 }
    }
}
