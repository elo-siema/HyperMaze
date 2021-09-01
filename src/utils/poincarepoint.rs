use std::cmp::Ordering;

use hyperpoint::{HyperPoint, HyperWall};
use nalgebra::*;
use point::Point;
use serde::Deserialize;

use crate::utils::hyperpoint;

use super::{hyperpoint::HyperObject, point};

/// Struct representing a point on the
/// Poincare disk model.
/// Wrapper for nalgebra's Point2.
#[derive(Clone, Debug, Deserialize)]
pub struct PoincarePoint(pub Point2<f64>);

impl From<HyperPoint> for PoincarePoint {
    fn from(hyperpoint: HyperPoint) -> Self {
        let denom = hyperpoint.0[2] + 1.0;
        PoincarePoint::new(hyperpoint.0[0] / denom, hyperpoint.0[1] / denom)
    }
}

impl From<&HyperPoint> for PoincarePoint {
    fn from(hyperpoint: &HyperPoint) -> Self {
        let denom = hyperpoint.0[2] + 1.0;
        PoincarePoint::new(hyperpoint.0[0] / denom, hyperpoint.0[1] / denom)
    }
}

impl PoincarePoint {
    pub fn new(x: f64, y: f64) -> PoincarePoint {
        PoincarePoint {
            0: Point2::<f64>::new(x, y),
        }
    }
}

impl point::Point for PoincarePoint {
    /// Return the Minkowski inner product of the two vectors provided, where the
    /// last co-ordinate is interpreted as being time-like.
    fn minkowski_dot(a: &PoincarePoint, b: &PoincarePoint) -> f64 {
        a.0[0] * b.0[0] - a.0[1] * b.0[1]
    }

    /// Distance to origin in the Poincare metric.
    fn distance_to_origin(&self) -> f64 {
        self.distance_to(&PoincarePoint::new_at_origin())
    }

    /// New point at 0, 0.
    fn new_at_origin() -> Self {
        PoincarePoint::new(0., 0.)
    }

    /// Distance to another point in the Poincare metric.
    fn distance_to(&self, to: &Self) -> f64 {
        let (x1, y1): (f64, f64) = (self.0[0], self.0[1]);
        let (x2, y2): (f64, f64) = (to.0[0], to.0[1]);

        let z1 = nalgebra::Complex::new(x1, y1);
        let z2 = nalgebra::Complex::new(x2, y2);
        let one = nalgebra::Complex::new(1., 0.);

        let upper: Complex<f64> = z1 - z2;
        let lower: Complex<f64> = one - z1 * (z2.conj());
        let div: Complex<f64> = upper / lower;
        let norm: f64 = div.norm();
        let result: f64 = 2. * norm.atanh();
        result
    }

    fn angle(&self) -> f64 {
        self.0.y.atan2(self.0.x)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct PoincareWall {
    pub beginning: PoincarePoint,
    pub end: PoincarePoint,
    pub texture: String,
    pub height: f64,
}

impl From<HyperWall> for PoincareWall {
    fn from(hyperwall: HyperWall) -> PoincareWall {
        PoincareWall {
            beginning: hyperwall.beginning.into(),
            end: hyperwall.end.into(),
            texture: hyperwall.texture,
            height: hyperwall.height,
        }
    }
}

#[derive(Deserialize)]
pub struct PoincareObject {
    pub position: PoincarePoint,
    pub active: bool,
}

impl From<&HyperObject> for PoincareObject {
    fn from(obj: &HyperObject) -> PoincareObject {
        PoincareObject {
            position: PoincarePoint::from(&obj.position),
            active: obj.active,
        }
    }
}
