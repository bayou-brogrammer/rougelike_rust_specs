use std::collections::HashSet;

use super::{draw_corridor, BuilderMap, MetaMapBuilder, Rect};
use rltk::RandomNumberGenerator;

pub struct NearestCorridors {}

impl MetaMapBuilder for NearestCorridors {
    #[allow(dead_code)]
    fn build_map(&mut self, rng: &mut rltk::RandomNumberGenerator, build_data: &mut BuilderMap) {
        self.corridors(rng, build_data);
    }
}

impl NearestCorridors {
    #[allow(dead_code)]
    pub fn new() -> Box<NearestCorridors> { Box::new(NearestCorridors {}) }

    fn corridors(&mut self, _rng: &mut RandomNumberGenerator, build_data: &mut BuilderMap) {
        let rooms: Vec<Rect> = if let Some(rooms_builder) = &build_data.rooms {
            rooms_builder.clone()
        } else {
            panic!("Straight Line Corridors require a builder with room structures");
        };

        let mut connected: HashSet<usize> = HashSet::new();
        let mut corridors: Vec<Vec<usize>> = Vec::new();

        for (i, room) in rooms.iter().enumerate() {
            let mut room_distance: Vec<(usize, f32)> = Vec::new();
            let room_center_pt = room.center();

            for (j, other_room) in rooms.iter().enumerate() {
                if i != j && !connected.contains(&j) {
                    let other_center_pt = other_room.center();
                    let distance = rltk::DistanceAlg::Pythagoras.distance2d(room_center_pt, other_center_pt);
                    room_distance.push((j, distance));
                }
            }

            if !room_distance.is_empty() {
                room_distance.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                let dest_center = rooms[room_distance[0].0].center();

                let corridor = draw_corridor(
                    &mut build_data.map,
                    room_center_pt.x,
                    room_center_pt.y,
                    dest_center.x,
                    dest_center.y,
                );

                connected.insert(i);
                build_data.take_snapshot();
                corridors.push(corridor);
            }
        }

        build_data.corridors = Some(corridors);
    }
}
