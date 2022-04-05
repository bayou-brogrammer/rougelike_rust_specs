use std::collections::HashMap;

use specs::{
    prelude::*,
    saveload::{MarkedBuilder, SimpleMarker},
};

use super::{RawMaster, SpawnType};
use crate::{
    random_table::RandomTable,
    raws::structs::*,
    EquipmentSlot,
    Equipped,
    InBackpack,
    Name,
    Position,
    SerializeMe,
};

pub fn spawn_position<'a>(
    pos: SpawnType,
    new_entity: EntityBuilder<'a>,
    tag: &str,
    raws: &RawMaster,
) -> EntityBuilder<'a> {
    let eb = new_entity;

    // Spawn in the specified location
    match pos {
        SpawnType::AtPosition { x, y } => eb.with(Position { x, y }),
        SpawnType::Carried { by } => eb.with(InBackpack { owner: by }),
        SpawnType::Equipped { by } => {
            let slot = find_slot_for_equippable_item(tag, raws);
            eb.with(Equipped { owner: by, slot })
        },
    }
}

pub fn get_renderable_component(renderable: &item_structs::Renderable) -> crate::components::Renderable {
    crate::components::Renderable {
        glyph: rltk::to_cp437(renderable.glyph.chars().next().unwrap()),
        fg: rltk::RGB::from_hex(&renderable.fg).expect("Invalid RGB"),
        bg: rltk::RGB::from_hex(&renderable.bg).expect("Invalid RGB"),
        render_order: renderable.order,
    }
}

pub fn build_base_entity<'a, T: BaseRawComponent + Clone>(
    raws: &RawMaster,
    ecs: &'a mut World,
    entity_list: &[T],
    indexes: &HashMap<String, usize>,
    key: &str,
    pos: SpawnType,
) -> (EntityBuilder<'a>, T) {
    let entity_template = &entity_list[indexes[key]];
    let mut eb = ecs.create_entity().marked::<SimpleMarker<SerializeMe>>();

    // Spawn in the specified location
    eb = spawn_position(pos, eb, key, raws);

    // Renderable
    if let Some(renderable) = &entity_template.renderable() {
        eb = eb.with(get_renderable_component(renderable));
    }

    // // Name Component
    eb = eb.with(Name {
        name: entity_template.name(),
    });

    (eb, entity_template.clone())
}

pub fn string_to_slot(slot: &str) -> EquipmentSlot {
    match slot {
        "Shield" => EquipmentSlot::Shield,
        "Head" => EquipmentSlot::Head,
        "Torso" => EquipmentSlot::Torso,
        "Legs" => EquipmentSlot::Legs,
        "Feet" => EquipmentSlot::Feet,
        "Hands" => EquipmentSlot::Hands,
        "Melee" => EquipmentSlot::Melee,
        _ => {
            rltk::console::log(format!("Warning: unknown equipment slot type [{}])", slot));
            EquipmentSlot::Melee
        },
    }
}

fn find_slot_for_equippable_item(tag: &str, raws: &RawMaster) -> EquipmentSlot {
    if !raws.item_index.contains_key(tag) {
        panic!("Trying to equip an unknown item: {}", tag);
    }

    let item_index = raws.item_index[tag];
    let item = &raws.raws.items[item_index];

    if let Some(_wpn) = &item.weapon {
        return EquipmentSlot::Melee;
    } else if let Some(wearable) = &item.wearable {
        return string_to_slot(&wearable.slot);
    }

    panic!("Trying to equip {}, but it has no slot tag.", tag);
}

pub fn get_item_drop(raws: &RawMaster, rng: &mut rltk::RandomNumberGenerator, table: &str) -> Option<String> {
    if raws.loot_index.contains_key(table) {
        let mut rt = RandomTable::new();
        let available_options = &raws.raws.loot_tables[raws.loot_index[table]];

        for item in available_options.drops.iter() {
            rt = rt.add(item.name.clone(), item.weight);
        }

        return Some(rt.roll(rng));
    }

    None
}

pub fn get_vendor_items(categories: &[String], raws: &RawMaster) -> Vec<(String, f32)> {
    let mut result: Vec<(String, f32)> = Vec::new();

    for item in raws.raws.items.iter() {
        if let Some(cat) = &item.vendor_category {
            if categories.contains(cat) && item.base_value.is_some() {
                result.push((item.name.clone(), item.base_value.unwrap()));
            }
        }
    }

    result
}
