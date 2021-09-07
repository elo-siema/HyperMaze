use std::collections::HashMap;

use futures::executor;
use macroquad::prelude::*;

use crate::constants::*;
use crate::{game::Game, utils::euclideanpoint::*, utils::kleinpoint::*, utils::poincarepoint::*};

/// FPP renderer in hyperbolic space.
/// Converts world from hyperboloid to Klein model,
/// then uses polar coordinates using the Klein metric to
/// map walls and objects to Euclidean space and render them.
pub struct FppRenderer {
    textures: HashMap<String, Texture2D>,
}
impl FppRenderer {
    /// Initializes the renderer, loads textures.
    pub fn new() -> FppRenderer {
        FppRenderer {
            textures: Self::load_textures(),
        }
    }

    /// Load textures. They are included in the executable at compile time.
    fn load_textures() -> HashMap<String, Texture2D> {
        let mut textures = HashMap::new();
        textures.insert(
            "WALL".to_string(),
            Texture2D::from_file_with_format(include_bytes!("../assets/textures/wall.png"), None)
        );
        textures.insert(
            "MARBLE".to_string(),
            Texture2D::from_file_with_format(include_bytes!("../assets/textures/marble.png"), None)
        );
        textures.insert(
            "CONCRETE".to_string(),
            Texture2D::from_file_with_format(include_bytes!("../assets/textures/concrete.png"), None)
        );

        textures
    }

    /// Renders one frame into the screen.
    pub fn render(&self, game: &Game) {
        clear_background(BLACK);
        Self::draw_floor();

        set_camera(&Camera3D {
            position: vec3(0., 0., 0.05),
            up: vec3(0., 0., 1.),
            target: vec3(1., 0., 0.05),
            ..Default::default()
        });

        let walls_euclidean = game.map.get_walls_iter().map(|wall| {
            let wall = KleinWall::from(wall.clone());
            (&wall).into()
        });

        for wall in walls_euclidean {
            self.draw_wall(&wall);
        }

        let objects_euclidean = game.map.get_objects_iter().map(|obj| {
            let obj = KleinObject::from(obj.clone());
            (&obj).into()
        });

        for obj in objects_euclidean {
            self.draw_object(&obj);
        }
    }

    /// Draws textured sphere.
    fn draw_object(&self, object: &EuclideanObject) {
        if object.active {
            draw_sphere(
                Vec3::new(object.position.x as f32, object.position.y as f32, 0.02),
                OBJECT_RADIUS,
                self.textures.get("MARBLE").unwrap().clone(),
                OBJECT_COLOR,
            );
        } else {
            draw_sphere(
                Vec3::new(object.position.x as f32, object.position.y as f32, 0.02),
                OBJECT_RADIUS,
                self.textures.get("MARBLE").unwrap().clone(),
                Color {
                    r: 0.5,
                    g: 0.5,
                    b: 0.5,
                    a: 0.5,
                },
            );
        }
    }

    /// Draws textured wall.
    fn draw_wall(&self, wall: &EuclideanWall) {
        //println!("Drawing wall, beg:{},{}, end:{},{}", wall.beginning.x, wall.beginning.y, wall.end.x, wall.end.y);
        let mesh = Mesh {
            vertices: vec![
                macroquad::models::Vertex {
                    position: Vec3::new(wall.beginning.x as f32, wall.beginning.y as f32, 0.),
                    uv: Vec2::new(0., 0.),
                    color: WHITE,
                },
                macroquad::models::Vertex {
                    position: Vec3::new(
                        wall.beginning.x as f32,
                        wall.beginning.y as f32,
                        wall.height as f32,
                    ),
                    uv: Vec2::new(0., 1.),
                    color: WHITE,
                },
                macroquad::models::Vertex {
                    position: Vec3::new(wall.end.x as f32, wall.end.y as f32, 0.),
                    uv: Vec2::new(1., 0.),
                    color: WHITE,
                },
                macroquad::models::Vertex {
                    position: Vec3::new(wall.end.x as f32, wall.end.y as f32, wall.height as f32),
                    uv: Vec2::new(1., 1.),
                    color: WHITE,
                },
            ],
            indices: vec![0, 1, 2, 1, 2, 3],
            texture: Some(self.textures.get(&wall.texture).unwrap().clone()),
        };
        draw_mesh(&mesh);
    }

    /// Draws floor as a large gray flat surface.
    fn draw_floor() {
        let mesh = Mesh {
            vertices: vec![
                macroquad::models::Vertex {
                    position: Vec3::new(-1000., -1000., 0.),
                    uv: Vec2::new(0., 0.),
                    color: GRAY,
                },
                macroquad::models::Vertex {
                    position: Vec3::new(-1000., 1000., 0.),
                    uv: Vec2::new(0., 0.),
                    color: GRAY,
                },
                macroquad::models::Vertex {
                    position: Vec3::new(1000., -1000., 0.),
                    uv: Vec2::new(0., 0.),
                    color: GRAY,
                },
                macroquad::models::Vertex {
                    position: Vec3::new(1000., 1000., 0.),
                    uv: Vec2::new(0., 0.),
                    color: GRAY,
                },
            ],
            indices: vec![0, 1, 2, 1, 2, 3],
            texture: None,
        };
        draw_mesh(&mesh);
    }
}
