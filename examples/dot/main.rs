use term2d::{
    add,
    color::{Color, Rgb},
    point::Point,
    run,
    screen::DefaultScreen,
    Event, Key,
};

struct Controller {}

impl term2d::Controller for Controller {
    fn update(&mut self, context: &mut term2d::Context) -> bool {
        match context.event {
            Event::Key(key) => match key {
                Key::Char('q') => return false,
                Key::Ctrl('c') => return false,
                _ => {}
            },
            Event::Resize => {}
            Event::Elapse => {}
        }

        context.screen.clear();
        context.screen.draw_pixel(
            Point::new(5, 5),
            Color {
                bg: Rgb::red(),
                fg: Rgb::black(),
            },
        );
        context.screen.display();

        true
    }

    fn get_config(&self) -> term2d::Config {
        term2d::Config { fps: 10 }
    }
}

fn main() {
    let controller = Controller {};
    run(controller);
}
