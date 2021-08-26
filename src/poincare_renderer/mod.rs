use crate::utils::color::RGBColor;
use crate::window::canvas::Canvas;
use crate::{game::Game, utils::poincarepoint::PoincareWall};
use line_drawing::Bresenham;

/// Draws a top-down view on a Poincare disk.
pub struct TopDownRenderer {
    /// The size of the physical computer display in relation to a grid field
    pub relative_screen_size: f64,

    /// The focal length used for determining the window angle
    pub focal_length: f64,

    /// The radius around the player where objects should appear illuminated
    pub illumination_radius: f64,

    /// The minimum environment light of the scene
    pub minimum_light: f64,
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
        relative_screen_size: f64,
        focal_length: f64,
        illumination_radius: f64,
        minimum_light: f64,
    ) -> TopDownRenderer {
        TopDownRenderer {
            relative_screen_size,
            focal_length,
            illumination_radius,
            minimum_light,
        }
    }

    /// Renders one frame into a canvas.
    ///
    /// # Parameters:
    ///		- canvas		The canvas that should be drawn to.
    pub fn render(&self, canvas: &mut Canvas, game: &Game) {
        //draw walls:
        game
            .map
            .get_walls_as_poincare()
            .iter()
            .map(|w| w.clone().into())
            .for_each(|wall: PoincareWall| {
                self.draw_wall(&wall, canvas);
            });
    }

    /// Draws wall as a line on the Poincare disk model.
    fn draw_wall(&self, wall: &PoincareWall, canvas: &mut Canvas) {
        let start =
            self.translate_to_canvas_coords(wall.beginning.0[0], wall.beginning.0[1], canvas);
        let end = self.translate_to_canvas_coords(wall.end.0[0], wall.end.0[1], canvas);

        for (x, y) in Bresenham::new(start, end) {
            canvas.draw_pixel(x as usize, y as usize, &wall.color);
        }
    }

    /// todo:: Consider the size of the canvas.
    fn translate_to_canvas_coords(&self, x: f64, y: f64, canvas: &Canvas) -> (i32, i32) {
        let window_height = canvas.height();
        let window_width = canvas.width();
        let dim_diff = ((window_width - window_height) / 2) as i32;
        let left_pad = if dim_diff > 0 {dim_diff} else {0};
        let top_pad = if dim_diff < 0 {-dim_diff} else {0};

        (
            ((y + 1.) * (window_height as f64) * 0.5) as i32 + left_pad,
            ((-x + 1.) * (window_height as f64) * 0.5) as i32 + top_pad,
        )
    }
}
