use specs::prelude::*;

use super::{ai, gamelog, map, spawner, systems::*, Map, RunState};

pub mod cheat_actions;

pub mod vendor_actions;
pub use vendor_actions::VendorMode;

pub struct State {
    pub ecs: World,
    pub mapgen_next_state: Option<RunState>,
    pub mapgen_history: Vec<Map>,
    pub mapgen_index: usize,
    pub mapgen_timer: f32,
}

///////////////////////////////////////////////////////////////////////////
// Running Systems
///////////////////////////////////////////////////////////////////////////
impl State {
    pub fn run_systems(&mut self) {
        let mut mapindex = MapIndexingSystem {};
        mapindex.run_now(&self.ecs);

        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);

        let mut encumbrance = ai::EncumbranceSystem {};
        encumbrance.run_now(&self.ecs);

        let mut initiative = ai::InitiativeSystem {};
        initiative.run_now(&self.ecs);

        let mut turnstatus = ai::TurnStatusSystem {};
        turnstatus.run_now(&self.ecs);

        let mut quipper = ai::QuipSystem {};
        quipper.run_now(&self.ecs);

        let mut adjacent = ai::AdjacentAI {};
        adjacent.run_now(&self.ecs);

        let mut visible = ai::VisibleAI {};
        visible.run_now(&self.ecs);

        let mut approach = ai::ApproachAI {};
        approach.run_now(&self.ecs);

        let mut flee = ai::FleeAI {};
        flee.run_now(&self.ecs);

        let mut chase = ai::ChaseAI {};
        chase.run_now(&self.ecs);

        let mut defaultmove = ai::DefaultMoveAI {};
        defaultmove.run_now(&self.ecs);

        let mut moving = MovementSystem {};
        moving.run_now(&self.ecs);

        let mut triggers = TriggerSystem {};
        triggers.run_now(&self.ecs);

        let mut melee = MeleeCombatSystem {};
        melee.run_now(&self.ecs);

        let mut damage = DamageSystem {};
        damage.run_now(&self.ecs);

        let mut pickup = ItemCollectionSystem {};
        pickup.run_now(&self.ecs);

        let mut item_use = ItemUseSystem {};
        item_use.run_now(&self.ecs);

        let mut item_id = ItemIdentificationSystem {};
        item_id.run_now(&self.ecs);

        let mut drop_items = ItemDropSystem {};
        drop_items.run_now(&self.ecs);

        let mut item_remove = ItemRemoveSystem {};
        item_remove.run_now(&self.ecs);

        let mut hunger = hunger_system::HungerSystem {};
        hunger.run_now(&self.ecs);

        let mut particles = particle_system::ParticleSpawnSystem {};
        particles.run_now(&self.ecs);

        let mut lighting = LightingSystem {};
        lighting.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

///////////////////////////////////////////////////////////////////////////
// Helper Functions
///////////////////////////////////////////////////////////////////////////
impl State {
    pub fn goto_level(&mut self, offset: i32) {
        map::freeze_level_entities(&mut self.ecs);

        // Build a new map and place the player
        let current_depth = self.ecs.fetch::<Map>().depth;
        self.generate_world_map(current_depth + offset, offset);

        // Notify the player
        let mut gamelog = self.ecs.fetch_mut::<gamelog::GameLog>();
        gamelog.entries.push("You change level.".to_string());
    }

    pub fn game_over_cleanup(&mut self) {
        // Delete everything
        let mut to_delete = Vec::new();
        for e in self.ecs.entities().join() {
            to_delete.push(e);
        }
        for del in to_delete.iter() {
            self.ecs.delete_entity(*del).expect("Deletion failed");
        }

        // Spawn a new player
        {
            let player_entity = spawner::player(&mut self.ecs, 0, 0);
            let mut player_entity_writer = self.ecs.write_resource::<Entity>();
            *player_entity_writer = player_entity;
        }

        // Replace the world maps
        self.ecs.insert(map::MasterDungeonMap::new());

        // Build a new map and place the player
        self.generate_world_map(1, 0);
    }

    pub fn generate_world_map(&mut self, new_depth: i32, offset: i32) {
        self.mapgen_index = 0;
        self.mapgen_timer = 0.0;
        self.mapgen_history.clear();
        let map_building_info = map::level_transition(&mut self.ecs, new_depth, offset);
        if let Some(history) = map_building_info {
            self.mapgen_history = history;
        } else {
            map::thaw_level_entities(&mut self.ecs);
        }
    }
}
