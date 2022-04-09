use specs::prelude::*;

use crate::{MagicItem, MasterDungeonMap, Name, ObfuscatedName};

pub fn obfuscate_name(
    item: Entity,
    names: &ReadStorage<Name>,
    magic_items: &ReadStorage<MagicItem>,
    obfuscated_names: &ReadStorage<ObfuscatedName>,
    dm: &MasterDungeonMap,
) -> String {
    if let Some(name) = names.get(item) {
        if magic_items.get(item).is_some() {
            if dm.identified_items.contains(&name.name) {
                name.name.clone()
            } else if let Some(obfuscated) = obfuscated_names.get(item) {
                obfuscated.name.clone()
            } else {
                "Unidentified magic item".to_string()
            }
        } else {
            name.name.clone()
        }
    } else {
        "Nameless item (bug)".to_string()
    }
}
