use serde::{Deserialize, Serialize};

mod logstore;
use logstore::*;
pub use logstore::{clear_log, clone_log, print_log, restore_log};

mod builder;
pub use builder::*;

mod events;
pub use events::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct LogFragment {
    pub color: rltk::RGB,
    pub text: String,
}
