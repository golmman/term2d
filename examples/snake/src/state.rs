use term2d::model::point::Point;

use crate::random::Random;

pub struct State {
    pub food: Point,
    pub frame: u32,
    pub game_over: bool,
    pub screen_size: Point,
    pub snake: Vec<Point>,

    boundary: (Point, Point),
    direction: Point,
    random: Random,
}

impl State {
    pub fn new() -> Self {
        Self {
            boundary: (Point::new(0, 0), Point::new(0, 0)),
            direction: Point::new(0, 0),
            food: Point::new(0, 0),
            frame: 0,
            game_over: false,
            random: Random::new(),
            screen_size: Point::new(0, 0),
            snake: Vec::new(),
        }
    }

    pub fn resize(&mut self, screen_size: Point) {
        self.screen_size = screen_size;

        self.boundary = (
            Point::new(1, 6),
            Point::new(self.screen_size.width() - 1, self.screen_size.height() - 2),
        );

        self.reset();
    }

    pub fn reset(&mut self) {
        self.game_over = false;
        self.reset_food();
        self.reset_snake();
    }

    pub fn update(&mut self) {
        if self.snake.len() == 0 {
            return;
        }

        if self.game_over {
            return;
        }

        let head = Point::new(
            self.snake[0].x + self.direction.x,
            self.snake[0].y + self.direction.y,
        );

        if !(self.boundary.0.x..self.boundary.1.x).contains(&head.x) {
            self.game_over = true;
            return;
        }

        if !(self.boundary.0.y..self.boundary.1.y).contains(&head.y) {
            self.game_over = true;
            return;
        }

        for i in 3..self.snake.len() {
            if head == self.snake[i] {
                self.game_over = true;
                return;
            }
        }

        if head == self.food {
            let tail_index = self.snake.len() - 1;
            self.snake.push(self.snake[tail_index].clone());
            self.reset_food();
        }

        for i in 1..self.snake.len() {
            let j = self.snake.len() - i;
            self.snake[j] = self.snake[j - 1].clone();
        }

        self.snake[0] = head;
    }

    pub fn go_up(&mut self) {
        if self.direction.y == 1 {
            return;
        }
        self.direction = Point::new(0, -1);
    }

    pub fn go_down(&mut self) {
        if self.direction.y == -1 {
            return;
        }
        self.direction = Point::new(0, 1);
    }

    pub fn go_left(&mut self) {
        if self.direction.x == 1 {
            return;
        }
        self.direction = Point::new(-1, 0);
    }

    pub fn go_right(&mut self) {
        if self.direction.x == -1 {
            return;
        }
        self.direction = Point::new(1, 0);
    }

    fn reset_food(&mut self) {
        let from_x = self.boundary.0.x as u32;
        let to_x = self.boundary.1.x as u32;
        let from_y = self.boundary.0.y as u32;
        let to_y = self.boundary.1.y as u32;

        self.food = Point::new(
            self.random.next_range(from_x..to_x) as i32,
            self.random.next_range(from_y..to_y) as i32,
        );
    }

    fn reset_snake(&mut self) {
        const SNAKE_START_LENGTH: i32 = 5;

        let start_x = (self.boundary.1.x - self.boundary.0.x) / 3
            + self.boundary.0.x
            + SNAKE_START_LENGTH / 2;
        let start_y = (self.boundary.1.y - self.boundary.0.y) / 2 + self.boundary.0.y;

        self.snake = Vec::new();
        for i in 0..SNAKE_START_LENGTH {
            self.snake.push(Point::new(start_x - i, start_y));
        }

        self.direction = Point::new(1, 0);
    }
}
