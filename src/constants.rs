use macroquad::prelude::*;

//pub const GAME_SIZE_X: i32 = 160;
pub const GAME_SIZE_X: i32 = 1024;
pub const GAME_SIZE_Y: i32 = 768;
pub const GAME_CENTER_X: f32 = GAME_SIZE_X as f32 * 0.5f32;
pub const GAME_CENTER_Y: f32 = GAME_SIZE_Y as f32 * 0.5f32;
pub const _ASPECT_RATIO: f32 = GAME_SIZE_X as f32 / GAME_SIZE_Y as f32;

pub const KEY_RIGHT: KeyCode = KeyCode::Right;
pub const KEY_LEFT: KeyCode = KeyCode::Left;
pub const KEY_FORWARD: KeyCode = KeyCode::Up;
pub const KEY_BACKWARD: KeyCode = KeyCode::Down;
pub const KEY_STRAFE_L: KeyCode = KeyCode::A;
pub const KEY_STRAFE_R: KeyCode = KeyCode::D;

pub const KEY_SHOOT: KeyCode = KeyCode::Space;
pub const KEY_START_GAME: KeyCode = KeyCode::Space;

