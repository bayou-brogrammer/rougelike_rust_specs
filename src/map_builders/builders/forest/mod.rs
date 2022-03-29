use super::{
    AreaStartingPosition,
    BuilderChain,
    CellularAutomataBuilder,
    CullUnreachable,
    VoronoiSpawning,
    XStart,
    YStart,
};

mod yellow_brick_road;
use yellow_brick_road::YellowBrickRoad;

pub fn forest_builder(new_depth: i32, _rng: &mut rltk::RandomNumberGenerator, width: i32, height: i32) -> BuilderChain {
    let mut chain = BuilderChain::new(new_depth, width, height, "Into the Woods");

    chain.start_with(CellularAutomataBuilder::new());
    chain.with(AreaStartingPosition::new(XStart::Center, YStart::Center));
    chain.with(CullUnreachable::new());
    chain.with(AreaStartingPosition::new(XStart::Left, YStart::Center));
    chain.with(VoronoiSpawning::new());
    chain.with(YellowBrickRoad::new());

    chain
}
