use std::cmp::Ordering;

use hyperpoint::{HyperWall, Hyperpoint};
use nalgebra::*;
use point::Point;
use serde::Deserialize;

use crate::utils::hyperpoint;

use super::{color::RGBColor, hyperpoint::HyperObject, point};

/// Struct representing a point on the
/// Klein disk model.
/// Wrapper for nalgebra's Point2.
#[derive(Clone, Debug, Deserialize)]
pub struct KleinPoint(pub Point2<f64>);

impl From<Hyperpoint> for KleinPoint {
    fn from(hyperpoint: Hyperpoint) -> Self {
        let denom = hyperpoint.0.x.powi(2)
            + hyperpoint.0.y.powi(2)
            + hyperpoint.0.z.powi(2)
            + hyperpoint.0.z * 2.0
            + 1.0;
        let x = (2.0 * hyperpoint.0.x * (1.0 + hyperpoint.0.z)) / denom;
        let y = (2.0 * hyperpoint.0.y * (1.0 + hyperpoint.0.z)) / denom;
        KleinPoint::new(x, y)
    }
}

impl From<&Hyperpoint> for KleinPoint {
    fn from(hyperpoint: &Hyperpoint) -> Self {
        let denom = hyperpoint.0.x.powi(2)
            + hyperpoint.0.y.powi(2)
            + hyperpoint.0.z.powi(2)
            + hyperpoint.0.z * 2.0
            + 1.0;
        let x = (2.0 * hyperpoint.0.x * (1.0 + hyperpoint.0.z)) / denom;
        let y = (2.0 * hyperpoint.0.y * (1.0 + hyperpoint.0.z)) / denom;
        KleinPoint::new(x, y)
    }
}

impl KleinPoint {
    pub fn new(x: f64, y: f64) -> KleinPoint {
        KleinPoint {
            0: Point2::<f64>::new(x, y),
        }
    }
}

impl point::Point for KleinPoint {
    /// Return the Minkowski inner product of the two vectors provided, where the
    /// last co-ordinate is interpreted as being time-like.
    fn minkowski_dot(a: &KleinPoint, b: &KleinPoint) -> f64 {
        a.0[0] * b.0[0] - a.0[1] * b.0[1]
    }

    /// Distance to origin in the Klein metric.
    fn distance_to_origin(&self) -> f64 {
        let eucl = (self.0[0].powi(2) + self.0[1].powi(2)).sqrt();
        eucl.atanh()
    }

    /*fn distance_to_origin(&self) -> f64 {
        let euclidian_distance =
            (self.0[0].powi(2) + self.0[1].powi(2))
            .sqrt();
        euclidian_distance
    }*/

    /// New point at 0, 0.
    fn new_at_origin() -> Self {
        KleinPoint::new(0., 0.)
    }

    /// Distance to another point in the Klein metric.
    fn distance_to(&self, to: &Self) -> f64 {
        0.
    }

    fn angle(&self) -> f64 {
        self.0.y.atan2(self.0.x)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct KleinWall {
    pub beginning: KleinPoint,
    pub end: KleinPoint,
    pub texture: String,
    pub height: f64,
}

impl From<HyperWall> for KleinWall {
    fn from(hyperwall: HyperWall) -> KleinWall {
        KleinWall {
            beginning: hyperwall.beginning.into(),
            end: hyperwall.end.into(),
            texture: hyperwall.texture,
            height: hyperwall.height,
        }
    }
}

impl KleinWall {}

#[derive(Deserialize)]
pub struct KleinObject {
    pub position: KleinPoint,
    pub active: bool,
}

impl From<&HyperObject> for KleinObject {
    fn from(obj: &HyperObject) -> KleinObject {
        KleinObject {
            position: KleinPoint::from(&obj.position),
            active: obj.active,
        }
    }
}
