pub mod hypermap;
pub mod svgloader;

use macroquad::prelude::*;
use macroquad::ui::*;

use crate::constants::*;
use crate::game::hypermap::*;
use crate::utils::kleinpoint::*;
use crate::utils::point::Point;

/// Represents the state of our game's virtual world
pub struct Game {
    /// The map of our virtual world
    pub map: HyperMap,
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

    fn move_player_internal(&mut self, dx: f64, dy: f64) {
        self.map.translate(dx, dy);
    }

    /// Actions taken every frame.
    pub fn tick(&mut self) {
        self.solve_wall_collisions();
        self.solve_object_collisions();
    }

    /// Detects collisions with objects, marks them as collected.
    fn solve_object_collisions(&mut self) {
        self.map
            .get_objects_iter_mut()
            .filter(|o| o.active)
            .for_each(|o| {
                let pos_klein = KleinPoint::from(o.position);
                let distance = pos_klein.distance_to_origin();

                if distance < (OBJECT_RADIUS as f64 + COLLISION_RADIUS) {
                    o.active = false;
                }
            });
    }

    /// Detects collisions with walls, adjusts players position.
    fn solve_wall_collisions(&mut self) {
        let walls = self
            .map
            .get_walls_iter()
            .map(|wall| KleinWall::from(wall.clone()));

        // First, find collisions with walls - approximate them to lines
        let corrections = walls.filter_map(|wall| {
            let x1 = wall.end.0.x;
            let y1 = wall.end.0.y;
            let x2 = wall.beginning.0.x;
            let y2 = wall.beginning.0.y;

            let lccollision = line_circle_collision_avg(x1, y1, x2, y2, COLLISION_RADIUS);

            let distance = match lccollision {
                Some(lccolresult) => distance_between(lccolresult.0, lccolresult.1, 0., 0.),
                None => f64::INFINITY,
            };

            let helper = distance_between(x1, y1, 0., 0.) + distance_between(x2, y2, 0., 0.)
                - distance_between(x1, y1, x2, y2);
            let is_between = -EPSILON < helper && helper < EPSILON;

            let collision = distance < COLLISION_RADIUS && is_between;

            if collision {
                //println!("collision at {} ", distance);

                let normal = Vec2::new(
                    -lccollision.unwrap().0 as f32,
                    lccollision.unwrap().1 as f32,
                );
                let difference = distance - COLLISION_RADIUS;
                //println!("difference: {} ", difference);

                let normal_scaled = normal.clamp_length_max(difference as f32);

                return Some(normal_scaled);
            }
            None
        });

        // Next, find collisions with wall ends - approximate them to circles
        let mut vertices = vec![];

        self.map
            .get_walls_iter()
            .map(|wall| KleinWall::from(wall.clone()))
            .for_each(|wall| {
                // Get points coords along with vectors which point where to push player out
                // beginning:
                let beg_direction = Vec2::new(
                    (wall.beginning.0.x - wall.end.0.x) as f32,
                    (wall.beginning.0.y - wall.end.0.y) as f32,
                )
                .normalize();
                let beg = (wall.beginning.0.clone(), beg_direction);
                vertices.push(beg);
                // end:
                let end_direction = Vec2::new(
                    (wall.end.0.x - wall.beginning.0.x) as f32,
                    (wall.end.0.y - wall.beginning.0.y) as f32,
                )
                .normalize();
                let end = (wall.end.0.clone(), end_direction);
                vertices.push(end);
            });

        let vertex_corrections = vertices.iter().filter_map(|&tuple| {
            let v = tuple.0;
            let direction = tuple.1;

            let distance = distance_between(v.x, v.y, 0., 0.);
            let collision = distance < COLLISION_RADIUS;
            if collision {
                let normal = Vec2::new(-v.x as f32, -v.y as f32) + direction;
                let difference = distance - COLLISION_RADIUS;
                let normal_scaled = normal.clamp_length_max(difference as f32);

                return Some(normal_scaled);
            }
            None
        });

        //Apply corrections from both sources
        let sum_corrections = corrections
            .chain(vertex_corrections)
            .reduce(|acc, correction| acc + correction);

        if let Some(v) = sum_corrections {
            self.move_player_internal(v.x as f64, v.y as f64);
            //println!("Correcting player position by x:{}, y:{}", v.x, v.y);
        }
    }

    /// Displays current score / information about win.
    pub fn display_hud(&self) {
        let total_objects = self.map.get_objects_iter().count();
        let inactive_objects = self.map.get_objects_iter().filter(|o| !o.active).count();
        if inactive_objects == total_objects {
            root_ui().label(None, "You won!");
        } else {
            root_ui().label(
                None,
                &format!("{}/{} found...", inactive_objects, total_objects),
            );
        }
    }
}

/// Helper function. Euclidean distance between 2 points.
fn distance_between(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    ((ax - bx).powi(2) + (ay - by).powi(2)).sqrt()
}

/// Helper function. Finds approximate point on a line which collides with circle at (0,0).
fn line_circle_collision_avg(x1: f64, y1: f64, x2: f64, y2: f64, r: f64) -> Option<(f64, f64)> {
    let a = (y2 - y1) / (x2 - x1);
    let b = y1 - a * x1;

    let common = ((a.powi(2) + 1.) * r.powi(2) - b.powi(2)).sqrt();
    let denom = a.powi(2) + 1.;

    let result_x1 = (-common - a * b) / denom;
    let result_x2 = (common - a * b) / denom;

    let result_y1 = (b - a * common) / denom;
    let result_y2 = (b + a * common) / denom;

    let result_x = (result_x1 + result_x2) / 2.;
    let result_y = (result_y1 + result_y2) / 2.;

    Some((result_x, result_y))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_line_circle_collision() {
        let expected = (0.5, 0.5);
        let result = line_circle_collision_avg(2., -1., -1., 2., 1.);
        assert_eq!(expected, result.unwrap());

        let expected = (0.5, 0.5);
        let result = line_circle_collision_avg(-1., 2., 2., -1., 1.);
        assert_eq!(expected, result.unwrap());

        let expected = (-0.5, 0.5);
        let result = line_circle_collision_avg(-1., 0., 0., 1., 1.);
        assert_eq!(expected, result.unwrap());
    }
}
