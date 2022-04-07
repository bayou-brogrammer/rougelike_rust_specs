use specs::prelude::*;

use super::State;
use crate::components::*;

#[derive(PartialEq, Copy, Clone)]
pub enum VendorMode {
    Buy,
    Sell,
}

///////////////////////////////////////////////////////////////////////////
// Vendor Helper Functions
///////////////////////////////////////////////////////////////////////////
impl State {
    pub fn sell_items(&mut self, item: Option<Entity>) {
        let price = self.ecs.read_storage::<Item>().get(item.unwrap()).unwrap().base_value * 0.8;

        self.ecs
            .write_storage::<Pools>()
            .get_mut(*self.ecs.fetch::<Entity>())
            .unwrap()
            .gold += price;

        self.ecs.delete_entity(item.unwrap()).expect("Unable to delete");
    }

    pub fn buy_items(&mut self, tag: Option<String>, price: Option<f32>) {
        use crate::raws::*;

        let tag = tag.unwrap();
        let price = price.unwrap();
        let mut pools = self.ecs.write_storage::<Pools>();
        let player_pools = pools.get_mut(*self.ecs.fetch::<Entity>()).unwrap();

        if player_pools.gold >= price {
            player_pools.gold -= price;
            std::mem::drop(pools);

            let player_entity = *self.ecs.fetch::<Entity>();
            crate::raws::spawn_named_item(
                &RAWS.lock().unwrap(),
                &mut self.ecs,
                &tag,
                SpawnType::Carried { by: player_entity },
            );
        }
    }
}
