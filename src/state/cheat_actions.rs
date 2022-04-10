use specs::prelude::*;

use super::{Map, RunState, State};
use crate::{components::*, gui::CheatMenuResult};

///////////////////////////////////////////////////////////////////////////
// Cheat Menu Helper Functions
///////////////////////////////////////////////////////////////////////////
impl State {
    pub fn handle_cheat_action(&mut self, result: CheatMenuResult) -> RunState {
        match result {
            CheatMenuResult::NoResponse => RunState::ShowCheatMenu,
            CheatMenuResult::Cancel => RunState::AwaitingInput,
            CheatMenuResult::TeleportToExit => {
                self.goto_level(1);
                self.mapgen_next_state = Some(RunState::PreRun);
                RunState::MapGeneration
            },
            CheatMenuResult::Heal => {
                let player = self.ecs.fetch::<Entity>();
                let mut pools = self.ecs.write_storage::<Pools>();
                let mut player_pools = pools.get_mut(*player).unwrap();
                player_pools.hit_points.current = player_pools.hit_points.max;
                RunState::AwaitingInput
            },
            CheatMenuResult::Reveal => {
                let mut map = self.ecs.fetch_mut::<Map>();
                for v in map.revealed_tiles.iter_mut() {
                    *v = true;
                }
                RunState::AwaitingInput
            },
            CheatMenuResult::GodMode => {
                let player = self.ecs.fetch::<Entity>();
                let mut pools = self.ecs.write_storage::<Pools>();
                let mut player_pools = pools.get_mut(*player).unwrap();
                player_pools.god_mode = true;
                RunState::AwaitingInput
            },
        }
    }
}
