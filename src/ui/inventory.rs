use bevy::{ecs::system::SystemState, prelude::*};

use crate::components::Inventory;

use super::{
    slot::{SlotUi, SpawnSlotUi},
    SLOT_SIZE,
};

/// UI element representing the underlying inventory.
#[derive(Component)]
pub struct InventoryUi {
    /// Inventory this is representing.
    pub data: Entity,
}

/// Command for spawning inventory UI entity.
pub struct SpawnInventoryUi {
    /// UI entity this will belong to.
    pub parent: Entity,
    /// Absolute offset in relation to parent entity.
    pub offset: Vec2,
    /// Inventory data.
    pub inventory: InventoryUi,
}

impl Command for SpawnInventoryUi {
    fn apply(self, world: &mut World) {
        let mut state = SystemState::<(Commands, Query<&Inventory>)>::new(world);
        let (mut commands, inventories) = state.get(world);
        let data = self.inventory.data;
        let inventory = inventories.get(data).unwrap();
        let size = inventory.0.len();
        let root = commands
            .spawn((
                self.inventory,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(self.offset.x),
                    top: Val::Px(self.offset.y),
                    ..default()
                },
            ))
            .id();
        for i in 0..size {
            let y = SLOT_SIZE * i as f32;
            commands.queue(SpawnSlotUi {
                parent: root,
                offset: Vec2::new(0.0, y),
                slot: SlotUi {
                    data: inventory.0[i],
                    inventory: data,
                    index: i,
                },
            });
        }
        state.apply(world);
    }
}
