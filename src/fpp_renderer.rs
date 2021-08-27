use macroquad::prelude::*;

use crate::utils::color::RGBColor;
use crate::{game::Game, utils::cartesianpoint::*, utils::polarpoint::*};

enum Hit {
    /// The ray hit a wall with a given color at a given distance.
    Wall { color: RGBColor, distance: f64 },
}
/// Raycaster in hyperbolic space.
pub struct FppRenderer {
    /// The radius around the player where objects should appear illuminated
    pub illumination_radius: f64,
    pub relative_screen_size: f64,
    pub focal_length: f64,

    /// The minimum environment light of the scene
    pub minimum_light: f64,

    pub player_height: f64,
    pub field_of_vision: f64,
}


impl FppRenderer {
    /// Initializes the renderer with a map, a player and a focal length that should be used for rendering.
    ///
    /// # Parameters:
    /// 	- game:						The virtual world state (i.e. the game's map and player position)
    ///		- relative_screen_size:		The size of the physical computer display in relation to a grid field
    ///  	- focal_length:				A focal length that should be used for rendering.
    ///	 	- illumination_radius:		The radius around the player where objects should appear illuminated.
    ///	 	- minimum_öight:			The minimum environment light of the scene.
    ///
    pub fn new(
        relative_screen_size: f64,
        focal_length: f64,
        illumination_radius: f64,
        minimum_light: f64,
    ) -> FppRenderer {
        FppRenderer {
            relative_screen_size,
            focal_length,
            illumination_radius,
            minimum_light,
            player_height: 0.05,
            field_of_vision: std::f64::consts::PI / 2.0,
        }
    }

    /// Renders one frame into a canvas.
    ///
    /// # Parameters:
    ///		- canvas		The canvas that should be drawn to.
    pub fn render(&self, game: &Game) {
        
        clear_background(LIGHTGRAY);
        Self::draw_floor();

        set_camera(&Camera3D {
            position: vec3(0., 0., 0.05),
            up: vec3(0., 0., 1.),
            target: vec3(1., 0., 0.05),
            ..Default::default()
        });

        let walls = game.map.get_walls_iter();

        let walls_polar: Vec<PolarWall> = walls
        .map(|wall| {
            wall.into()
        })
        .collect(); 

        let walls_cartesian: Vec<CartesianWall> = walls_polar
        .iter()
        .map(|wall| {
            wall.into()
        })
        .collect();

        for wall in walls_cartesian {
            Self::draw_wall(&wall);
        }
    }

    fn draw_wall(wall: &CartesianWall) {
        println!("Drawing wall, beg:{},{}, end:{},{}", wall.beginning.x, wall.beginning.y, wall.end.x, wall.end.y);
        let mesh = Mesh{
            vertices: vec![
                macroquad::models::Vertex{
                    position: Vec3::new(wall.beginning.x as f32, wall.beginning.y as f32, 0.),
                    uv: Vec2::new(0., 0.),
                    color: RED
                },
                macroquad::models::Vertex{
                    position: Vec3::new(wall.beginning.x as f32, wall.beginning.y as f32, 0.1),
                    uv: Vec2::new(0., 0.),
                    color: RED
                },
                macroquad::models::Vertex{
                    position: Vec3::new(wall.end.x as f32, wall.end.y as f32, 0.),
                    uv: Vec2::new(0., 0.),
                    color: BLACK
                },
                macroquad::models::Vertex{
                    position: Vec3::new(wall.end.x as f32, wall.end.y as f32, 0.1),
                    uv: Vec2::new(0., 0.),
                    color: BLACK
                },
            ],
            indices: vec![
                0,1,2,1,2,3
            ],
            texture: None
        };
        draw_mesh(&mesh);
    }

    fn draw_floor() {
        let mesh = Mesh{
            vertices: vec![
                macroquad::models::Vertex{
                    position: Vec3::new(-1000., -1000., 0.),
                    uv: Vec2::new(0., 0.),
                    color: GRAY
                },
                macroquad::models::Vertex{
                    position: Vec3::new(-1000., 1000., 0.),
                    uv: Vec2::new(0., 0.),
                    color: GRAY
                },
                macroquad::models::Vertex{
                    position: Vec3::new(1000., -1000., 0.),
                    uv: Vec2::new(0., 0.),
                    color: GRAY
                },
                macroquad::models::Vertex{
                    position: Vec3::new(1000., 1000., 0.),
                    uv: Vec2::new(0., 0.),
                    color: GRAY
                },
            ],
            indices: vec![
                0,1,2,1,2,3
            ],
            texture: None
        };
        draw_mesh(&mesh);
    }


}
