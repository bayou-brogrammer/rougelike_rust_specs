use specs::prelude::*;
use std::cmp::{max, min};

use super::{
    gamelog::GameLog,
    Attributes,
    BlocksTile,
    BlocksVisibility,
    Bystander,
    Door,
    EntityMoved,
    HungerClock,
    HungerState,
    Item,
    Map,
    Monster,
    Player,
    Pools,
    Position,
    Renderable,
    RunState,
    State,
    TileType,
    Vendor,
    Viewshed,
    WantsToMelee,
    WantsToPickupItem,
};

use rltk::{Point, Rltk, VirtualKeyCode};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let entities = ecs.entities();

    let mut blocks_movement = ecs.write_storage::<BlocksTile>();
    let mut blocks_visibility = ecs.write_storage::<BlocksVisibility>();
    let mut doors = ecs.write_storage::<Door>();
    let mut entity_moved = ecs.write_storage::<EntityMoved>();
    let mut positions = ecs.write_storage::<Position>();
    let mut renderables = ecs.write_storage::<Renderable>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

    let players = ecs.read_storage::<Player>();
    let bystanders = ecs.read_storage::<Bystander>();
    let vendors = ecs.read_storage::<Vendor>();
    let combat_stats = ecs.read_storage::<Attributes>();

    let map = ecs.fetch::<Map>();
    let mut swap_entities: Vec<(Entity, i32, i32)> = Vec::new();

    for (entity, _player, pos, viewshed) in (&entities, &players, &mut positions, &mut viewsheds).join() {
        if pos.x + delta_x < 1
            || pos.x + delta_x > map.width - 1
            || pos.y + delta_y < 1
            || pos.y + delta_y > map.height - 1
        {
            return;
        }
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        for potential_target in map.tile_content[destination_idx].iter() {
            let bystander = bystanders.get(*potential_target);
            let vendor = vendors.get(*potential_target);

            if bystander.is_some() || vendor.is_some() {
                // Note that we want to move the bystander
                swap_entities.push((*potential_target, pos.x, pos.y));

                // Move the player
                pos.x = min(map.width - 1, max(0, pos.x + delta_x));
                pos.y = min(map.height - 1, max(0, pos.y + delta_y));
                entity_moved
                    .insert(entity, EntityMoved {})
                    .expect("Unable to insert marker");

                viewshed.dirty = true;
                let mut ppos = ecs.write_resource::<Point>();
                ppos.x = pos.x;
                ppos.y = pos.y;
            } else {
                let target = combat_stats.get(*potential_target);
                // Attack Target
                if let Some(_target) = target {
                    wants_to_melee
                        .insert(
                            entity,
                            WantsToMelee {
                                target: *potential_target,
                            },
                        )
                        .expect("Add target failed");
                    return;
                }
            }

            // Check if Door
            let door = doors.get_mut(*potential_target);
            if let Some(door) = door {
                door.open = true;
                viewshed.dirty = true;

                blocks_visibility.remove(*potential_target);
                blocks_movement.remove(*potential_target);

                let glyph = renderables.get_mut(*potential_target).unwrap();
                glyph.glyph = rltk::to_cp437('/');
            }
        }

        if !map.blocked[destination_idx] {
            pos.x = min(map.width - 1, max(0, pos.x + delta_x));
            pos.y = min(map.height - 1, max(0, pos.y + delta_y));

            entity_moved
                .insert(entity, EntityMoved {})
                .expect("Unable to insert marker");

            viewshed.dirty = true;

            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }

    for m in swap_entities.iter() {
        let their_pos = positions.get_mut(m.0);
        if let Some(their_pos) = their_pos {
            their_pos.x = m.1;
            their_pos.y = m.2;
        }
    }
}

pub fn try_next_level(ecs: &mut World) -> bool {
    let player_pos = ecs.fetch::<Point>();
    let map = ecs.fetch::<Map>();
    let player_idx = map.xy_idx(player_pos.x, player_pos.y);
    if map.tiles[player_idx] == TileType::DownStairs {
        true
    } else {
        let mut gamelog = ecs.fetch_mut::<GameLog>();
        gamelog.entries.push("There is no way down from here.".to_string());
        false
    }
}

fn get_item(ecs: &mut World) {
    let player_pos = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();
    let mut gamelog = ecs.fetch_mut::<GameLog>();

    let mut target_item: Option<Entity> = None;
    for (item_entity, _item, position) in (&entities, &items, &positions).join() {
        if position.x == player_pos.x && position.y == player_pos.y {
            target_item = Some(item_entity);
        }
    }

    match target_item {
        None => gamelog.entries.push("There is nothing here to pick up.".to_string()),
        Some(item) => {
            let mut pickup = ecs.write_storage::<WantsToPickupItem>();
            pickup
                .insert(
                    *player_entity,
                    WantsToPickupItem {
                        collected_by: *player_entity,
                        item,
                    },
                )
                .expect("Unable to insert want to pickup");
        },
    }
}

