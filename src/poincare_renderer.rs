use crate::constants::_ASPECT_RATIO;
use crate::{game::Game, utils::kleinpoint::*, utils::poincarepoint::*};
use macroquad::camera::Camera2D;
use macroquad::prelude::*;
use nalgebra::base::*;

/// Draws a top-down view on a Klein disk.
pub struct PoincareRenderer {}

impl PoincareRenderer {
    pub fn new() -> PoincareRenderer {
        PoincareRenderer {}
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
            .map(|w| PoincareWall::from(w.clone()))
            .for_each(|wall| {
                self.draw_wall_poincare(&wall);
            });

        // draw objects:
        game.map.get_objects_iter().for_each(|obj| {
            let obj = PoincareObject::from(obj.clone());
            self.draw_object_poincare(&obj);
        });
    }

    fn draw_wall_poincare(&self, wall: &PoincareWall) {
        let (x1, y1) = (wall.beginning.0.x, wall.beginning.0.y);
        let (x2, y2) = (wall.end.0.x, wall.end.0.y);

        // We need to find a third point, through which a circle
        // will pass, on which will lie a geodesic between
        // wall.beginning, wall.end
        // https://math.stackexchange.com/questions/1322444/how-to-construct-a-line-on-a-poincare-disk

        let denom = (x1.powi(2) + y1.powi(2));
        let (x3, y3) = (x1/denom, y1/denom);

        // And now find the circle:
        // https://math.stackexchange.com/questions/213658/get-the-equation-of-a-circle-when-given-3-points 
        let minor11 = nalgebra::Matrix3::new(
            x1, y1, 1.,
            x2, y2, 1.,
            x3, y3, 1.
        );
        let minor12 = nalgebra::Matrix3::new(
            x1.powi(2)+y1.powi(2), y1, 1.,
            x2.powi(2)+y2.powi(2), y2, 1.,
            x3.powi(2)+y3.powi(2), y3, 1.
        );
        let minor13 = nalgebra::Matrix3::new(
            x1.powi(2)+y1.powi(2), x1, 1.,
            x2.powi(2)+y2.powi(2), x2, 1.,
            x3.powi(2)+y3.powi(2), x3, 1.
        );
        let minor14 = nalgebra::Matrix3::new(
            x1.powi(2)+y1.powi(2), x1, y1,
            x2.powi(2)+y2.powi(2), x2, y2,
            x3.powi(2)+y3.powi(2), x3, y3
        );

        let det11 = minor11.determinant();
        let det12 = minor12.determinant();
        let det13 = minor13.determinant();
        let det14 = minor14.determinant();

        let x0 = 0.5 * (det12/det11);
        let y0 = 0.5 * (det13/det11);
        let r = (x0.powi(2) + y0.powi(2)).sqrt();

        println!("x0: {:?}, y0: {:?}, r: {:?}", x0, y0, r);

        draw_circle_lines(
            x0 as f32,
            y0 as f32, 
            r as f32,
            0.003,
            GRAY
        );

        //draw_line(
        //    wall.beginning.0.x as f32,
        //    wall.beginning.0.y as f32,
        //    wall.end.0.x as f32,
        //    wall.end.0.y as f32,
        //    0.005,
        //    BLUE,
        //);
    }

    fn draw_object_poincare(&self, object: &PoincareObject) {
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
