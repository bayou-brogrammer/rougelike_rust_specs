use std::collections::HashSet;

use super::{BuilderMap, TileType, TownBuilder};

impl TownBuilder {
    pub fn spawn_dockers(&mut self, build_data: &mut BuilderMap) {
        for (idx, tt) in build_data.map.tiles.iter().enumerate() {
            if *tt == TileType::Bridge && crate::rng::roll_dice(1, 6) == 1 {
                let roll = crate::rng::roll_dice(1, 3);

                match roll {
                    1 => build_data.spawn_list.push((idx, "Dock Worker".to_string())),
                    2 => build_data.spawn_list.push((idx, "Wannabe Pirate".to_string())),
                    _ => build_data.spawn_list.push((idx, "Fisher".to_string())),
                }
            }
        }
    }

    pub fn spawn_townsfolk(&mut self, build_data: &mut BuilderMap, available_building_tiles: &mut HashSet<usize>) {
        for idx in available_building_tiles.iter() {
            if crate::rng::roll_dice(1, 10) == 1 {
                let roll = crate::rng::roll_dice(1, 4);
                match roll {
                    1 => build_data.spawn_list.push((*idx, "Peasant".to_string())),
                    2 => build_data.spawn_list.push((*idx, "Drunk".to_string())),
                    3 => build_data.spawn_list.push((*idx, "Dock Worker".to_string())),
                    _ => build_data.spawn_list.push((*idx, "Fisher".to_string())),
                }
            }
        }
    }
}
