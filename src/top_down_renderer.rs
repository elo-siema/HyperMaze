use crate::constants::_ASPECT_RATIO;
use crate::utils::color::RGBColor;
use crate::{game::Game, utils::poincarepoint::*};
use line_drawing::Bresenham;
use macroquad::prelude::*;

/// Draws a top-down view on a Poincare disk.
pub struct TopDownRenderer {
}

impl TopDownRenderer {
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
    ) -> TopDownRenderer {
        TopDownRenderer {
        }
    }

    /// Renders one frame into a canvas.
    ///
    /// # Parameters:
    ///		- canvas		The canvas that should be drawn to.
    pub fn render(&self, game: &Game) {

        clear_background(BLACK);
        set_camera(&Camera2D {
            target: vec2(0., 0.),
            zoom: vec2(1./_ASPECT_RATIO,1.),
            rotation: 90.,
            ..Default::default()
        });
        draw_circle_lines(0., 0., 1., 0.005,  WHITE);
        draw_circle(0., 0., 0.005, WHITE);
        
        //draw walls:
        game
            .map
            .get_walls_as_poincare()
            .iter()
            .map(|w| w.clone().into())
            .for_each(|wall: PoincareWall| {
                self.draw_wall(&wall);
            });

        //draw objects:
        game
            .map
            .get_objects_iter()
            .for_each(|object| {
                self.draw_object(&object.into());
            });
    }

    // Draws wall as a line on the Poincare disk model.
    fn draw_wall(&self, wall: &PoincareWall) {
        draw_line(
            wall.beginning.0.x as f32, 
            wall.beginning.0.y as f32, 
            wall.end.0.x as f32, 
            wall.end.0.y as f32, 
            0.005, 
            BLUE);
    }

    fn draw_object(&self, object: &PoincareObject) {
        if object.active {
            draw_circle(
                object.position.0.x as f32,
                object.position.0.y as f32,
                0.005,
                RED);
        } else {
            draw_circle_lines(
                object.position.0.x as f32,
                object.position.0.y as f32,
                0.005,
                0.003,
                RED);
        }
    }

    //    let start =
    //        self.translate_to_canvas_coords(wall.beginning.0[0], wall.beginning.0[1], canvas);
    //    let end = self.translate_to_canvas_coords(wall.end.0[0], wall.end.0[1], canvas);
//
    //    for (x, y) in Bresenham::new(start, end) {
    //        canvas.draw_pixel(x as usize, y as usize, &wall.color);
    //    }
    //}

    // todo:: Consider the size of the canvas.
    //fn translate_to_canvas_coords(&self, x: f64, y: f64, canvas: &Canvas) -> (i32, i32) {
    //    let window_height = canvas.height();
    //    let window_width = canvas.width();
    //    let dim_diff = ((window_width - window_height) / 2) as i32;
    //    let left_pad = if dim_diff > 0 {dim_diff} else {0};
    //    let top_pad = if dim_diff < 0 {-dim_diff} else {0};
//
    //    (
    //        ((y + 1.) * (window_height as f64) * 0.5) as i32 + left_pad,
    //        ((-x + 1.) * (window_height as f64) * 0.5) as i32 + top_pad,
    //    )
    //}
}
