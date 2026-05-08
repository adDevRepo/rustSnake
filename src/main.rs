use macroquad::prelude::*;

#[macroquad::main("Snake")]
async fn main() {
    loop {
        clear_background(BLACK);
        draw_circle(100.0, 100.0, 10.0, GREEN);

        draw_text("Snake Game", 300.0, 40.0, 40.0, YELLOW);

        next_frame().await;
    }
}