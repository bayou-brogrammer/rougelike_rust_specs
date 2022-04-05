use specs::prelude::*;

use crate::{EquipmentChanged, GameLog, InBackpack, Name, Position, WantsToPickupItem};

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantsToPickupItem>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, InBackpack>,
        WriteStorage<'a, EquipmentChanged>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut gamelog, mut wants_pickup, mut positions, names, mut backpack, mut dirty_equipment) =
            data;

        for pickup in wants_pickup.join() {
            positions.remove(pickup.item);

            backpack
                .insert(
                    pickup.item,
                    InBackpack {
                        owner: pickup.collected_by,
                    },
                )
                .expect("Unable to insert backpack entry");

            dirty_equipment
                .insert(pickup.collected_by, EquipmentChanged {})
                .expect("Unable to insert");

            if pickup.collected_by == *player_entity {
                gamelog
                    .entries
                    .push(format!("You pick up the {}.", names.get(pickup.item).unwrap().name));
            }
        }

        wants_pickup.clear();
    }
}
