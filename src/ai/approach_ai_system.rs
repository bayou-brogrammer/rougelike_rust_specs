use specs::prelude::*;

use super::{ApplyMove, Map, MyTurn, Position, WantsToApproach};

pub struct ApproachAI {}

impl<'a> System<'a> for ApproachAI {
    type SystemData = (
        WriteStorage<'a, MyTurn>,
        WriteStorage<'a, WantsToApproach>,
        ReadStorage<'a, Position>,
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, ApplyMove>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut turns, mut want_approach, positions, map, entities, mut apply_move) = data;

        let mut turn_done: Vec<Entity> = Vec::new();
        for (entity, pos, approach, _myturn) in (&entities, &positions, &want_approach, &turns).join() {
            turn_done.push(entity);

            let path = rltk::a_star_search(
                map.xy_idx(pos.x, pos.y) as i32,
                map.xy_idx(approach.idx % map.width, approach.idx / map.width) as i32,
                &*map,
            );

            if path.success && path.steps.len() > 1 {
                apply_move
                    .insert(
                        entity,
                        ApplyMove {
                            dest_idx: path.steps[1],
                        },
                    )
                    .expect("Unable to insert");
            }
        }

        want_approach.clear();

        // Remove turn marker for those that are done
        for done in turn_done.iter() {
            turns.remove(*done);
        }
    }
}
