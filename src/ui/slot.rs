use bevy::{ecs::system::SystemState, prelude::*};

use super::{
    item::{ItemUi, SpawnItemUi},
    nearest_sampler, SLOT_SIZE,
};

/// UI element representing an item slot of an inventory.
#[derive(Component)]
pub struct SlotUi {
    /// Cached item data.
    pub data: Option<Entity>,
    /// Inventory this slot is representing.
    pub inventory: Entity,
    /// Index in the inventory this slot is representing.
    pub index: usize,
}

/// Command for spawning slot UI entity.
pub struct SpawnSlotUi {
    /// UI entity this will belong to.
    pub parent: Entity,
    /// Absolute offset in relation to parent entity.
    pub offset: Vec2,
    /// Item slot data.
    pub slot: SlotUi,
}

impl Command for SpawnSlotUi {
    fn apply(self, world: &mut World) {
        let mut state = SystemState::<(Commands, Res<AssetServer>)>::new(world);
        let (mut commands, asset_server) = state.get(world);
        let content = self.slot.data;
        let root = commands
            .spawn((
                self.slot,
                UiImage::new(asset_server.load_with_settings("item-slot.png", nearest_sampler)),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(self.offset.x),
                    top: Val::Px(self.offset.y),
                    width: Val::Px(SLOT_SIZE),
                    height: Val::Px(SLOT_SIZE),
                    ..default()
                },
            ))
            .set_parent(self.parent)
            .id();

        if let Some(item) = content {
            commands.queue(SpawnItemUi {
                parent: root,
                item: ItemUi { data: item },
            });
        };
        state.apply(world);
    }
}
