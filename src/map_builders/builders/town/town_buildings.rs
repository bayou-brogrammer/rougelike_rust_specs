use super::{BuilderMap, Position, TileType, TownBuilder};

impl TownBuilder {
    pub fn random_building_spawn(
        &mut self,
        building: &(i32, i32, i32, i32),
        build_data: &mut BuilderMap,
        rng: &mut rltk::RandomNumberGenerator,
        to_place: &mut Vec<&str>,
        player_idx: usize,
    ) {
        for y in building.1..building.1 + building.3 {
            for x in building.0..building.0 + building.2 {
                let idx = build_data.map.xy_idx(x, y);

                if build_data.map.tiles[idx] == TileType::WoodFloor
                    && idx != player_idx
                    && rng.roll_dice(1, 3) == 1
                    && !to_place.is_empty()
                {
                    let entity_tag = to_place[0];
                    to_place.remove(0);
                    build_data.spawn_list.push((idx, entity_tag.to_string()));
                }
            }
        }
    }

    pub fn build_pub(
        &mut self,
        building: &(i32, i32, i32, i32),
        build_data: &mut BuilderMap,
        rng: &mut rltk::RandomNumberGenerator,
    ) {
        // Place the player
        build_data.starting_position = Some(Position {
            x: building.0 + (building.2 / 2),
            y: building.1 + (building.3 / 2),
        });
        let player_idx = build_data
            .map
            .xy_idx(building.0 + (building.2 / 2), building.1 + (building.3 / 2));

        // Place other items
        let mut to_place: Vec<&str> = vec![
            "Barkeep",
            "Shady Salesman",
            "Patron",
            "Patron",
            "Keg",
            "Table",
            "Chair",
            "Table",
            "Chair",
        ];
        self.random_building_spawn(building, build_data, rng, &mut to_place, player_idx);
    }

    pub fn build_temple(
        &mut self,
        building: &(i32, i32, i32, i32),
        build_data: &mut BuilderMap,
        rng: &mut rltk::RandomNumberGenerator,
    ) {
        // Place items
        let mut to_place: Vec<&str> = vec![
            "Priest",
            "Parishioner",
            "Parishioner",
            "Chair",
            "Chair",
            "Candle",
            "Candle",
        ];
        self.random_building_spawn(building, build_data, rng, &mut to_place, 0);
    }

    pub fn build_smith(
        &mut self,
        building: &(i32, i32, i32, i32),
        build_data: &mut BuilderMap,
        rng: &mut rltk::RandomNumberGenerator,
    ) {
        // Place items
        let mut to_place: Vec<&str> = vec!["Blacksmith", "Anvil", "Water Trough", "Weapon Rack", "Armor Stand"];
        self.random_building_spawn(building, build_data, rng, &mut to_place, 0);
    }

    pub fn build_clothier(
        &mut self,
        building: &(i32, i32, i32, i32),
        build_data: &mut BuilderMap,
        rng: &mut rltk::RandomNumberGenerator,
    ) {
        // Place items
        let mut to_place: Vec<&str> = vec!["Clothier", "Cabinet", "Table", "Loom", "Hide Rack"];
        self.random_building_spawn(building, build_data, rng, &mut to_place, 0);
    }

    pub fn build_alchemist(
        &mut self,
        building: &(i32, i32, i32, i32),
        build_data: &mut BuilderMap,
        rng: &mut rltk::RandomNumberGenerator,
    ) {
        // Place items
        let mut to_place: Vec<&str> = vec!["Alchemist", "Chemistry Set", "Dead Thing", "Chair", "Table"];
        self.random_building_spawn(building, build_data, rng, &mut to_place, 0);
    }

    pub fn build_my_house(
        &mut self,
        building: &(i32, i32, i32, i32),
        build_data: &mut BuilderMap,
        rng: &mut rltk::RandomNumberGenerator,
    ) {
        // Place items
        let mut to_place: Vec<&str> = vec!["Mom", "Bed", "Cabinet", "Chair", "Table"];
        self.random_building_spawn(building, build_data, rng, &mut to_place, 0);
    }

    pub fn build_hovel(
        &mut self,
        building: &(i32, i32, i32, i32),
        build_data: &mut BuilderMap,
        rng: &mut rltk::RandomNumberGenerator,
    ) {
        // Place items
        let mut to_place: Vec<&str> = vec!["Peasant", "Bed", "Chair", "Table"];
        self.random_building_spawn(building, build_data, rng, &mut to_place, 0);
    }

    pub fn build_abandoned_house(
        &mut self,
        building: &(i32, i32, i32, i32),
        build_data: &mut BuilderMap,
        rng: &mut rltk::RandomNumberGenerator,
    ) {
        for y in building.1..building.1 + building.3 {
            for x in building.0..building.0 + building.2 {
                let idx = build_data.map.xy_idx(x, y);
                if build_data.map.tiles[idx] == TileType::WoodFloor && idx != 0 && rng.roll_dice(1, 2) == 1 {
                    build_data.spawn_list.push((idx, "Rat".to_string()));
                }
            }
        }
    }
}
