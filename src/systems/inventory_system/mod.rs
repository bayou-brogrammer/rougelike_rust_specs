use specs::prelude::*;

use super::{
    effects, AreaOfEffect, CursedItem, EquipmentChanged, Equippable, Equipped, IdentifiedItem, InBackpack, Item,
    MagicItem, Map, MasterDungeonMap, Name, ObfuscatedName, Position, WantsToCastSpell, WantsToDropItem,
    WantsToPickupItem, WantsToRemoveItem, WantsToUseItem,
};

mod collection_system;
mod drop_system;
mod identification_system;
mod remove_system;
mod use_equip;
mod use_system;

pub use collection_system::ItemCollectionSystem;
pub use drop_system::ItemDropSystem;
pub use identification_system::ItemIdentificationSystem;
pub use remove_system::ItemRemoveSystem;
pub use use_equip::ItemEquipOnUse;
pub use use_system::{ItemUseSystem, SpellUseSystem};

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
