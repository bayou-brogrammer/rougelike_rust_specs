use specs::prelude::*;
use std::collections::{HashMap, HashSet};

pub use super::{structs::BaseRawComponent, Raws};
use crate::{components::*, random_table::RandomTable};

mod load_helpers;
use load_helpers::*;

mod spawn_helpers;
use spawn_helpers::*;

pub enum SpawnType {
    AtPosition { x: i32, y: i32 },
}

pub struct RawMaster {
    raws: Raws,
    item_index: HashMap<String, usize>,
    mob_index: HashMap<String, usize>,
    prop_index: HashMap<String, usize>,
}

impl RawMaster {
    pub fn empty() -> RawMaster {
        RawMaster {
            raws: Raws {
                items: Vec::new(),
                mobs: Vec::new(),
                props: Vec::new(),
                spawn_table: Vec::new(),
            },
            item_index: HashMap::new(),
            mob_index: HashMap::new(),
            prop_index: HashMap::new(),
        }
    }

    pub fn load(&mut self, raws: Raws) {
        self.raws = raws;

        self.item_index = HashMap::new();
        self.mob_index = HashMap::new();
        self.prop_index = HashMap::new();

        let mut used_names: HashSet<String> = HashSet::new();

        // Items
        load_entity_data(&self.raws.items, &mut self.item_index, &mut used_names);
        // Mobs
        load_entity_data(&self.raws.mobs, &mut self.mob_index, &mut used_names);
        // Props
        load_entity_data(&self.raws.props, &mut self.prop_index, &mut used_names);

        // Spawn Table
        for spawn in self.raws.spawn_table.iter() {
            if !used_names.contains(&spawn.name) {
                rltk::console::log(format!(
                    "WARNING - Spawn tables references unspecified entity {}",
                    spawn.name
                ));
            }
        }
    }
}

fn spawn_base_ent<'a, T: BaseRawComponent>(
    new_entity: EntityBuilder<'a>,
    entity_list: &[T],
    indexes: &HashMap<String, usize>,
    key: &str,
    pos: SpawnType,
) -> Option<(EntityBuilder<'a>, T)> {
    if !indexes.contains_key(key) {
        return None;
    }

    let entity_template = &entity_list[indexes[key]];
    let mut eb = new_entity;

    // Spawn in the specified location
    eb = spawn_position(pos, eb);

    // Renderable
    if let Some(renderable) = &entity_template.renderable() {
        eb = eb.with(get_renderable_component(renderable));
    }

    // // Name Component
    eb = eb.with(Name {
        name: entity_template.name(),
    });

    // Individual Builders
    // if raws.item_index.contains_key(key) {
    //     eb = spawn_named_item(eb, entity_template.into());
    // } else if raws.mob_index.contains_key(key) {
    //     eb = spawn_named_mob(eb, entity_template.into());
    // } else if raws.prop_index.contains_key(key) {
    //     eb = spawn_named_prop(eb, entity_template.into());
    // }

    Some((eb, entity_template.clone()))
}

fn spawn_named_item(raws: &RawMaster, new_entity: EntityBuilder, key: &str, pos: SpawnType) -> Option<Entity> {
    let (mut eb, item_template) = match spawn_base_ent(new_entity, &raws.raws.items, &raws.item_index, key, pos) {
        None => return None,
        Some(builder) => builder,
    };

    // Item Component
    eb = eb.with(crate::components::Item {});

    // Consumable Component
    if let Some(consumable) = &item_template.consumable {
        eb = eb.with(crate::components::Consumable {});

        for effect in consumable.effects.iter() {
            let effect_name = effect.0.as_str();

            match effect_name {
                "provides_healing" => {
                    eb = eb.with(ProvidesHealing {
                        heal_amount: effect.1.parse::<i32>().unwrap(),
                    })
                },
                "ranged" => {
                    eb = eb.with(Ranged {
                        range: effect.1.parse::<i32>().unwrap(),
                    })
                },
                "damage" => {
                    eb = eb.with(InflictsDamage {
                        damage: effect.1.parse::<i32>().unwrap(),
                    })
                },
                "area_of_effect" => {
                    eb = eb.with(AreaOfEffect {
                        radius: effect.1.parse::<i32>().unwrap(),
                    })
                },
                "confusion" => {
                    eb = eb.with(Confusion {
                        turns: effect.1.parse::<i32>().unwrap(),
                    })
                },
                "magic_mapping" => eb = eb.with(MagicMapper {}),
                "food" => eb = eb.with(ProvidesFood {}),
                _ => {
                    rltk::console::log(format!("Warning: consumable effect {} not implemented.", effect_name));
                },
            }
        }
    }

    // Equippables
    // Weapion Component
    if let Some(weapon) = &item_template.weapon {
        eb = eb.with(Equippable {
            slot: EquipmentSlot::Melee,
        });
        eb = eb.with(MeleePowerBonus {
            power: weapon.power_bonus,
        });
    }

    // Shield Component
    if let Some(shield) = &item_template.shield {
        eb = eb.with(Equippable {
            slot: EquipmentSlot::Shield,
        });
        eb = eb.with(DefenseBonus {
            defense: shield.defense_bonus,
        });
    }

    Some(eb.build())
}

