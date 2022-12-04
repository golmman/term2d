use term2d::{
    color::{Color, Rgba},
    point::Point,
    view::{
        canvas::{halfblock::HalfblockCanvas, Canvas},
        screen::DefaultScreen,
    },
};

use crate::state::State;

pub struct Renderer {
    canvas: HalfblockCanvas,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            canvas: HalfblockCanvas::new(),
        }
    }

    pub fn init(&mut self, screen: DefaultScreen) {
        self.canvas.init(screen);
    }

    pub fn draw(&mut self, state: &State) {
        self.canvas.clear();

        self.draw_world(state);
        self.draw_water(state);
        self.draw_debug(state);
        self.draw_cursor(state);

        self.canvas.display();
    }

    pub fn draw_cursor(&mut self, state: &State) {
        self.canvas.draw_pixel(&state.cursor, &Rgba::red());
    }

    pub fn draw_debug(&mut self, state: &State) {
        self.canvas.draw_text(
            &Point::new(2, 0),
            &Color {
                fg: Rgba::white(),
                bg: Rgba::transparent(),
            },
            &format!("press 'q' to quit, frame: {}", state.frame),
        );
        self.canvas.draw_pixel(&Point::new(10, 7), &Rgba::red());
    }

    pub fn draw_world(&mut self, state: &State) {
        self.canvas.draw_image(
            &Point::new(state.world.pos.x, state.world.pos.y),
            &state.world.image,
        );
    }

    pub fn draw_water(&mut self, state: &State) {
        for droplet in &state.world.water {
            let a = Point::new(
                state.world.pos.x + droplet.pos.x,
                state.world.pos.y + droplet.pos.y,
            );
            self.canvas.draw_pixel(&a, &droplet.rgba);
        }
    }
}
