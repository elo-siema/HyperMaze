use std::cmp::*;

/// Trait describing common operations on points,
/// regardless of model.
pub trait Point {
    fn distance_to_origin(&self) -> f64;
    fn distance_to(&self, to: &Self) -> f64;
    fn minkowski_dot(a: &Self, b: &Self) -> f64;
    fn new_at_origin() -> Self;
    fn angle(&self) -> f64;
}

/// Trait describing common operations on walls,
/// regardless of model.
pub trait Wall: Ord + Eq + PartialEq + PartialOrd {
    fn distance_to_closest_point(&self) -> f64;
    fn intersection(&self, angle: f64) -> Option<f64>;
}
