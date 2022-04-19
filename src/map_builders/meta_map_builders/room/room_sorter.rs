use super::{BuilderMap, MetaMapBuilder, Rect};
use rltk::RandomNumberGenerator;

#[allow(dead_code)]
pub enum RoomSort {
    Left,
    Right,
    Top,
    Bottom,
    Central,
}

pub struct RoomSorter {
    sort_by: RoomSort,
}

impl MetaMapBuilder for RoomSorter {
    #[allow(dead_code)]
    fn build_map(&mut self, rng: &mut rltk::RandomNumberGenerator, build_data: &mut BuilderMap) {
        self.sorter(rng, build_data);
    }
}

impl RoomSorter {
    #[allow(dead_code)]
    pub fn new(sort_by: RoomSort) -> Box<RoomSorter> { Box::new(RoomSorter { sort_by }) }

    fn sorter(&mut self, _rng: &mut RandomNumberGenerator, build_data: &mut BuilderMap) {
        match self.sort_by {
            RoomSort::Left => build_data.rooms.as_mut().unwrap().sort_by(|a, b| a.x1.cmp(&b.x1)),
            RoomSort::Right => build_data.rooms.as_mut().unwrap().sort_by(|a, b| b.x2.cmp(&a.x2)),
            RoomSort::Top => build_data.rooms.as_mut().unwrap().sort_by(|a, b| a.y1.cmp(&b.y1)),
            RoomSort::Bottom => build_data.rooms.as_mut().unwrap().sort_by(|a, b| b.y2.cmp(&a.y2)),
            RoomSort::Central => {
                let map_center = rltk::Point::new(build_data.map.width / 2, build_data.map.height / 2);
                let center_sort = |a: &Rect, b: &Rect| {
                    let a_center_pt = a.center();
                    let b_center_pt = b.center();

                    let distance_a = rltk::DistanceAlg::Pythagoras.distance2d(a_center_pt, map_center);
                    let distance_b = rltk::DistanceAlg::Pythagoras.distance2d(b_center_pt, map_center);

                    distance_a.partial_cmp(&distance_b).unwrap()
                };

                build_data.rooms.as_mut().unwrap().sort_by(center_sort);
            },
        }
    }
}
