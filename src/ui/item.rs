//! UI representation of items.

use bevy::{ecs::system::SystemState, prelude::*};

use crate::{
    components::{Count, Icon},
    params::ItemData,
};

use super::{ITEM_SIZE, SLOT_SIZE};

/// UI element representing an item.
#[derive(Component)]
pub struct ItemUi {
    /// Item this is representing.
    pub data: Entity,
}

/// Command for spawning item UI entity.
pub struct SpawnItemUi {
    /// UI entity this will belong to.
    pub parent: Entity,
    /// Item data.
    pub item: ItemUi,
}

impl Command for SpawnItemUi {
    fn apply(self, world: &mut World) {
        let mut state = SystemState::<(Commands, ItemData<&Icon>, ItemData<&Count>)>::new(world);
        let (mut commands, icons, counts) = state.get(world);
        let data = self.item.data;
        let icon = icons.extended_get(data).unwrap().unwrap();
        let item = commands
            .spawn((
                self.item,
                UiImage::new(icon.0.clone()),
                Node {
                    width: Val::Px(ITEM_SIZE),
                    height: Val::Px(ITEM_SIZE),
                    margin: UiRect::all(Val::Px((SLOT_SIZE - ITEM_SIZE) / 2.0)),
                    ..default()
                },
            ))
            .set_parent(self.parent)
            .id();
        if let Ok(Some(count)) = counts.get(data) {
            commands
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        bottom: Val::Percent(0.0),
                        right: Val::Percent(0.0),
                        ..default()
                    },
                    Text::new(format!("{}", count.0)),
                    PickingBehavior::IGNORE,
                ))
                .set_parent(item);
        }
        state.apply(world);
    }
}
