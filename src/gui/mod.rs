use specs::prelude::*;
use std::cmp::Ordering;

use super::{
    camera,
    components::*,
    gamelog::GameLog,
    rex_assets::RexAssets,
    Map,
    MasterDungeonMap,
    RunState,
    State,
    VendorMode,
};
use rltk::{Point, Rltk, VirtualKeyCode, RGB};

mod tooltip;
use tooltip::*;

mod menu;
pub use menu::*;

pub fn draw_hollow_box(console: &mut Rltk, sx: i32, sy: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
    use rltk::to_cp437;

    console.set(sx, sy, fg, bg, to_cp437('┌'));
    console.set(sx + width, sy, fg, bg, to_cp437('┐'));
    console.set(sx, sy + height, fg, bg, to_cp437('└'));
    console.set(sx + width, sy + height, fg, bg, to_cp437('┘'));

    for x in sx + 1..sx + width {
        console.set(x, sy, fg, bg, to_cp437('─'));
        console.set(x, sy + height, fg, bg, to_cp437('─'));
    }

    for y in sy + 1..sy + height {
        console.set(sx, y, fg, bg, to_cp437('│'));
        console.set(sx + width, y, fg, bg, to_cp437('│'));
    }
}

fn draw_attribute(name: &str, attribute: &Attribute, y: i32, ctx: &mut Rltk) {
    let black = RGB::named(rltk::BLACK);
    let attr_gray: RGB = RGB::from_hex("#CCCCCC").expect("Oops");

    ctx.print_color(50, y, attr_gray, black, name);

    let color: RGB = match attribute.modifiers.cmp(&0) {
        Ordering::Less => RGB::from_f32(1.0, 0.0, 0.0),
        Ordering::Equal => RGB::named(rltk::WHITE),
        Ordering::Greater => RGB::from_f32(0.0, 1.0, 0.0),
    };

    ctx.print_color(
        67,
        y,
        color,
        black,
        &format!("{}", attribute.base + attribute.modifiers),
    );

    ctx.print_color(73, y, color, black, &format!("{}", attribute.bonus));

    if attribute.bonus > 0 {
        ctx.set(72, y, color, black, rltk::to_cp437('+'));
    }
}

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    use rltk::to_cp437;
    let box_gray: RGB = RGB::from_hex("#999999").expect("Oops");
    let black = RGB::named(rltk::BLACK);
    let white = RGB::named(rltk::WHITE);

    draw_hollow_box(ctx, 0, 0, 79, 59, box_gray, black); // Overall box
    draw_hollow_box(ctx, 0, 0, 49, 45, box_gray, black); // Map box
    draw_hollow_box(ctx, 0, 45, 79, 14, box_gray, black); // Log box
    draw_hollow_box(ctx, 49, 0, 30, 8, box_gray, black); // Top-right panel

    // Draw box connectors
    ctx.set(0, 45, box_gray, black, to_cp437('├'));
    ctx.set(49, 8, box_gray, black, to_cp437('├'));
    ctx.set(49, 0, box_gray, black, to_cp437('┬'));
    ctx.set(49, 45, box_gray, black, to_cp437('┴'));
    ctx.set(79, 8, box_gray, black, to_cp437('┤'));
    ctx.set(79, 45, box_gray, black, to_cp437('┤'));

    // Draw the town name
    let map = ecs.fetch::<Map>();
    let name_length = map.name.len() + 2;
    let x_pos = (22 - (name_length / 2)) as i32;
    ctx.set(x_pos, 0, box_gray, black, to_cp437('┤'));
    ctx.set(x_pos + name_length as i32 - 1, 0, box_gray, black, to_cp437('├'));
    ctx.print_color(x_pos + 1, 0, white, black, &map.name);
    std::mem::drop(map);

    // Draw stats
    let player_entity = ecs.fetch::<Entity>();
    let pools = ecs.read_storage::<Pools>();
    let player_pools = pools.get(*player_entity).unwrap();
    let health = format!(
        "Health: {}/{}",
        player_pools.hit_points.current, player_pools.hit_points.max
    );
    let mana = format!("Mana:   {}/{}", player_pools.mana.current, player_pools.mana.max);
    let xp = format!("Level:  {}", player_pools.level);

    ctx.print_color(50, 1, white, black, &health);
    ctx.print_color(50, 2, white, black, &mana);
    ctx.print_color(50, 3, white, black, &xp);

    ctx.draw_bar_horizontal(
        64,
        1,
        14,
        player_pools.hit_points.current,
        player_pools.hit_points.max,
        RGB::named(rltk::RED),
        RGB::named(rltk::BLACK),
    );
    ctx.draw_bar_horizontal(
        64,
        2,
        14,
        player_pools.mana.current,
        player_pools.mana.max,
        RGB::named(rltk::BLUE),
        RGB::named(rltk::BLACK),
    );

    let xp_level_start = (player_pools.level - 1) * 1000;
    ctx.draw_bar_horizontal(
        64,
        3,
        14,
        player_pools.xp - xp_level_start,
        1000,
        RGB::named(rltk::GOLD),
        RGB::named(rltk::BLACK),
    );

    // Attributes
    let attributes = ecs.read_storage::<Attributes>();
    let attr = attributes.get(*player_entity).unwrap();
    draw_attribute("Might:", &attr.might, 4, ctx);
    draw_attribute("Quickness:", &attr.quickness, 5, ctx);
    draw_attribute("Fitness:", &attr.fitness, 6, ctx);
    draw_attribute("Intelligence:", &attr.intelligence, 7, ctx);

    // Initiative and weight
    ctx.print_color(
        50,
        9,
        white,
        black,
        &format!(
            "{:.0} lbs ({} lbs max)",
            player_pools.total_weight,
            (attr.might.base + attr.might.modifiers) * 15
        ),
    );
    ctx.print_color(
        50,
        10,
        white,
        black,
        &format!("Initiative Penalty: {:.0}", player_pools.total_initiative_penalty),
    );
    ctx.print_color(
        50,
        11,
        rltk::RGB::named(rltk::GOLD),
        black,
        &format!("Gold: {:.1}", player_pools.gold),
    );

    // Equipped
    let mut y = 13;
    let entities = ecs.entities();
    let equipped = ecs.read_storage::<Equipped>();
    for (entity, equipped_by) in (&entities, &equipped).join() {
        if equipped_by.owner == *player_entity {
            ctx.print_color(
                50,
                y,
                get_item_color(ecs, entity),
                black,
                &get_item_display_name(ecs, entity),
            );
            y += 1;
        }
    }

    // Consumables
    y += 1;
    let yellow = RGB::named(rltk::YELLOW);
    let consumables = ecs.read_storage::<Consumable>();
    let backpack = ecs.read_storage::<InBackpack>();
    let mut index = 1;
    for (entity, carried_by, _consumable) in (&entities, &backpack, &consumables).join() {
        if carried_by.owner == *player_entity && index < 10 {
            ctx.print_color(50, y, yellow, black, &format!("↑{}", index));
            ctx.print_color(
                53,
                y,
                get_item_color(ecs, entity),
                black,
                &get_item_display_name(ecs, entity),
            );
            y += 1;
            index += 1;
        }
    }

    // Status
    let hunger = ecs.read_storage::<HungerClock>();
    let hc = hunger.get(*player_entity).unwrap();
    match hc.state {
        HungerState::WellFed => ctx.print_color(50, 44, RGB::named(rltk::GREEN), RGB::named(rltk::BLACK), "Well Fed"),
        HungerState::Normal => {},
        HungerState::Hungry => ctx.print_color(50, 44, RGB::named(rltk::ORANGE), RGB::named(rltk::BLACK), "Hungry"),
        HungerState::Starving => ctx.print_color(50, 44, RGB::named(rltk::RED), RGB::named(rltk::BLACK), "Starving"),
    }

    // Draw the log
    let log = ecs.fetch::<GameLog>();
    let mut y = 46;
    for s in log.entries.iter().rev() {
        if y < 59 {
            ctx.print(2, y, s);
        }
        y += 1;
    }

    draw_tooltips(ecs, ctx);
}

