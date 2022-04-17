// use super::{parse::parse_dice_string, RawMaster};

use super::{
    raws::{parse_dice_string, Item, MagicItem, SpawnTableEntry},
    RawMaster,
};

impl RawMaster {
    pub fn build_base_magic_item(&self, nmw: &super::NewMagicItem) -> Item {
        let base_item_index = self.item_index[&nmw.name];
        let mut base_item_copy = self.raws.items[base_item_index].clone();
        base_item_copy.vendor_category = None;

        if nmw.bonus == -1 {
            base_item_copy.name = format!("{} -1", nmw.name);
        } else {
            base_item_copy.name = format!("{} +{}", nmw.name, nmw.bonus);
        }

        base_item_copy.magic = Some(MagicItem {
            class: match nmw.bonus {
                2 => "rare".to_string(),
                3 => "rare".to_string(),
                4 => "rare".to_string(),
                5 => "legendary".to_string(),
                _ => "common".to_string(),
            },
            naming: base_item_copy
                .template_magic
                .as_ref()
                .unwrap()
                .unidentified_name
                .clone(),
            cursed: if nmw.bonus == -1 { Some(true) } else { None },
        });

        if let Some(initiative_penalty) = base_item_copy.initiative_penalty.as_mut() {
            *initiative_penalty -= nmw.bonus as f32;
        }

        if let Some(base_value) = base_item_copy.base_value.as_mut() {
            *base_value += (nmw.bonus as f32 + 1.0) * 50.0;
        }

        if let Some(mut weapon) = base_item_copy.weapon.as_mut() {
            weapon.hit_bonus += nmw.bonus;

            let (n, die, plus) = parse_dice_string(&weapon.base_damage);
            let final_bonus = plus + nmw.bonus;

            weapon.base_damage = match final_bonus.cmp(&0) {
                std::cmp::Ordering::Greater => format!("{}d{}", n, die),
                _ => format!("{}d{}-{}", n, die, i32::abs(final_bonus)),
            };
        }

        if let Some(mut armor) = base_item_copy.wearable.as_mut() {
            armor.armor_class += nmw.bonus as f32;
        }

        base_item_copy
    }

    pub fn build_magic_weapon_or_armor(&mut self, items_to_build: &[super::NewMagicItem]) {
        for nmw in items_to_build.iter() {
            let base_item_copy = self.build_base_magic_item(nmw);

            let real_name = base_item_copy.name.clone();
            self.raws.items.push(base_item_copy);
            self.item_index.insert(real_name.clone(), self.raws.items.len() - 1);

            self.raws.spawn_table.push(SpawnTableEntry {
                name: real_name.clone(),
                weight: 10 - i32::abs(nmw.bonus),
                min_depth: 1 + i32::abs((nmw.bonus - 1) * 3),
                max_depth: 100,
                add_map_depth_to_weight: None,
            });
        }
    }

    pub fn build_traited_weapons(&mut self, items_to_build: &[super::NewMagicItem]) {
        items_to_build.iter().filter(|i| i.bonus > 0).for_each(|nmw| {
            for wt in self.raws.weapon_traits.iter() {
                let mut base_item_copy = self.build_base_magic_item(nmw);
                if let Some(mut weapon) = base_item_copy.weapon.as_mut() {
                    base_item_copy.name = format!("{} {}", wt.name, base_item_copy.name);
                    if let Some(base_value) = base_item_copy.base_value.as_mut() {
                        *base_value *= 2.0;
                    }
                    weapon.proc_chance = Some(0.25);
                    weapon.proc_effects = Some(wt.effects.clone());

                    let real_name = base_item_copy.name.clone();
                    self.raws.items.push(base_item_copy);
                    self.item_index.insert(real_name.clone(), self.raws.items.len() - 1);

                    self.raws.spawn_table.push(SpawnTableEntry {
                        name: real_name.clone(),
                        weight: 9 - i32::abs(nmw.bonus),
                        min_depth: 2 + i32::abs((nmw.bonus - 1) * 3),
                        max_depth: 100,
                        add_map_depth_to_weight: None,
                    });
                }
            }
        });
    }
}
