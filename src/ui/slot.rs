use bevy::{ecs::system::SystemState, prelude::*};

use super::{item::SpawnItemUi, nearest_sampler, SLOT_SIZE};
pub struct SpawnSlotUi {
    pub parent_ui: Entity,
    pub offset: Vec2,
    pub slot: Option<Entity>,
    pub inventory: Entity,
    pub index: usize,
}

#[derive(Component)]
pub struct SlotUi {
    pub slot: Option<Entity>,
    pub inventory: Entity,
    pub index: usize,
}

impl Command for SpawnSlotUi {
    fn apply(self, world: &mut World) {
        let mut state = SystemState::<(Commands, Res<AssetServer>)>::new(world);
        let (mut commands, asset_server) = state.get(world);
        let root = commands
            .spawn((
                SlotUi {
                    slot: self.slot,
                    inventory: self.inventory,
                    index: self.index,
                },
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
            .set_parent(self.parent_ui)
            .id();

        if let Some(item) = self.slot {
            commands.queue(SpawnItemUi {
                parent_ui: root,
                item,
            });
        };
        state.apply(world);
    }
}
