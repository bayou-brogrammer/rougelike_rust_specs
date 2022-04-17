use serde::Deserialize;
use std::collections::HashMap;

use super::{item_structs::Renderable, BaseRawComponent};

// Trait Implementations
impl BaseRawComponent for Mob {
    fn name(&self) -> String { self.name.clone() }
    fn renderable(&self) -> Option<&Renderable> { self.renderable.as_ref() }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

impl<T: BaseRawComponent> From<&T> for Mob {
    fn from(base: &T) -> Self { base.into() }
}

// Base
#[derive(Deserialize, Debug, Clone)]
pub struct Mob {
    pub name: String,
    pub renderable: Option<Renderable>,
    pub blocks_tile: bool,
    pub vision_range: i32,
    pub movement: String,
    pub quips: Option<Vec<String>>,
    pub attributes: MobAttributes,
    pub skills: Option<HashMap<String, i32>>,
    pub level: Option<i32>,
    pub hp: Option<i32>,
    pub mana: Option<i32>,
    pub equipped: Option<Vec<String>>,
    pub natural: Option<MobNatural>,
    pub loot_table: Option<String>,
    pub light: Option<MobLight>,
    pub faction: Option<String>,
    pub gold: Option<String>,
    pub vendor: Option<Vec<String>>,
    pub abilities: Option<Vec<MobAbility>>,
    pub on_death: Option<Vec<MobAbility>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MobAttributes {
    pub might: Option<i32>,
    pub fitness: Option<i32>,
    pub quickness: Option<i32>,
    pub intelligence: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MobNatural {
    pub armor_class: Option<i32>,
    pub attacks: Option<Vec<NaturalAttack>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NaturalAttack {
    pub name: String,
    pub hit_bonus: i32,
    pub damage: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MobLight {
    pub range: i32,
    pub color: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MobAbility {
    pub spell: String,
    pub chance: f32,
    pub range: f32,
    pub min_range: f32,
}
