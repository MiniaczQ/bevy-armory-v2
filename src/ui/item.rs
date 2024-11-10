//! UI representation of items.

use bevy::{ecs::system::SystemState, prelude::*};

use crate::{
    components::{Count, Icon},
    params::ItemData,
};

use super::ITEM_SIZE;

/// UI element representing an item.
#[derive(Component)]
pub struct ItemUi {
    /// Item this is representing.
    pub item: Entity,
}

/// Command for spawning item UI entity.
pub struct SpawnItemUi {
    /// UI entity this will belong to.
    pub parent: Entity,
    /// Item data.
    pub item: Entity,
}

impl Command for SpawnItemUi {
    fn apply(self, world: &mut World) {
        spawn_item(world, self.item)
            .insert(ItemUi { item: self.item })
            .set_parent(self.parent);
    }
}

pub fn spawn_item(world: &mut World, item: Entity) -> EntityWorldMut {
    let mut state = SystemState::<(Commands, ItemData<&Icon>, ItemData<&Count>)>::new(world);
    let (mut commands, icons, counts) = state.get(world);
    let icon = icons.extended_get(item).unwrap().unwrap();
    let item_ui = commands
        .spawn((
            UiImage::new(icon.0.clone()),
            Node {
                width: Val::Px(ITEM_SIZE),
                height: Val::Px(ITEM_SIZE),
                ..default()
            },
        ))
        .id();
    if let Ok(Some(count)) = counts.get(item) {
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
            .set_parent(item_ui);
    }
    state.apply(world);
    world.entity_mut(item_ui)
}
