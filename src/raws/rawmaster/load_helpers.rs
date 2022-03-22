use std::collections::{HashMap, HashSet};

use super::BaseRawComponent;

pub fn load_entity_data<'a, T: 'static + BaseRawComponent>(
    raws: &[T],
    indexes: &mut HashMap<String, usize>,
    used_names: &mut HashSet<String>,
) {
    for (i, entity) in raws.iter().enumerate() {
        let entity_name = entity.name();

        if used_names.contains(&entity_name) {
            rltk::console::log(format!("WARNING - duplicate entity name in raws [{}]", entity_name));
        }

        indexes.insert(entity_name.clone(), i);
        used_names.insert(entity_name.clone());
    }
}