fn spawn_named_mob(raws: &RawMaster, new_entity: EntityBuilder, key: &str, pos: SpawnType) -> Option<Entity> {
    let (mut eb, mob_template) = match spawn_base_ent(new_entity, &raws.raws.mobs, &raws.mob_index, key, pos) {
        None => return None,
        Some(builder) => builder,
    };

    match mob_template.ai.as_ref() {
        "melee" => eb = eb.with(Monster {}),
        "bystander" => eb = eb.with(Bystander {}),
        "vendor" => eb = eb.with(Vendor {}),
        _ => {},
    }

    // BlocksTile
    if mob_template.blocks_tile {
        eb = eb.with(BlocksTile {});
    }

    // Combat
    eb = eb.with(CombatStats {
        max_hp: mob_template.stats.max_hp,
        hp: mob_template.stats.hp,
        power: mob_template.stats.power,
        defense: mob_template.stats.defense,
    });

    // Viewshed
    eb = eb.with(Viewshed {
        visible_tiles: Vec::new(),
        range: mob_template.vision_range,
        dirty: true,
    });

    // Quips
    if let Some(quips) = &mob_template.quips {
        eb = eb.with(Quips {
            available: quips.clone(),
        });
    }

    Some(eb.build())
}

fn spawn_named_prop(raws: &RawMaster, new_entity: EntityBuilder, key: &str, pos: SpawnType) -> Option<Entity> {
    let (mut eb, prop_template) = match spawn_base_ent(new_entity, &raws.raws.props, &raws.prop_index, key, pos) {
        None => return None,
        Some(builder) => builder,
    };

    // Hidden Trait
    if let Some(hidden) = prop_template.hidden {
        if hidden {
            eb = eb.with(Hidden {})
        };
    }

    // Blocks Visibility Trait
    if let Some(blocks_visibility) = prop_template.blocks_visibility {
        if blocks_visibility {
            eb = eb.with(BlocksVisibility {})
        };
    }

    // Door?
    if let Some(door_open) = prop_template.door_open {
        eb = eb.with(Door { open: door_open });
    }

    // Trigger Trait (Traps)
    if let Some(entry_trigger) = &prop_template.entry_trigger {
        eb = eb.with(EntryTrigger {});

        for effect in entry_trigger.effects.iter() {
            match effect.0.as_str() {
                "damage" => {
                    eb = eb.with(InflictsDamage {
                        damage: effect.1.parse::<i32>().unwrap(),
                    })
                },
                "single_activation" => eb = eb.with(SingleActivation {}),
                _ => {},
            }
        }
    }

    Some(eb.build())
}

pub fn get_spawn_table_for_depth(raws: &RawMaster, depth: i32) -> RandomTable {
    use super::SpawnTableEntry;

    let available_options: Vec<&SpawnTableEntry> = raws
        .raws
        .spawn_table
        .iter()
        .filter(|a| depth >= a.min_depth && depth <= a.max_depth)
        .collect();

    let mut rt = RandomTable::new();

    for e in available_options.iter() {
        let mut weight = e.weight;

        if e.add_map_depth_to_weight.is_some() {
            weight += depth;
        }

        rt = rt.add(e.name.clone(), weight);
    }

    rt
}

pub fn spawn_named_entity(raws: &RawMaster, new_entity: EntityBuilder, key: &str, pos: SpawnType) -> Option<Entity> {
    if raws.item_index.contains_key(key) {
        return spawn_named_item(raws, new_entity, key, pos);
    } else if raws.mob_index.contains_key(key) {
        return spawn_named_mob(raws, new_entity, key, pos);
    } else if raws.prop_index.contains_key(key) {
        return spawn_named_prop(raws, new_entity, key, pos);
    }

    None
}
