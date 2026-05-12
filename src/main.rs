use crate::game::Game;

mod direction;
mod game;
mod snake;
mod apple;


#[macroquad::main("Snake")]
async fn main() {
    let mut game = Game::new();

    loop {
        game.update();
        game.draw();


        macroquad::prelude::next_frame().await;
    }

}