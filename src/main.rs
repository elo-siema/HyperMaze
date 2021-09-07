mod constants;
mod fpp_renderer;
mod game;
mod top_down_renderer;
mod utils;

use constants::*;
use fpp_renderer::*;
use futures::{executor, task::Spawn};
use game::*;
use macroquad::prelude::*;
use top_down_renderer::*;
use svgloader::*;

use crate::game::hypermap::HyperMap;

fn window_conf() -> Conf {
    Conf {
        window_title: "HyperMaze".to_owned(),
        window_width: GAME_SIZE_X,
        window_height: GAME_SIZE_Y,
        ..Default::default()
    }
}

fn show_loading() {
    clear_background(BLACK);
    draw_text("Loading...", 50., 100., 100., RED);
    draw_text("Tip: Press TAB for minimap", 50., 200., 50., RED);
}

#[macroquad::main(window_conf)]
async fn main() {
    let map = load_map("assets/map.svg");
    // Experiment determined that we need to display two frames
    // in order for the loading screen to be shown.
    show_loading();
    next_frame().await;
    show_loading();
    next_frame().await;

    // Initialize the game
    let mut game = Game::new(map);
    
    //let mut game = Game::new(HyperMap::new(include_str!("../assets/demolarge.json")));

    // Initialize the renderers. This takes a bit of time
    // because it needs to load the textures.
    let fpp_renderer = executor::block_on(FppRenderer::new());
    let top_down_renderer = TopDownRenderer::new();

    loop {
        // Update the game
        let movement = if is_key_down(KEY_FASTER) {
            MOVEMENT_SPEED * get_frame_time() as f64 * 2.
        } else {
            MOVEMENT_SPEED * get_frame_time() as f64
        };
        let rotation = ROTATION_SPEED * get_frame_time() as f64;
        if is_key_down(KEY_FORWARD) || is_key_down(KEY_FORWARD_ALT) {
            game.move_player(-movement);
        }
        if is_key_down(KEY_BACKWARD) || is_key_down(KEY_BACKWARD_ALT) {
            game.move_player(movement);
        }
        if is_key_down(KEY_STRAFE_L) {
            game.strafe_player(movement);
        }
        if is_key_down(KEY_STRAFE_R) {
            game.strafe_player(-movement);
        }
        if is_key_down(KEY_LEFT) {
            game.rotate_player(-rotation);
        }
        if is_key_down(KEY_RIGHT) {
            game.rotate_player(rotation);
        }
        if is_key_down(KEY_EXIT) {
            std::process::exit(0);
        }

        game.tick();

        // Render the game
        if is_key_down(KEY_CHANGE_VIEW) {
            top_down_renderer.render(&game)
        } else {
            fpp_renderer.render(&game);
        }
        game.display_hud();
        next_frame().await
    }
}
