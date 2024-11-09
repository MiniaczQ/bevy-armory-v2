pub mod change_propagation;
pub mod components;
pub mod params;
pub mod ui;

use bevy::prelude::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((change_propagation::plugin, ui::plugin));
    }
}

pub mod prelude {
    pub use super::{
        change_propagation::{InventoryChanged, ItemChanged, SlotChanged},
        components::{Count, Extends, Icon, Inventory, Item},
        params::ItemData,
        ui::prelude::*,
        ItemPlugin,
    };
}
