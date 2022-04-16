pub mod faction_structs;
pub mod item_structs;
pub mod loot_structs;
pub mod mob_structs;
pub mod prop_structs;
pub mod spawn_table_structs;
pub mod spell_structs;

pub use faction_structs::*;
pub use item_structs::*;
pub use loot_structs::*;
pub use mob_structs::*;
pub use prop_structs::*;
pub use spawn_table_structs::*;
pub use spell_structs::*;

use core::fmt::Debug;

pub trait BaseRawComponent {
    fn name(&self) -> String;
    fn renderable(&self) -> Option<&Renderable>;
    // fn as_any(&self) -> &dyn Any;
}

impl Debug for dyn BaseRawComponent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "BaseRawComponent{{{}}}", self.name())
    }
}
