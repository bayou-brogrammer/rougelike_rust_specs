use std::collections::HashMap;

use specs::prelude::*;

use super::{Name, Position, SpawnType};
use crate::raws::structs::*;

pub fn spawn_position(pos: SpawnType, new_entity: EntityBuilder) -> EntityBuilder {
    let mut eb = new_entity;

    // Spawn in the specified location
    match pos {
        SpawnType::AtPosition { x, y } => {
            eb = eb.with(Position { x, y });
        },
    }

    eb
}

pub fn get_renderable_component(renderable: &item_structs::Renderable) -> crate::components::Renderable {
    crate::components::Renderable {
        glyph: rltk::to_cp437(renderable.glyph.chars().next().unwrap()),
        fg: rltk::RGB::from_hex(&renderable.fg).expect("Invalid RGB"),
        bg: rltk::RGB::from_hex(&renderable.bg).expect("Invalid RGB"),
        render_order: renderable.order,
    }
}

pub fn build_base_entity<'a, T: BaseRawComponent + Clone>(
    new_entity: EntityBuilder<'a>,
    entity_list: &[T],
    indexes: &HashMap<String, usize>,
    key: &str,
    pos: SpawnType,
) -> (EntityBuilder<'a>, T) {
    let entity_template = &entity_list[indexes[key]];
    let mut eb = new_entity;

    // Spawn in the specified location
    eb = spawn_position(pos, eb);

    // Renderable
    if let Some(renderable) = &entity_template.renderable() {
        eb = eb.with(get_renderable_component(renderable));
    }

    // // Name Component
    eb = eb.with(Name {
        name: entity_template.name(),
    });

    (eb, entity_template.clone())
}
