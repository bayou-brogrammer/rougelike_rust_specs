use super::*;

pub mod dispatcher;
pub use dispatcher::UnifiedDispatcher;

pub mod ai;
pub mod damage_system;
pub mod hunger_system;
pub mod inventory_system;
pub mod lighting_system;
pub mod map_indexing_system;
pub mod melee_combat_system;
pub mod movement_system;
pub mod particle_system;
pub mod ranged_combat_system;
pub mod saveload_system;
pub mod trigger_system;
pub mod visibility_system;

use ai::*;
use inventory_system::*;

use hunger_system::HungerSystem;
use lighting_system::LightingSystem;
use map_indexing_system::MapIndexingSystem;
use melee_combat_system::MeleeCombatSystem;
use movement_system::MovementSystem;
use particle_system::ParticleSpawnSystem;
use ranged_combat_system::RangedCombatSystem;
use trigger_system::TriggerSystem;
use visibility_system::VisibilitySystem;

pub fn build() -> Box<dyn dispatcher::UnifiedDispatcher + 'static> { dispatcher::new() }
