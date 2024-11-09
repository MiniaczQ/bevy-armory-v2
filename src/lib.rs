//! Highly configurable crate for items and inventories.

pub mod components;
pub mod params;
pub mod ui;

use bevy::prelude::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ui::plugin,));
    }
}

pub mod prelude {
    pub use super::{
        components::{Count, Icon, Inventory, Item, Template},
        params::ItemData,
        ui::prelude::*,
        ItemPlugin,
    };
}
