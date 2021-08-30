pub mod hypermap;
pub mod textures;

use macroquad::prelude::*;
use macroquad::ui::*;

use crate::constants::*;
use crate::game::hypermap::*;
use crate::utils::kleinpoint::KleinWall;

/// Represents the state of our game's virtual world
pub struct Game {
    /// The map of our virtual world
    pub map: HyperMap

}

impl Game {
    /// Initializes a new game based on a given map and player.
    pub fn new(map: HyperMap) -> Game {
        Game { map }
    }

    /// Rotates the player's viewing angle with the given angle.
    ///
    /// # Parameters:
    ///		- `angle`:		The angle the player should rotated with (0…2π).
    pub fn rotate_player(&mut self, step: f64) {
        self.map.rotate(step);
    }

    /// Moves the player by the given distance in its current viewing direction. The player is not moved if it would collide with a wall.
    ///
    /// # Parameters:
    ///		- `distance:		The distance the player should be moved by.
    pub fn move_player(&mut self, distance: f64) {
        self.move_player_internal(distance, 0.0);
    }

    /// Moves the player by the given distance in its current viewing direction. The player is not moved if it would collide with a wall.
    ///
    /// # Parameters:
    ///		- `distance:		The distance the player should be moved by.
    pub fn strafe_player(&mut self, distance: f64) {
        self.move_player_internal(0.0, distance);
    }

    /// TODO:: collision checks
    fn move_player_internal(&mut self, dx: f64, dy: f64) {
        self.map.translate(dx, dy);
    }

    /// Actions taken every frame.
    pub fn tick(&mut self) {
        self.solve_wall_collisions();
    }   

    fn solve_wall_collisions(&mut self) {
        // use klein because fuck it
        let walls = self.map
        .get_walls_iter()
        .map(|wall| {
            KleinWall::from(wall.clone())
        });

        let corrections = walls.filter_map(|wall| {
            let x1 = wall.beginning.0.x;
            let y1 = wall.beginning.0.y;
            let x2 = wall.end.0.x;
            let y2 = wall.end.0.y;

            let numerator = ((x2 - x1) * y1 - (y2 - y1) * x1).abs();
            let denominator = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
            let distance = numerator / denominator;
            
            // We only want to detect collision, if the player is actually
            // colliding with the wall, not with the line on which the wall lies.
            // So we can imagine a bounding rectangle around the wall. We only
            // count a collision, if the bounding box contains the origin.
            // We can achieve that by checking whether X or Y coordinates
            // of wall beginnings and ends have opposite signs (pass through origin).
            let within_bounding_box = x1*x2 <= 0. && y1*y2 <= 0.;
            //println!("x1*x2: {}, y1*y2: {}", x1*x2, y1*y2);
            fn distance_between(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
                ((ax-bx).powi(2) + (ay-by).powi(2)).sqrt()
            }

            let helper = distance_between(x1, y1, 0., 0.) + distance_between(x2, y2, 0., 0.) - distance_between(x1, y1, x2, y2);
            let is_between = -EPSILON < helper && helper < EPSILON;

            let collision = distance < COLLISION_RADIUS && is_between;
            if collision { 
                println!("collision at {}", distance); 
                let normal = Vec2::new((x2-x1) as f32, (y2-y1) as f32);
                let difference = distance - COLLISION_RADIUS;
                let normal_scaled = normal.clamp_length_max(difference as f32);

                return Some(normal_scaled);
            }
            None
        });
        
        let sum_corrections = corrections.reduce(|acc, correction| {
            acc + correction
        });

        if let Some(v) = sum_corrections {
            self.move_player_internal(v.y as f64, v.x as f64);
            println!("Correcting player position by x:{}, y:{}", -v.x, v.y);
        }
    }

    pub fn display_hud(&self) {
        let total_objects = self.map.get_objects_iter().count();
        let inactive_objects = self.map
            .get_objects_iter()
            .filter(|o| o.active)
            .count();
        if inactive_objects == total_objects {
            root_ui().label(None, "You won!");
        } else {
            root_ui().label(None, &format!("{}/{} found...", inactive_objects, total_objects));
        }
    }
}

