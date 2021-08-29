use crate::game::hypermap::HyperMap;
use serde::Deserialize;
use super::color::*;
use super::hyperpoint::*;
use super::point::*;
use super::kleinpoint::*;

pub struct CartesianPoint {
    pub x: f64,
    pub y: f64
}

pub struct CartesianWall  {
    pub beginning: CartesianPoint,
    pub end: CartesianPoint,
    pub texture: String,
    pub height: f64
}

impl From<&Hyperpoint> for CartesianPoint {
    fn from(h: &Hyperpoint) -> CartesianPoint {
        // get polar coords on hyperboloid
        let angle = h.angle();
        let distance = h.distance_to_origin();

        // map to cartesian coords
        CartesianPoint {
            x: distance * angle.cos(),
            y: distance * angle.sin(),
        }
    }
}

impl From<&KleinPoint> for CartesianPoint {
    fn from(h: &KleinPoint) -> CartesianPoint {
        // get polar coords on hyperboloid
        let angle = h.angle();
        let distance = h.distance_to_origin();

        // map to cartesian coords
        CartesianPoint {
            x: distance * angle.cos(),
            y: distance * angle.sin(),
        }
    }
}

impl From<&HyperWall> for CartesianWall {
    fn from(h: &HyperWall) -> CartesianWall {
        CartesianWall {
            beginning: CartesianPoint::from(&h.beginning),
            end: CartesianPoint::from(&h.end),
            texture: h.texture.clone(),
            height: h.height
        }
    }
}

impl From<&KleinWall> for CartesianWall {
    fn from(h: &KleinWall) -> CartesianWall {
        CartesianWall {
            beginning: CartesianPoint::from(&h.beginning),
            end: CartesianPoint::from(&h.end),
            texture: h.texture.clone(),
            height: h.height
        }
    }
}
pub struct CartesianObject {
    pub position: CartesianPoint,
    pub active: bool
}

impl From<&HyperObject> for CartesianObject {
    fn from(object: &HyperObject) -> CartesianObject {
        CartesianObject {
            position: CartesianPoint::from(&object.position),
            active: object.active
        }
    }
}

impl From<&KleinObject> for CartesianObject {
    fn from(object: &KleinObject) -> CartesianObject {
        CartesianObject {
            position: CartesianPoint::from(&object.position),
            active: object.active
        }
    }
}