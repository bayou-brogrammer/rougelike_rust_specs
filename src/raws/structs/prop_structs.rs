use serde::Deserialize;
use std::collections::HashMap;

use super::{item_structs::Renderable, BaseRawComponent};

// Trait Implementations
impl BaseRawComponent for Prop {
    fn name(&self) -> String { self.name.clone() }
    fn renderable(&self) -> Option<&Renderable> { self.renderable.as_ref() }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

impl<T: BaseRawComponent> From<&T> for Prop {
    fn from(base: &T) -> Self { base.into() }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Prop {
    pub name: String,
    pub renderable: Option<Renderable>,
    pub hidden: Option<bool>,
    pub blocks_tile: Option<bool>,
    pub blocks_visibility: Option<bool>,
    pub door_open: Option<bool>,
    pub entry_trigger: Option<EntryTrigger>,
    pub light: Option<super::mob_structs::MobLight>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EntryTrigger {
    pub effects: HashMap<String, String>,
}
