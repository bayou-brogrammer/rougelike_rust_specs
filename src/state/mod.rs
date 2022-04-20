use crate::prelude::*;
use crate::systems::*;

mod actions;
pub use actions::*;

mod runstate;
pub use runstate::*;

pub struct State {
    pub ecs: World,
    pub(crate) mapgen_next_state: Option<RunState>,
    pub(crate) mapgen_history: Vec<Map>,
    pub(crate) mapgen_index: usize,
    pub(crate) mapgen_timer: f32,
    pub(crate) dispatcher: Box<dyn crate::systems::UnifiedDispatcher + 'static>,
}

///////////////////////////////////////////////////////////////////////////
// Running Systems
///////////////////////////////////////////////////////////////////////////
impl State {
    fn run_systems(&mut self) {
        self.dispatcher.run_now(&mut self.ecs);
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
        crate::gamelog::Logger::new().append("You change level.").log();
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

        crate::gamelog::clear_log();
        crate::gamelog::Logger::new()
            .append("Welcome to")
            .append_with_color("Rusty Roguelike", rltk::CYAN)
            .log();

        crate::gamelog::clear_events();
    }
}
