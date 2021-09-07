use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Line;
use svg::parser::Event;
use super::HyperMap;
use crate::utils::*;
use crate::utils::hyperpoint::HyperWall;
use crate::utils::kleinpoint::*;
use crate::constants::*;

pub fn load_map(path: &str) -> HyperMap {
    let mut content = String::new();
    let walls: Vec<_> = svg::open(path, &mut content).unwrap().filter_map(|event| {
        match event {
            Event::Tag(Line, _, attributes) => {
                let id = attributes.get("id").unwrap();
                let stroke = attributes.get("stroke").unwrap();
                let x1 = attributes.get("x1").unwrap().parse::<f64>().unwrap() / 1000. - 1.;
                let x2 = attributes.get("x2").unwrap().parse::<f64>().unwrap() / 1000. - 1.;
                let y1 = attributes.get("y1").unwrap().parse::<f64>().unwrap() / 1000. - 1.;
                let y2 = attributes.get("y2").unwrap().parse::<f64>().unwrap() / 1000. - 1.;

                let beginning = KleinPoint::new(x1, y1);
                let end = KleinPoint::new(x2, y2);
                let wall = KleinWall{
                    beginning: beginning,
                    end: end,
                    texture: "WALL".to_string(),
                    height: WALL_HEIGHT as f64,
                };

                Some(wall)
            }
            _ => None
        }
    }).map(|w| HyperWall::from(w)).collect();

    let map = HyperMap::new_with(walls, vec![]);
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        load_map("fsdfds");
        assert!(true);
    }
}
