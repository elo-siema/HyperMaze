mod constants;
mod game;
mod fpp_renderer;
mod top_down_renderer;
mod utils;

use macroquad::prelude::*;
use constants::*;
use fpp_renderer::*;
use top_down_renderer::*;
use game::*;

use crate::game::hypermap::HyperMap;

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


    //let mut top_down_view = false;
    let mut game = Game::new(HyperMap::new(include_str!("../assets/demo.json")));
    let fpp_renderer = FppRenderer::new(1.0, 0.75, 1., 0.25);
    let top_down_renderer = TopDownRenderer::new(1.0, 0.75, 1., 0.25);



    loop {
        clear_background(RED);

        //draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        //draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        //draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        //draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        let movement = MOVEMENT_SPEED*get_frame_time();
        let rotation = ROTATION_SPEED*get_frame_time();
        if is_key_down(KEY_FORWARD) { game.move_player(-movement as f64); }
        if is_key_down(KEY_BACKWARD) { game.move_player(movement as f64); }
        if is_key_down(KEY_STRAFE_L) { game.strafe_player(-movement as f64); }
        if is_key_down(KEY_STRAFE_R) { game.strafe_player(movement as f64); }
        if is_key_down(KEY_LEFT) { game.rotate_player(rotation as f64); }
        if is_key_down(KEY_RIGHT) { game.rotate_player(-rotation as f64); }
        if is_key_down(KEY_EXIT) { std::process::exit(0); }
        if is_key_down(KEY_CHANGE_VIEW) { 
            top_down_renderer.render(&game) 
        }
        else { 
            fpp_renderer.render(&game); 
        }
        next_frame().await
    }
}