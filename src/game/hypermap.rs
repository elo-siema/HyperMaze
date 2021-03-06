use crate::utils::{hyperpoint::*, poincarepoint::*};
use serde::{Deserialize};

/// Represents the map in the Minkowski hyperboloid model.
pub struct HyperMap {
    /// Walls of the map.
    walls: Vec<HyperWall>,
    objects: Vec<HyperObject>,
}

#[derive(Deserialize)]
struct PoincareMap {
    walls: Vec<PoincareWall>,
    objects: Vec<PoincareObject>,
}

impl HyperMap {
    /// Creates a new map from the given JSON string.
    ///
    /// # Parameters
    ///    - `map_string`:	A JSON representation of the map, an array of PoincareWalls.
    pub fn new(map_string: &str) -> HyperMap {
        // Parse JSON to PoincareMap.
        let map: PoincareMap = serde_json::from_str(map_string).unwrap();

        // Then transform them into the Minkowski Hyperboloid as internal representation.
        // This is done so it's easier to do transformations on the points
        let transformed_walls: Vec<HyperWall> = map.walls.into_iter().map(|w| w.into()).collect();
        let transformed_objects: Vec<HyperObject> =
            map.objects.into_iter().map(|o| o.into()).collect();

        HyperMap {
            walls: transformed_walls,
            objects: transformed_objects,
        }
    }

    pub fn new_with(walls: Vec<HyperWall>, objects: Vec<HyperObject>,) -> HyperMap {
        HyperMap {
            walls: walls,
            objects: objects,
        }
    }

    /// Returns iterator of HyperWall references.
    pub fn get_walls_iter(&self) -> impl Iterator<Item = &HyperWall> {
        self.walls.iter()
    }

    /// Returns mutable iterator of HyperObject references.
    pub fn get_objects_iter_mut(&mut self) -> impl Iterator<Item = &mut HyperObject> {
        self.objects.iter_mut()
    }

    /// Returns iterator of HyperObject references.
    pub fn get_objects_iter(&self) -> impl Iterator<Item = &HyperObject> {
        self.objects.iter()
    }

    /// Rotate all walls around an origin.
    pub fn rotate(&mut self, step: f64) {
        for wall in &mut self.walls {
            wall.beginning.rotate(step);
            wall.end.rotate(step);
        }

        for object in &mut self.objects {
            object.position.rotate(step);
        }
    }

    /// Move all walls along the x and y axes.
    pub fn translate(&mut self, x: f64, y: f64) {
        for wall in &mut self.walls {
            wall.beginning.translate(x, y);
            wall.end.translate(x, y);
        }

        for object in &mut self.objects {
            object.position.translate(x, y);
        }
    }
}
