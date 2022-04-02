use crate::map_builders::*;

mod bsp_dungeon;
mod bsp_interior;
mod cellular_automata;
mod dla;
mod drunkard;
mod forest;
mod limestone_cavern;
mod maze;
mod simple_map;
mod voronoi;
mod waveform_collapse;

pub mod prefab_builder;
pub mod town;

pub use bsp_dungeon::BspDungeonBuilder;
pub use bsp_interior::BspInteriorBuilder;
pub use cellular_automata::CellularAutomataBuilder;
pub use dla::DLABuilder;
pub use drunkard::DrunkardsWalkBuilder;
pub use maze::MazeBuilder;
pub use prefab_builder::PrefabBuilder;
pub use simple_map::SimpleMapBuilder;
pub use voronoi::VoronoiCellBuilder;
pub use waveform_collapse::WaveformCollapseBuilder;

pub use forest::*;
pub use limestone_cavern::*;
pub use town::*;
