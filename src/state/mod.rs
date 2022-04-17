use crate::prelude::*;

mod actions;
pub use actions::*;

mod runstate;
pub use runstate::*;

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
        let mut mapindex = map_indexing_system::MapIndexingSystem {};
        mapindex.run_now(&self.ecs);

        let mut vis = visibility_system::VisibilitySystem {};
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

        let mut moving = movement_system::MovementSystem {};
        moving.run_now(&self.ecs);

        let mut triggers = trigger_system::TriggerSystem {};
        triggers.run_now(&self.ecs);

        let mut melee = melee_combat_system::MeleeCombatSystem {};
        melee.run_now(&self.ecs);

        let mut ranged = ranged_combat_system::RangedCombatSystem {};
        ranged.run_now(&self.ecs);

        let mut pickup = inventory_system::ItemCollectionSystem {};
        pickup.run_now(&self.ecs);

        let mut itemequip = inventory_system::ItemEquipOnUse {};
        itemequip.run_now(&self.ecs);

        let mut item_use = inventory_system::ItemUseSystem {};
        item_use.run_now(&self.ecs);

        let mut spelluse = inventory_system::SpellUseSystem {};
        spelluse.run_now(&self.ecs);

        let mut item_id = inventory_system::ItemIdentificationSystem {};
        item_id.run_now(&self.ecs);

        let mut drop_items = inventory_system::ItemDropSystem {};
        drop_items.run_now(&self.ecs);

        let mut item_remove = inventory_system::ItemRemoveSystem {};
        item_remove.run_now(&self.ecs);

        let mut hunger = hunger_system::HungerSystem {};
        hunger.run_now(&self.ecs);

        run_effects_queue(&mut self.ecs);

        let mut particles = particle_system::ParticleSpawnSystem {};
        particles.run_now(&self.ecs);

        let mut lighting = lighting_system::LightingSystem {};
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
        gamelog.add("You change level.".to_string());
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
