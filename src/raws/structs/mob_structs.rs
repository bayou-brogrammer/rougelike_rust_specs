use serde::Deserialize;
use std::any::Any;

use super::{BaseRawComponent, Renderable};

// Base
#[derive(Deserialize, Debug, Clone)]
pub struct Mob {
    pub name: String,
    pub renderable: Option<Renderable>,
    pub blocks_tile: bool,
    pub stats: MobStats,
    pub vision_range: i32,
    pub ai: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MobStats {
    pub max_hp: i32,
    pub hp: i32,
    pub power: i32,
    pub defense: i32,
}

// Trait Implementations
impl BaseRawComponent for Mob {
    fn name(&self) -> String { self.name.clone() }
    fn renderable(&self) -> Option<Renderable> { self.renderable.clone() }
    fn as_any(&self) -> &dyn Any { self }
}

impl<T: BaseRawComponent> From<&T> for Mob {
    fn from(base: &T) -> Self { base.into() }
}
