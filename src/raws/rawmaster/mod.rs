use specs::prelude::*;
use std::collections::{HashMap, HashSet};

pub use super::{structs::BaseRawComponent, Raws};
use crate::{components::*, gamesystem::*, random_table::RandomTable};

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

fn spawn_named_item(new_entity: EntityBuilder, item_template: super::Item) -> Option<Entity> {
    let mut eb = new_entity;

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

fn spawn_named_mob(new_entity: EntityBuilder, mob_template: super::Mob) -> Option<Entity> {
    let mut eb = new_entity;

    match mob_template.ai.as_ref() {
        "melee" => eb = eb.with(Monster {}),
        "bystander" => eb = eb.with(Bystander {}),
        "vendor" => eb = eb.with(Vendor {}),
        _ => {},
    }

    ///////////////////////////////////////////////////////////////////////////
    // BlocksTile
    ///////////////////////////////////////////////////////////////////////////
    if mob_template.blocks_tile {
        eb = eb.with(BlocksTile {});
    }

    ///////////////////////////////////////////////////////////////////////////
    // Viewshed
    ///////////////////////////////////////////////////////////////////////////
    eb = eb.with(Viewshed {
        visible_tiles: Vec::new(),
        range: mob_template.vision_range,
        dirty: true,
    });

    ///////////////////////////////////////////////////////////////////////////
    // Quips
    ///////////////////////////////////////////////////////////////////////////
    if let Some(quips) = &mob_template.quips {
        eb = eb.with(Quips {
            available: quips.clone(),
        });
    }

    ///////////////////////////////////////////////////////////////////////////
    // Atrributes
    ///////////////////////////////////////////////////////////////////////////
    let mut mob_fitness = 11;
    let mut mob_int = 11;

    #[rustfmt::skip]
    let mut attr = Attributes{
        might: Attribute{ base: 11, modifiers: 0, bonus: attr_bonus(11) },
        fitness: Attribute{ base: 11, modifiers: 0, bonus: attr_bonus(11) },
        quickness: Attribute{ base: 11, modifiers: 0, bonus: attr_bonus(11) },
        intelligence: Attribute{ base: 11, modifiers: 0, bonus: attr_bonus(11) },
    };

    // might
    if let Some(might) = mob_template.attributes.might {
        attr.might = Attribute {
            base: might,
            modifiers: 0,
            bonus: attr_bonus(might),
        };
    }

    // fitness
    if let Some(fitness) = mob_template.attributes.fitness {
        attr.fitness = Attribute {
            base: fitness,
            modifiers: 0,
            bonus: attr_bonus(fitness),
        };
        mob_fitness = fitness;
    }

    // quickness
    if let Some(quickness) = mob_template.attributes.quickness {
        attr.quickness = Attribute {
            base: quickness,
            modifiers: 0,
            bonus: attr_bonus(quickness),
        };
    }

    // intelligence
    if let Some(intelligence) = mob_template.attributes.intelligence {
        attr.intelligence = Attribute {
            base: intelligence,
            modifiers: 0,
            bonus: attr_bonus(intelligence),
        };
        mob_int = intelligence;
    }
    eb = eb.with(attr);

    ///////////////////////////////////////////////////////////////////////////
    // Pools
    ///////////////////////////////////////////////////////////////////////////
    let mob_level = if mob_template.level.is_some() { mob_template.level.unwrap() } else { 1 };
    let mob_hp = npc_hp(mob_fitness, mob_level);
    let mob_mana = mana_at_level(mob_int, mob_level);

    let pools = Pools {
        level: mob_level,
        xp: 0,
        hit_points: Pool {
            current: mob_hp,
            max: mob_hp,
        },
        mana: Pool {
            current: mob_mana,
            max: mob_mana,
        },
    };
    eb = eb.with(pools);

    ///////////////////////////////////////////////////////////////////////////
    // Skills
    ///////////////////////////////////////////////////////////////////////////
    let mut skills = Skills { skills: HashMap::new() };
    skills.skills.insert(Skill::Melee, 1);
    skills.skills.insert(Skill::Defense, 1);
    skills.skills.insert(Skill::Magic, 1);

    if let Some(mobskills) = &mob_template.skills {
        for sk in mobskills.iter() {
            match sk.0.as_str() {
                "Melee" => {
                    skills.skills.insert(Skill::Melee, *sk.1);
                },
                "Defense" => {
                    skills.skills.insert(Skill::Defense, *sk.1);
                },
                "Magic" => {
                    skills.skills.insert(Skill::Magic, *sk.1);
                },
                _ => {
                    rltk::console::log(format!("Unknown skill referenced: [{}]", sk.0));
                },
            }
        }
    }
    eb = eb.with(skills);

    // Build a mob person thing
    Some(eb.build())
}

fn spawn_named_prop(new_entity: EntityBuilder, prop_template: super::Prop) -> Option<Entity> {
    let mut eb = new_entity;

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
        let (eb, item) = build_base_entity(new_entity, &raws.raws.items, &raws.item_index, key, pos);
        return spawn_named_item(eb, item);
    } else if raws.mob_index.contains_key(key) {
        let (eb, mob) = build_base_entity(new_entity, &raws.raws.mobs, &raws.mob_index, key, pos);
        return spawn_named_mob(eb, mob);
    } else if raws.prop_index.contains_key(key) {
        let (eb, prop) = build_base_entity(new_entity, &raws.raws.props, &raws.prop_index, key, pos);
        return spawn_named_prop(eb, prop);
    }

    None
}
