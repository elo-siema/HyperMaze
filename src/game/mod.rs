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

            fn distance_between(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
                ((ax-bx).powi(2) + (ay-by).powi(2)).sqrt()
            }

            let numerator = ((x2 - x1) * y1 - (y2 - y1) * x1).abs();
            let denominator = distance_between(x1, y1, x2, y2);//((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
            let distance = numerator / denominator;           
            
            let lccollision = line_circle_collision_avg(x1, y1, x2, y2, COLLISION_RADIUS*2.);
                if let Some (lccolresult) = lccollision {
                    let lc_distance = distance_between(lccolresult.0, lccolresult.1, 0., 0.);
                    //println!("lccolresult: distance:{}, x:{}, y:{}",lc_distance, lccolresult.0, lccolresult.1);
                }

            let distance = match lccollision{
                Some(lccolresult) => distance_between(lccolresult.0, lccolresult.1, 0., 0.),
                None => f64::INFINITY
            };

            let helper = distance_between(x1, y1, 0., 0.) + distance_between(x2, y2, 0., 0.) - distance_between(x1, y1, x2, y2);
            let is_between = -EPSILON < helper && helper < EPSILON;

            let collision = distance < COLLISION_RADIUS*100. && is_between;

            

            if collision { 
                println!("collision at {}", distance); 

                

                // TODO :: using a normal here is wrong, 
                // better to find the exact point of collision
                let normal = Vec2::new((y2-y1) as f32, (x2-x1) as f32);
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
            self.move_player_internal(v.x as f64, v.y as f64);
            println!("Correcting player position by x:{}, y:{}", v.x, v.y);
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

fn line_circle_collision_avg(x1: f64, y1: f64, x2: f64, y2: f64, r: f64) -> Option<(f64, f64)> {
    let slope = (y2-y1)/(x2-x1);
    let intercept = y1 - slope * x1;

    let delta = -4. * (slope.powi(2) + intercept.powi(2) - r.powi(2));

    if(delta < 0.0) { return None } //no collision
    println!("delta: {}", delta);

    let sqrt_delta = delta.sqrt();

    let result_x1 = (-(2.*slope*intercept) - sqrt_delta) / 2.*(slope.powi(2) + 1.);
    let result_x2 = (-(2.*slope*intercept) + sqrt_delta) / 2.*(slope.powi(2) + 1.);
    let result_y1 = slope * result_x1 + intercept;
    let result_y2 = slope * result_x2 + intercept;

    let result_x = (result_x1 + result_x2) / 2.;
    let result_y = (result_y1 + result_y2) / 2.;

    Some((result_x, result_y))
}

fn point_circle_collision(x: f64, y: f64, r: f64) -> bool {
    if r == 0. { return false }
    return x*x+y+y <= r*r;
}

fn line_circle_collision(ax: f64, ay: f64, bx: f64, by: f64, r: f64) -> Option<(f64, f64)> {
    if point_circle_collision(ax, ay, r) {
        return Some((ax, ay));
    }
    if point_circle_collision(bx, by, r) {
        return Some((bx, by));
    }

    //vector d
    let dx = bx - ax;
    let dy = by - ay;

    let lcx = -ax;
    let lcy = -ay;

    let dLen2 = dx*dx + dy*dy;
    let mut px = dx;
    let mut py = dy;
    if dLen2 > 0. {
        let dp = (lcx * dx + lcy * dy) / dLen2;
        px *= dp;
        py *= dp;
    }

    let resultx = ax + px;
    let resulty = ay + py;

    let pLen2 = px*px + py*py;

    if point_circle_collision(resultx, resulty, r) 
        && pLen2 <= dLen2 
        && (px * dx + py * dy) >= 0. {
            return Some((resultx, resulty))
        }

    None
}

