use bevy::{
    ecs::{
        query::{QueryData, QueryEntityError, WorldQuery},
        system::{SystemParam, SystemState},
    },
    prelude::*,
};

/// Marker component for items.
#[derive(Component)]
pub struct Item;

/// Image handle for this item.
#[derive(Component)]
pub struct Icon(pub Handle<Image>);

/// Derive item from another item.
/// If component lookup fails, the other item components will be looked up instead.
#[derive(Component)]
pub struct Extends(pub Entity);

/// Count signified how many items are stored in a slot.
#[derive(Component)]
pub struct Count(pub u32);

/// System parameter for accessing items.
#[derive(SystemParam)]
pub struct Items<'w, 's, D: QueryData + 'static> {
    items: Query<'w, 's, (Option<&'static Extends>, Option<D>), With<Item>>,
}

impl<'w, 's, D: QueryData> Items<'w, 's, D> {
    fn extend_find(&self, mut entity: Entity) -> Result<Option<Entity>, QueryEntityError> {
        loop {
            let (maybe_extends, maybe_data) = self.items.get(entity)?;
            if maybe_data.is_some() {
                break Ok(Some(entity));
            }
            let Some(&Extends(new_entity)) = maybe_extends else {
                break Ok(None);
            };
            entity = new_entity;
        }
    }

    pub fn extended_get(
        &self,
        entity: Entity,
    ) -> Result<Option<<D::ReadOnly as WorldQuery>::Item<'_>>, QueryEntityError> {
        let Some(entity) = self.extend_find(entity)? else {
            return Ok(None);
        };
        Ok(self.items.get(entity).unwrap().1)
    }

    pub fn get(
        &self,
        entity: Entity,
    ) -> Result<Option<<D::ReadOnly as WorldQuery>::Item<'_>>, QueryEntityError> {
        Ok(self.items.get(entity)?.1)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Result<Option<D::Item<'_>>, QueryEntityError> {
        Ok(self.items.get_mut(entity)?.1)
    }
}

/// Constant size container of items.
pub struct Inventory(pub Box<[Option<Entity>]>);

impl Inventory {
    pub fn new<const N: usize>() -> Self {
        Self(Box::new(core::array::from_fn::<_, N, _>(|_| None)))
    }
}

#[derive(Component)]
pub struct ItemUi(pub Entity);

pub struct SpawnItemUi {
    pub parent_ui: Entity,
    pub item: Entity,
}

impl Command for SpawnItemUi {
    fn apply(self, world: &mut World) {
        let mut state = SystemState::<(Commands, Items<&Icon>)>::new(world);
        let (mut commands, items) = state.get(world);
        let image = items.extended_get(self.item).unwrap().unwrap();
        commands
            .spawn((
                ItemUi(self.item),
                UiImage::new(image.0.clone()),
                Node {
                    width: Val::Px(64.0),
                    height: Val::Px(64.0),
                    ..default()
                },
                Button,
                BufferedInteraction(Interaction::None),
            ))
            .set_parent(self.parent_ui);
        state.apply(world);
    }
}

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, item_ui_hover);
        app.add_systems(Last, buffered_interaction);
    }
}

#[derive(Component)]
struct HasTooltip(Entity);

#[derive(Component)]
struct Tooltip;

fn item_ui_hover(
    mut commands: Commands,
    ui: Query<(
        Entity,
        &ItemUi,
        &BufferedInteraction,
        &Interaction,
        Option<&mut HasTooltip>,
    )>,
    items: Items<&Name>,
) {
    for (entity, item, buffered_interaction, interaction, tooltip) in ui.iter() {
        if buffered_interaction.0 != Interaction::Hovered && *interaction == Interaction::Hovered {
            let item = items.extended_get(item.0).unwrap().unwrap();
            let tooltip = commands
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(100.0),
                        top: Val::Percent(100.0),
                        ..default()
                    },
                    Text::new(item.as_str()),
                    Tooltip,
                ))
                .set_parent(entity)
                .id();
            commands.entity(entity).insert(HasTooltip(tooltip));
        } else if buffered_interaction.0 == Interaction::Hovered
            && *interaction != Interaction::Hovered
        {
            if let Some(tooltip) = tooltip {
                commands.entity(tooltip.0).despawn_recursive();
                commands.entity(entity).remove::<HasTooltip>();
            }
        }
    }
}

#[derive(Component)]
struct BufferedInteraction(Interaction);

fn buffered_interaction(mut query: Query<(&Interaction, &mut BufferedInteraction)>) {
    for (interaction, mut buffered_interaction) in query.iter_mut() {
        if buffered_interaction.0 != *interaction {
            buffered_interaction.0 = *interaction;
        }
    }
}
