use super::color::*;
use super::hyperpoint::*;
use super::poincarepoint::*;
use super::polarpoint::*;
use super::point::*;

pub struct CartesianPoint {
    pub x: f64,
    pub y: f64
}

pub struct CartesianWall  {
    pub beginning: CartesianPoint,
    pub end: CartesianPoint,
    pub color: RGBColor,
}

impl From<&PolarPoint> for CartesianPoint {
    fn from(h: &PolarPoint) -> CartesianPoint {
        CartesianPoint {
            x: h.distance * h.angle.cos(),
            y: h.distance * h.angle.sin(),
        }
    }
}

impl From<&PolarWall> for CartesianWall {
    fn from(h: &PolarWall) -> CartesianWall {
        CartesianWall {
            beginning: CartesianPoint::from(&h.beginning),
            end: CartesianPoint::from(&h.end),
            color: h.color.clone()
        }
    }
}