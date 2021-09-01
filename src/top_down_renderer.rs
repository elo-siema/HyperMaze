use crate::constants::_ASPECT_RATIO;
use crate::{game::Game, utils::kleinpoint::*};
use macroquad::prelude::*;

/// Draws a top-down view on a Poincare disk.
pub struct TopDownRenderer {}

impl TopDownRenderer {
    pub fn new() -> TopDownRenderer {
        TopDownRenderer {}
    }

    pub fn render(&self, game: &Game) {
        // set camera and outline:
        clear_background(BLACK);
        set_camera(&Camera2D {
            target: vec2(0., 0.),
            zoom: vec2(1. / _ASPECT_RATIO, 1.),
            rotation: 90.,
            ..Default::default()
        });
        draw_circle_lines(0., 0., 1., 0.005, WHITE);
        draw_circle(0., 0., 0.005, WHITE);

        // draw walls:
        game.map
            .get_walls_iter()
            .map(|w| KleinWall::from(w.clone()))
            .for_each(|wall| {
                self.draw_wall_klein(&wall);
            });

        // draw objects:
        game.map.get_objects_iter().for_each(|obj| {
            let obj = KleinObject::from(obj.clone());
            self.draw_object_klein(&obj);
        });
    }

    fn draw_wall_klein(&self, wall: &KleinWall) {
        draw_line(
            wall.beginning.0.x as f32,
            wall.beginning.0.y as f32,
            wall.end.0.x as f32,
            wall.end.0.y as f32,
            0.005,
            BLUE,
        );
    }

    fn draw_object_klein(&self, object: &KleinObject) {
        if object.active {
            draw_circle(
                object.position.0.x as f32,
                object.position.0.y as f32,
                0.005,
                RED,
            );
        } else {
            draw_circle_lines(
                object.position.0.x as f32,
                object.position.0.y as f32,
                0.005,
                0.003,
                BLUE,
            );
        }
    }
}
