use macroquad::prelude::*;

use crate::apple::Apple;
use crate::direction::Direction;
use crate::snake::Snake;

pub struct Game {
    snake: Snake,
    apple: Apple,
    move_timer: f32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            snake: Snake::new(),
            apple: Apple::new(),
            move_timer: 0.0,
        }
    }

    pub fn update(&mut self) {
        // Gestion du clavier
        if is_key_pressed(KeyCode::Up) {
            self.snake.direction = Direction::Up;
        }

        if is_key_pressed(KeyCode::Down) {
            self.snake.direction = Direction::Down;
        }

        if is_key_pressed(KeyCode::Left) {
            self.snake.direction = Direction::Left;
        }

        if is_key_pressed(KeyCode::Right) {
            self.snake.direction = Direction::Right;
        }

        // Timer pour ralentir le déplacement
        self.move_timer += get_frame_time();

        if self.move_timer >= 0.15 {
            self.move_timer = 0.0;

            self.snake.move_forward();

            let head = self.snake.body[0];

            if head.x == self.apple.position.x && head.y == self.apple.position.y {
                self.snake.grow();
                self.apple = Apple::new();
            }
        }
    }

    pub fn draw(&self) {
        clear_background(BLACK);

        self.apple.draw();
        self.snake.visual();
    }
}