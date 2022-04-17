use super::*;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    // Core
    PreRun,
    Ticking,
    GameOver,
    AwaitingInput,
    // Level
    SaveGame,
    NextLevel,
    PreviousLevel,
    // Map
    TownPortal,
    MapGeneration,
    MagicMapReveal { row: i32 },
    TeleportingToOtherLevel { x: i32, y: i32, depth: i32 },
    // GUI
    MainMenu { menu_selection: gui::MainMenuSelection },
    ShowCheatMenu,
    ShowDropItem,
    ShowIdentify,
    ShowInventory,
    ShowRemoveCurse,
    ShowRemoveItem,
    ShowTargeting { range: i32, item: Entity },
    ShowVendor { vendor: Entity, mode: VendorMode },
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        let mut newrunstate;
        {
            let run_state = self.ecs.fetch::<RunState>();
            newrunstate = *run_state;
        }

        ctx.cls();
        particle_system::update_particles(&mut self.ecs, ctx);

        match newrunstate {
            RunState::MainMenu { .. } => {},
            RunState::GameOver { .. } => {},
            _ => {
                camera::render_camera(&self.ecs, ctx);
                gui::draw_ui(&self.ecs, ctx);
            },
        }

        match newrunstate {
            RunState::MapGeneration => {
                if !SHOW_MAPGEN_VISUALIZER {
                    newrunstate = self.mapgen_next_state.unwrap();
                } else {
                    ctx.cls();

                    if self.mapgen_index < self.mapgen_history.len() {
                        camera::render_debug_map(&self.mapgen_history[self.mapgen_index], ctx);
                    }

                    self.mapgen_timer += ctx.frame_time_ms;
                    if self.mapgen_timer > 500.0 {
                        self.mapgen_timer = 0.0;
                        self.mapgen_index += 1;
                        if self.mapgen_index >= self.mapgen_history.len() {
                            //self.mapgen_index -= 1;
                            newrunstate = self.mapgen_next_state.unwrap();
                        }
                    }
                }
            },
            RunState::PreRun => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            },
            RunState::AwaitingInput => {
                newrunstate = player::player_input(self, ctx);
            },
            RunState::Ticking => {
                let mut should_change_target = false;

                while newrunstate == RunState::Ticking {
                    self.run_systems();
                    self.ecs.maintain();

                    match *self.ecs.fetch::<RunState>() {
                        RunState::AwaitingInput => {
                            newrunstate = RunState::AwaitingInput;
                            should_change_target = true;
                        },
                        RunState::MagicMapReveal { .. } => newrunstate = RunState::MagicMapReveal { row: 0 },
                        RunState::TownPortal => newrunstate = RunState::TownPortal,
                        RunState::TeleportingToOtherLevel { x, y, depth } => {
                            newrunstate = RunState::TeleportingToOtherLevel { x, y, depth }
                        },
                        RunState::ShowRemoveCurse => newrunstate = RunState::ShowRemoveCurse,
                        RunState::ShowIdentify => newrunstate = RunState::ShowIdentify,
                        _ => newrunstate = RunState::Ticking,
                    }
                }

                if should_change_target {
                    player::end_turn_targeting(&mut self.ecs);
                }
            },
            RunState::ShowInventory => {
                let result = gui::show_inventory(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {},
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let is_ranged = self.ecs.read_storage::<Ranged>();
                        let is_item_ranged = is_ranged.get(item_entity);
                        if let Some(is_item_ranged) = is_item_ranged {
                            newrunstate = RunState::ShowTargeting {
                                range: is_item_ranged.range,
                                item: item_entity,
                            };
                        } else {
                            let mut intent = self.ecs.write_storage::<WantsToUseItem>();
                            intent
                                .insert(
                                    *self.ecs.fetch::<Entity>(),
                                    WantsToUseItem {
                                        item: item_entity,
                                        target: None,
                                    },
                                )
                                .expect("Unable to insert intent");
                            newrunstate = RunState::Ticking;
                        }
                    },
                }
            },
            RunState::ShowDropItem => {
                let result = gui::drop_item_menu(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {},
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let mut intent = self.ecs.write_storage::<WantsToDropItem>();
                        intent
                            .insert(*self.ecs.fetch::<Entity>(), WantsToDropItem { item: item_entity })
                            .expect("Unable to insert intent");

                        newrunstate = RunState::Ticking;
                    },
                }
            },
            RunState::ShowRemoveItem => {
                let result = gui::remove_item_menu(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {},
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let mut intent = self.ecs.write_storage::<WantsToRemoveItem>();
                        intent
                            .insert(*self.ecs.fetch::<Entity>(), WantsToRemoveItem { item: item_entity })
                            .expect("Unable to insert intent");

                        newrunstate = RunState::Ticking;
                    },
                }
            },
            RunState::ShowTargeting { range, item } => {
                let result = gui::ranged_target(self, ctx, range);

                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {},
                    gui::ItemMenuResult::Selected => {
                        if self.ecs.read_storage::<SpellTemplate>().get(item).is_some() {
                            let mut intent = self.ecs.write_storage::<WantsToCastSpell>();
                            intent
                                .insert(
                                    *self.ecs.fetch::<Entity>(),
                                    WantsToCastSpell {
                                        spell: item,
                                        target: result.1,
                                    },
                                )
                                .expect("Unable to insert intent");
                        } else {
                            let mut intent = self.ecs.write_storage::<WantsToUseItem>();
                            intent
                                .insert(*self.ecs.fetch::<Entity>(), WantsToUseItem { item, target: result.1 })
                                .expect("Unable to insert intent");
                        }

                        newrunstate = RunState::Ticking;
                    },
                }
            },
            RunState::ShowVendor { vendor, mode } => {
                let result = gui::show_vendor_menu(self, ctx, vendor, mode);

                match result.0 {
                    gui::VendorResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::VendorResult::NoResponse => {},
                    gui::VendorResult::BuyMode => {
                        newrunstate = RunState::ShowVendor {
                            vendor,
                            mode: VendorMode::Buy,
                        }
                    },
                    gui::VendorResult::SellMode => {
                        newrunstate = RunState::ShowVendor {
                            vendor,
                            mode: VendorMode::Sell,
                        }
                    },
                    gui::VendorResult::Buy => self.buy_items(result.2, result.3),
                    gui::VendorResult::Sell => self.sell_items(result.1),
                }
            },
            RunState::ShowRemoveCurse => {
                let result = gui::remove_curse_menu(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::NoResponse => {},
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        self.ecs.write_storage::<CursedItem>().remove(item_entity);
                        newrunstate = RunState::Ticking;
                    },
                }
            },
            RunState::ShowIdentify => {
                let result = gui::identify_menu(self, ctx);

                match result.0 {
                    gui::ItemMenuResult::NoResponse => {},
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        if let Some(name) = self.ecs.read_storage::<Name>().get(item_entity) {
                            let mut dm = self.ecs.fetch_mut::<MasterDungeonMap>();
                            dm.identified_items.insert(name.name.clone());
                        }
                        newrunstate = RunState::Ticking;
                    },
                }
            },
            RunState::ShowCheatMenu => {
                let result = gui::show_cheat_menu(self, ctx);
                newrunstate = self.handle_cheat_action(result);
            },
            RunState::MainMenu { .. } => {
                let result = gui::main_menu(self, ctx);
                match result {
                    gui::MainMenuResult::NoSelection { selected } => {
                        newrunstate = RunState::MainMenu {
                            menu_selection: selected,
                        }
                    },
                    gui::MainMenuResult::Selected { selected } => match selected {
                        gui::MainMenuSelection::NewGame => newrunstate = RunState::PreRun,
                        gui::MainMenuSelection::LoadGame => {
                            saveload_system::load_game(&mut self.ecs);
                            newrunstate = RunState::AwaitingInput;
                            saveload_system::delete_save();
                        },
                        gui::MainMenuSelection::Quit => {
                            std::process::exit(0);
                        },
                    },
                }
            },
            RunState::GameOver => {
                let result = gui::game_over(ctx);
                match result {
                    gui::GameOverResult::NoSelection => {},
                    gui::GameOverResult::QuitToMenu => {
                        self.game_over_cleanup();
                        newrunstate = RunState::MapGeneration;
                        self.mapgen_next_state = Some(RunState::MainMenu {
                            menu_selection: gui::MainMenuSelection::NewGame,
                        });
                    },
                }
            },
            RunState::SaveGame => {
                saveload_system::save_game(&mut self.ecs);
                newrunstate = RunState::MainMenu {
                    menu_selection: gui::MainMenuSelection::LoadGame,
                };
            },
            RunState::NextLevel => {
                self.goto_level(1);
                self.mapgen_next_state = Some(RunState::PreRun);
                newrunstate = RunState::MapGeneration;
            },
            RunState::PreviousLevel => {
                self.goto_level(-1);
                self.mapgen_next_state = Some(RunState::PreRun);
                newrunstate = RunState::MapGeneration;
            },
            RunState::TownPortal => {
                // Spawn the portal
                spawner::spawn_town_portal(&mut self.ecs);

                // Transition
                let map_depth = self.ecs.fetch::<Map>().depth;
                let destination_offset = 0 - (map_depth - 1);

                self.goto_level(destination_offset);
                self.mapgen_next_state = Some(RunState::PreRun);

                newrunstate = RunState::MapGeneration;
            },
            RunState::TeleportingToOtherLevel { x, y, depth } => {
                self.goto_level(depth - 1);

                let player_entity = self.ecs.fetch::<Entity>();
                if let Some(pos) = self.ecs.write_storage::<Position>().get_mut(*player_entity) {
                    pos.x = x;
                    pos.y = y;
                }

                let mut ppos = self.ecs.fetch_mut::<rltk::Point>();
                ppos.x = x;
                ppos.y = y;
                self.mapgen_next_state = Some(RunState::PreRun);

                newrunstate = RunState::MapGeneration;
            },
            RunState::MagicMapReveal { row } => {
                let mut map = self.ecs.fetch_mut::<Map>();
                for x in 0..map.width {
                    let idx = map.xy_idx(x as i32, row);
                    map.revealed_tiles[idx] = true;
                }
                if row == map.height - 1 {
                    newrunstate = RunState::Ticking;
                } else {
                    newrunstate = RunState::MagicMapReveal { row: row + 1 };
                }
            },
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }

        damage_system::delete_the_dead(&mut self.ecs);
    }
}
