use core::panic;

use crate::prelude::{parse_dice_string as rlk_parse_dice_string, *};

pub fn parse_dice_string(dice: &str) -> (i32, i32, i32) {
    let DiceType {
        n_dice,
        die_type,
        bonus,
    } = match rlk_parse_dice_string(dice) {
        Ok(dt) => dt,
        Err(e) => panic!("Error parsing dice string: {}", e),
    };

    (n_dice, die_type, bonus)
}

pub fn parse_particle_line(n: &str) -> SpawnParticleLine {
    let tokens: Vec<_> = n.split(';').collect();

    SpawnParticleLine {
        glyph: rltk::to_cp437(tokens[0].chars().next().unwrap()),
        color: rltk::RGB::from_hex(tokens[1]).expect("Bad RGB"),
        lifetime_ms: tokens[2].parse::<f32>().unwrap(),
    }
}

pub fn parse_particle(n: &str) -> SpawnParticleBurst {
    let tokens: Vec<_> = n.split(';').collect();

    SpawnParticleBurst {
        glyph: rltk::to_cp437(tokens[0].chars().next().unwrap()),
        color: rltk::RGB::from_hex(tokens[1]).expect("Bad RGB"),
        lifetime_ms: tokens[2].parse::<f32>().unwrap(),
    }
}
