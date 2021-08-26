mod constants;
mod game;
mod hyperbolic_renderer;
mod poincare_renderer;
mod utils;

use macroquad::prelude::*;
use constants::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "HyperCaster".to_owned(),
        window_width: GAME_SIZE_X,
        window_height: GAME_SIZE_Y,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {


    let mut top_down_view = false;

    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}