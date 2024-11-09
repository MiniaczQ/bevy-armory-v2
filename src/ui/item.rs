use bevy::{ecs::system::SystemState, prelude::*};

use crate::{
    components::{Count, Icon},
    params::ItemData,
};

use super::{ITEM_SIZE, SLOT_SIZE};

#[derive(Component)]
pub struct ItemUi(pub Entity);

pub struct SpawnItemUi {
    pub parent_ui: Entity,
    pub item: Entity,
}

impl Command for SpawnItemUi {
    fn apply(self, world: &mut World) {
        let mut state = SystemState::<(Commands, ItemData<&Icon>, ItemData<&Count>)>::new(world);
        let (mut commands, icons, counts) = state.get(world);
        let icon = icons.extended_get(self.item).unwrap().unwrap();
        let item = commands
            .spawn((
                ItemUi(self.item),
                UiImage::new(icon.0.clone()),
                Node {
                    width: Val::Px(ITEM_SIZE),
                    height: Val::Px(ITEM_SIZE),
                    margin: UiRect::all(Val::Px((SLOT_SIZE - ITEM_SIZE) / 2.0)),
                    ..default()
                },
            ))
            .set_parent(self.parent_ui)
            .id();
        if let Ok(Some(count)) = counts.get(self.item) {
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