fn skip_turn(ecs: &mut World) -> RunState {
    let player_entity = ecs.fetch::<Entity>();
    let viewshed_components = ecs.read_storage::<Viewshed>();
    let monsters = ecs.read_storage::<Monster>();

    let worldmap_resource = ecs.fetch::<Map>();

    let mut can_heal = true;
    let viewshed = viewshed_components.get(*player_entity).unwrap();
    for tile in viewshed.visible_tiles.iter() {
        let idx = worldmap_resource.xy_idx(tile.x, tile.y);
        for entity_id in worldmap_resource.tile_content[idx].iter() {
            let mob = monsters.get(*entity_id);
            match mob {
                None => {},
                Some(_) => {
                    can_heal = false;
                },
            }
        }
    }

    let hunger_clocks = ecs.read_storage::<HungerClock>();
    let hc = hunger_clocks.get(*player_entity);
    if let Some(hc) = hc {
        match hc.state {
            HungerState::Hungry => can_heal = false,
            HungerState::Starving => can_heal = false,
            _ => {},
        }
    }

    if can_heal {
        let mut health_components = ecs.write_storage::<Pools>();
        let pools = health_components.get_mut(*player_entity).unwrap();
        pools.hit_points.current = i32::min(pools.hit_points.current + 1, pools.hit_points.max);
    }

    RunState::PlayerTurn
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Hotkeys
    if ctx.shift && ctx.key.is_some() {
        let key: Option<i32> = match ctx.key.unwrap() {
            VirtualKeyCode::Key1 => Some(1),
            VirtualKeyCode::Key2 => Some(2),
            VirtualKeyCode::Key3 => Some(3),
            VirtualKeyCode::Key4 => Some(4),
            VirtualKeyCode::Key5 => Some(5),
            VirtualKeyCode::Key6 => Some(6),
            VirtualKeyCode::Key7 => Some(7),
            VirtualKeyCode::Key8 => Some(8),
            VirtualKeyCode::Key9 => Some(9),
            _ => None,
        };
        if let Some(key) = key {
            return use_consumable_hotkey(gs, key - 1);
        }
    }

    // Player movement
    match ctx.key {
        None => return RunState::AwaitingInput, // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),

            // Diagonals
            VirtualKeyCode::Numpad9 | VirtualKeyCode::U => try_move_player(1, -1, &mut gs.ecs),
            VirtualKeyCode::Numpad7 | VirtualKeyCode::Y => try_move_player(-1, -1, &mut gs.ecs),
            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => try_move_player(1, 1, &mut gs.ecs),
            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),

            // Skip Turn
            VirtualKeyCode::Numpad5 | VirtualKeyCode::Space => return skip_turn(&mut gs.ecs),

            // Level changes
            VirtualKeyCode::Period => {
                if try_next_level(&mut gs.ecs) {
                    return RunState::NextLevel;
                }
            },

            // Picking up items
            VirtualKeyCode::G => get_item(&mut gs.ecs),
            VirtualKeyCode::I => return RunState::ShowInventory,
            VirtualKeyCode::D => return RunState::ShowDropItem,
            VirtualKeyCode::R => return RunState::ShowRemoveItem,

            // Save and Quit
            VirtualKeyCode::Escape => return RunState::SaveGame,

            _ => return RunState::AwaitingInput,
        },
    }
    RunState::PlayerTurn
}

fn use_consumable_hotkey(gs: &mut State, key: i32) -> RunState {
    use super::{Consumable, InBackpack, WantsToUseItem};

    let consumables = gs.ecs.read_storage::<Consumable>();
    let backpack = gs.ecs.read_storage::<InBackpack>();
    let player_entity = gs.ecs.fetch::<Entity>();
    let entities = gs.ecs.entities();
    let mut carried_consumables = Vec::new();

    for (entity, carried_by, _consumable) in (&entities, &backpack, &consumables).join() {
        if carried_by.owner == *player_entity {
            carried_consumables.push(entity);
        }
    }

    if (key as usize) < carried_consumables.len() {
        use crate::components::Ranged;

        if let Some(ranged) = gs.ecs.read_storage::<Ranged>().get(carried_consumables[key as usize]) {
            return RunState::ShowTargeting {
                range: ranged.range,
                item: carried_consumables[key as usize],
            };
        }

        let mut intent = gs.ecs.write_storage::<WantsToUseItem>();
        intent
            .insert(
                *player_entity,
                WantsToUseItem {
                    item: carried_consumables[key as usize],
                    target: None,
                },
            )
            .expect("Unable to insert intent");

        return RunState::PlayerTurn;
    }

    RunState::PlayerTurn
}
