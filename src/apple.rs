use macroquad::prelude::*;

use crate::snake::Position;

pub struct Apple {
    pub position: Position
}
impl Apple {
    //création de la pomme
    pub fn new() -> Self {
        Self {
            position:Position {
                x: rand::gen_range(0,30),
                y: rand::gen_range(0,30),
            },
        }
    }

    pub fn draw(&self) {
        const CELL_SIZE:f32 = 20.0;

        draw_rectangle(
            self.position.x as f32 * CELL_SIZE,
            self.position.y as f32 * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
            RED,
        );
    }
}