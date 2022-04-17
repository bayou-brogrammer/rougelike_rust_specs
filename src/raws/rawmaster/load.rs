use std::collections::{HashMap, HashSet};

use super::{
    raws::{BaseRawComponent, Item, Reaction},
    RawMaster, Raws,
};

pub struct NewMagicItem {
    pub name: String,
    pub bonus: i32,
}

impl RawMaster {
    pub fn load(&mut self, raws: Raws) {
        self.raws = raws;

        self.item_index = HashMap::new();
        let mut used_names: HashSet<String> = HashSet::new();
        let mut items_to_build: Vec<NewMagicItem> = Vec::new();

        // Items
        load_entity_data(
            &self.raws.items,
            &mut self.item_index,
            &mut used_names,
            &mut items_to_build,
        );
        // Mobs
        load_entity_data(
            &self.raws.mobs,
            &mut self.mob_index,
            &mut used_names,
            &mut items_to_build,
        );
        // Props
        load_entity_data(
            &self.raws.props,
            &mut self.prop_index,
            &mut used_names,
            &mut items_to_build,
        );

        // Spawn Table
        for spawn in self.raws.spawn_table.iter() {
            if !used_names.contains(&spawn.name) {
                rltk::console::log(format!(
                    "WARNING - Spawn tables references unspecified entity {}",
                    spawn.name
                ));
            }
        }

        // Loot Table
        for (i, loot) in self.raws.loot_tables.iter().enumerate() {
            self.loot_index.insert(loot.name.clone(), i);
        }

        // Faction Table
        for faction in self.raws.faction_table.iter() {
            let mut reactions: HashMap<String, Reaction> = HashMap::new();

            for other in faction.responses.iter() {
                reactions.insert(
                    other.0.clone(),
                    match other.1.as_str() {
                        "ignore" => Reaction::Ignore,
                        "flee" => Reaction::Flee,
                        _ => Reaction::Attack,
                    },
                );
            }

            self.faction_index.insert(faction.name.clone(), reactions);
        }

        // Spells
        for (i, spell) in self.raws.spells.iter().enumerate() {
            self.spell_index.insert(spell.name.clone(), i);
        }

        self.build_magic_weapon_or_armor(&items_to_build);
        self.build_traited_weapons(&items_to_build);
    }

    fn append_magic_template(items_to_build: &mut Vec<NewMagicItem>, item: &Item) {
        if let Some(template) = &item.template_magic {
            if item.weapon.is_some() || item.wearable.is_some() {
                if template.include_cursed {
                    items_to_build.push(NewMagicItem {
                        name: item.name.clone(),
                        bonus: -1,
                    });
                }

                for bonus in template.bonus_min..=template.bonus_max {
                    items_to_build.push(NewMagicItem {
                        name: item.name.clone(),
                        bonus,
                    });
                }
            } else {
                rltk::console::log(format!(
                    "{} is marked as templated, but isn't a weapon or armor.",
                    item.name
                ));
            }
        }
    }
}

pub fn load_entity_data<T: 'static + BaseRawComponent>(
    raws: &[T],
    entiy_index: &mut HashMap<String, usize>,
    used_names: &mut HashSet<String>,
    items_to_build: &mut Vec<NewMagicItem>,
) {
    for (i, entity) in raws.iter().enumerate() {
        let entity_name = entity.name();

        if used_names.contains(&entity_name) {
            rltk::console::log(format!("WARNING - duplicate entity name in raws [{}]", entity_name));
        }

        entiy_index.insert(entity_name.clone(), i);
        used_names.insert(entity_name.clone());

        if let Some(item) = &entity.as_any().downcast_ref::<Item>() {
            RawMaster::append_magic_template(items_to_build, item);
        }
    }
}
