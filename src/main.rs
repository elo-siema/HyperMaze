mod constants;
mod fpp_renderer;
mod game;
mod top_down_renderer;
mod utils;

use constants::*;
use fpp_renderer::*;
use game::*;
use macroquad::{prelude::*, ui::*};
use top_down_renderer::*;
use svgloader::*;

/// Creates window configuration.
fn window_conf() -> Conf {
    Conf {
        window_title: "HyperMaze".to_owned(),
        window_width: GAME_SIZE_X,
        window_height: GAME_SIZE_Y,
        ..Default::default()
    }
}

/// Applies styles to text displayed on the screen.
fn style_ui() {
    //style ui:
    let skin1 = {
        let label_style = root_ui()
            .style_builder()
            .font(include_bytes!("../assets/BebasNeue-Regular.ttf"))
            .unwrap()
            .text_color(Color::from_rgba(180, 0, 0, 255))
            .font_size(50)
            .margin(RectOffset {
                left: 10.,
                top: 10.,
                ..Default::default()
            })
            .build();
        Skin {
            label_style,
            ..root_ui().default_skin()
        }
    };

    root_ui().push_skin(&skin1);
}

/// Shows loading screen.
fn show_loading() {
    clear_background(BLACK);
    root_ui().label(None, "Loading...");
    root_ui().label(None, "Tip: Press TAB for minimap");
}

/// Main function.
#[macroquad::main(window_conf)]
async fn main() {
    // Apply styles
    style_ui();
    
    // Experiment determined that we need to display two frames
    // in order for the loading screen to be shown.
    show_loading();
    next_frame().await;
    next_frame().await;

    // Initialize the game
    let map = load_map(include_str!("../assets/map2.svg"));
    let mut game = Game::new(map);

    // Initialize the renderers. This takes a bit of time
    // because it needs to load the textures.
    let fpp_renderer = FppRenderer::new();
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
