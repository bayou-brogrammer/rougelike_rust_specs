use super::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        ReadStorage<'a, Pools>,
        ReadStorage<'a, TileSize>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, position, blockers, pools, sizes, entities) = data;

        crate::spatial::clear();
        crate::spatial::populate_blocked_from_map(&*map);

        for (entity, position) in (&entities, &position).join() {
            let mut alive = true;
            if let Some(pools) = pools.get(entity) {
                if pools.hit_points.current < 1 {
                    alive = false;
                }
            }

            if alive {
                if let Some(size) = sizes.get(entity) {
                    // Multi-tile
                    for y in position.y..position.y + size.y {
                        for x in position.x..position.x + size.x {
                            if x > 0 && x < map.width - 1 && y > 0 && y < map.height - 1 {
                                let idx = map.xy_idx(x, y);
                                crate::spatial::index_entity(entity, idx, blockers.get(entity).is_some());
                            }
                        }
                    }
                } else {
                    // Single tile
                    let idx = map.xy_idx(position.x, position.y);
                    crate::spatial::index_entity(entity, idx, blockers.get(entity).is_some());
                }
            }
        }
    }
}
