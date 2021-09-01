use std::collections::HashMap;

use futures::executor;
use macroquad::prelude::*;

use crate::constants::*;
use crate::{game::Game, utils::euclideanpoint::*, utils::kleinpoint::*, utils::poincarepoint::*};

/// Raycaster in hyperbolic space.
pub struct FppRenderer {
    textures: HashMap<String, Texture2D>,
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
    pub async fn new() -> FppRenderer {
        FppRenderer {
            textures: Self::load_textures().await,
        }
    }

    async fn load_textures() -> HashMap<String, Texture2D> {
        let mut textures = HashMap::new();
        textures.insert(
            "WALL".to_string(),
            load_texture("assets/textures/wall.png").await.unwrap(),
        );
        textures.insert(
            "MARBLE".to_string(),
            load_texture("assets/textures/marble.png").await.unwrap(),
        );
        textures.insert(
            "CONCRETE".to_string(),
            load_texture("assets/textures/concrete.png").await.unwrap(),
        );

        textures
    }

    /// Renders one frame into a canvas.
    ///
    /// # Parameters:
    ///		- canvas		The canvas that should be drawn to.
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
