use super::color::*;
use super::hyperpoint::*;
use super::kleinpoint::*;
use super::point::*;
use crate::game::hypermap::HyperMap;
use serde::Deserialize;

pub struct EuclideanPoint {
    pub x: f64,
    pub y: f64,
}

pub struct EuclideanWall {
    pub beginning: EuclideanPoint,
    pub end: EuclideanPoint,
    pub texture: String,
    pub height: f64,
}

impl From<&HyperPoint> for EuclideanPoint {
    fn from(h: &HyperPoint) -> EuclideanPoint {
        // get polar coords on hyperboloid
        let angle = h.angle();
        let distance = h.distance_to_origin();

        // map to euclidean coords
        EuclideanPoint {
            x: distance * angle.cos(),
            y: distance * angle.sin(),
        }
    }
}

impl From<&KleinPoint> for EuclideanPoint {
    fn from(h: &KleinPoint) -> EuclideanPoint {
        // get polar coords on hyperboloid
        let angle = h.angle();
        let distance = h.distance_to_origin();

        // map to euclidean coords
        EuclideanPoint {
            x: distance * angle.cos(),
            y: distance * angle.sin(),
        }
    }
}

impl From<&HyperWall> for EuclideanWall {
    fn from(h: &HyperWall) -> EuclideanWall {
        EuclideanWall {
            beginning: EuclideanPoint::from(&h.beginning),
            end: EuclideanPoint::from(&h.end),
            texture: h.texture.clone(),
            height: h.height,
        }
    }
}

impl From<&KleinWall> for EuclideanWall {
    fn from(h: &KleinWall) -> EuclideanWall {
        EuclideanWall {
            beginning: EuclideanPoint::from(&h.beginning),
            end: EuclideanPoint::from(&h.end),
            texture: h.texture.clone(),
            height: h.height,
        }
    }
}
pub struct EuclideanObject {
    pub position: EuclideanPoint,
    pub active: bool,
}

impl From<&HyperObject> for EuclideanObject {
    fn from(object: &HyperObject) -> EuclideanObject {
        EuclideanObject {
            position: EuclideanPoint::from(&object.position),
            active: object.active,
        }
    }
}

impl From<&KleinObject> for EuclideanObject {
    fn from(object: &KleinObject) -> EuclideanObject {
        EuclideanObject {
            position: EuclideanPoint::from(&object.position),
            active: object.active,
        }
    }
}
