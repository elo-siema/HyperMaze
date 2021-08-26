use crate::utils::color::RGBColor;
use crate::{game::Game, utils::poincarepoint::PoincareWall};

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
        let walls: Vec<PoincareWall> = game.map.get_walls_as_poincare();
 
    }


}