pub fn ranged_target(gs: &mut State, ctx: &mut Rltk, range: i32) -> (ItemMenuResult, Option<Point>) {
    let (min_x, max_x, min_y, max_y) = camera::get_screen_bounds(&gs.ecs, ctx);
    let player_entity = gs.ecs.fetch::<Entity>();
    let player_pos = gs.ecs.fetch::<Point>();
    let viewsheds = gs.ecs.read_storage::<Viewshed>();

    ctx.print_color(
        5,
        0,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "Select Target:",
    );

    // Highlight available target cells
    let mut available_cells = Vec::new();
    let visible = viewsheds.get(*player_entity);
    if let Some(visible) = visible {
        // We have a viewshed
        for idx in visible.visible_tiles.iter() {
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(*player_pos, *idx);
            if distance <= range as f32 {
                let screen_x = idx.x - min_x;
                let screen_y = idx.y - min_y;
                if screen_x > 1 && screen_x < (max_x - min_x) - 1 && screen_y > 1 && screen_y < (max_y - min_y) - 1 {
                    ctx.set_bg(screen_x, screen_y, RGB::named(rltk::BLUE));
                    available_cells.push(idx);
                }
            }
        }
    } else {
        return (ItemMenuResult::Cancel, None);
    }

    // Draw mouse cursor
    let mouse_pos = ctx.mouse_pos();
    let mut mouse_map_pos = mouse_pos;

    mouse_map_pos.0 += min_x - 1;
    mouse_map_pos.1 += min_y - 1;

    let mut valid_target = false;
    for idx in available_cells.iter() {
        if idx.x == mouse_map_pos.0 && idx.y == mouse_map_pos.1 {
            valid_target = true;
        }
    }

    if valid_target {
        ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::CYAN));

        if ctx.left_click {
            return (
                ItemMenuResult::Selected,
                Some(Point::new(mouse_map_pos.0, mouse_map_pos.1)),
            );
        }
    } else {
        ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::RED));
        if ctx.left_click {
            return (ItemMenuResult::Cancel, None);
        }
    }

    (ItemMenuResult::NoResponse, None)
}

#[derive(PartialEq, Copy, Clone)]
pub enum GameOverResult {
    NoSelection,
    QuitToMenu,
}

pub fn game_over(ctx: &mut Rltk) -> GameOverResult {
    ctx.print_color_centered(
        15,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "Your journey has ended!",
    );
    ctx.print_color_centered(
        17,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "One day, we'll tell you all about how you did.",
    );
    ctx.print_color_centered(
        18,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "That day, sadly, is not in this chapter..",
    );

    ctx.print_color_centered(
        20,
        RGB::named(rltk::MAGENTA),
        RGB::named(rltk::BLACK),
        "Press any key to return to the menu.",
    );

    match ctx.key {
        None => GameOverResult::NoSelection,
        Some(_) => GameOverResult::QuitToMenu,
    }
}
