use serde::Deserialize;
use std::{any::Any, collections::HashMap};

use super::BaseRawComponent;

#[derive(Deserialize, Debug, Clone)]
pub struct Item {
    pub name: String,
    pub renderable: Option<Renderable>,
    pub consumable: Option<Consumable>,
    pub weapon: Option<Weapon>,
    pub shield: Option<Shield>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Renderable {
    pub glyph: String,
    pub fg: String,
    pub bg: String,
    pub order: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Consumable {
    pub effects: HashMap<String, String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Weapon {
    pub range: String,
    pub power_bonus: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Shield {
    pub defense_bonus: i32,
}

// Trait Implementations
impl BaseRawComponent for Item {
    fn name(&self) -> String { self.name.clone() }
    fn renderable(&self) -> Option<Renderable> { self.renderable.clone() }
    fn as_any(&self) -> &dyn Any { self }
}

impl<T: BaseRawComponent> From<&T> for Item {
    fn from(base: &T) -> Self { base.into() }
}
