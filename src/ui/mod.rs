use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};

pub mod carry;
pub mod change_propagation;
pub mod inventory;
pub mod item;
pub mod layout;
pub mod slot;
pub mod tooltip;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        tooltip::plugin,
        carry::plugin,
        change_propagation::plugin,
        layout::plugin,
    ));
}

pub const ITEM_SIZE: f32 = 16.0 * 4.0;
pub const SLOT_SIZE: f32 = 24.0 * 4.0;

pub fn nearest_sampler(settings: &mut ImageLoaderSettings) {
    settings.sampler = ImageSampler::nearest()
}

pub mod prelude {
    pub use super::{
        carry::Carry,
        change_propagation::{InventoryChanged, ItemChanged, SlotChanged},
        inventory::{InventoryUi, SpawnInventoryUi},
        item::{ItemUi, SpawnItemUi},
        nearest_sampler,
        slot::{SlotUi, SpawnSlotUi},
        tooltip::Tooltip,
        ITEM_SIZE, SLOT_SIZE,
    };
}
