use bevy::{ecs::system::SystemState, prelude::*};

use crate::components::Inventory;

use super::{slot::SpawnSlotUi, SLOT_SIZE};

#[derive(Component)]
pub struct InventoryUi(pub Entity);

pub struct SpawnInventoryUi {
    pub parent_ui: Entity,
    pub offset: Vec2,
    pub inventory: Entity,
}

impl Command for SpawnInventoryUi {
    fn apply(self, world: &mut World) {
        let mut state = SystemState::<(Commands, Query<&Inventory>)>::new(world);
        let (mut commands, inventories) = state.get(world);
        let inventory = inventories.get(self.inventory).unwrap();
        let size = inventory.0.len();
        let root = commands
            .spawn((
                InventoryUi(self.inventory),
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
                parent_ui: root,
                offset: Vec2::new(0.0, y),
                slot: inventory.0[i],
                inventory: self.inventory,
                index: i,
            });
        }
        state.apply(world);
    }
}
