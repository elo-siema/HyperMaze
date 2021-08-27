use super::color::*;
use super::hyperpoint::*;
use super::poincarepoint::*;
use super::point::*;

pub struct PolarPoint {
    pub angle: f64,
    pub distance: f64
}

pub struct PolarWall  {
    pub beginning: PolarPoint,
    pub end: PolarPoint,
    pub color: RGBColor,
}

impl From<&Hyperpoint> for PolarPoint {
    fn from(h: &Hyperpoint) -> PolarPoint {
        PolarPoint {
            angle: h.angle(),
            distance: h.distance_to_origin()
        }
    }
}

impl From<&HyperWall> for PolarWall {
    fn from(h: &HyperWall) -> PolarWall {
        PolarWall {
            beginning: PolarPoint::from(&h.beginning),
            end: PolarPoint::from(&h.end),
            color: h.color.clone()
        }
    }
}