use specs::prelude::*;

use super::{effects::*, AreaOfEffect, EquipmentChanged, IdentifiedItem, Map, Name, WantsToUseItem};

pub struct ItemUseSystem {}

impl<'a> System<'a> for ItemUseSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, WantsToUseItem>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, AreaOfEffect>,
        WriteStorage<'a, EquipmentChanged>,
        WriteStorage<'a, IdentifiedItem>,
    );

    #[allow(clippy::cognitive_complexity)]
    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, map, entities, mut wants_use, names, aoe, mut dirty_equipment, mut identified_item) = data;

        for (entity, useitem) in (&entities, &wants_use).join() {
            dirty_equipment
                .insert(entity, EquipmentChanged {})
                .expect("Unable to insert");

            // Identify
            if entity == *player_entity {
                identified_item
                    .insert(
                        entity,
                        IdentifiedItem {
                            name: names.get(useitem.item).unwrap().name.clone(),
                        },
                    )
                    .expect("Unable to insert");
            }

            // Call the effects system
            add_effect(
                Some(entity),
                EffectType::ItemUse { item: useitem.item },
                match useitem.target {
                    None => Targets::Single { target: *player_entity },
                    Some(target) => {
                        if let Some(aoe) = aoe.get(useitem.item) {
                            Targets::Tiles {
                                tiles: aoe_tiles(&*map, target, aoe.radius),
                            }
                        } else {
                            Targets::Tile {
                                tile_idx: map.xy_idx(target.x, target.y) as i32,
                            }
                        }
                    },
                },
            );
        }

        wants_use.clear();
    }
}
