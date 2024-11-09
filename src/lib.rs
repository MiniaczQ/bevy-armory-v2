use bevy::{
    ecs::{
        query::{QueryData, QueryEntityError, WorldQuery},
        system::{SystemParam, SystemState},
    },
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    ui::RelativeCursorPosition,
    window::PrimaryWindow,
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
#[derive(Component)]
pub struct Inventory(pub Box<[Option<Entity>]>);

impl Inventory {
    pub fn new<const N: usize>() -> Self {
        Self(Box::new(core::array::from_fn::<_, N, _>(|_| None)))
    }
}

#[derive(Component)]
pub struct InventoryUi(Entity);

pub struct SpawnInventoryUi {
    pub parent_ui: Entity,
    pub offset: Vec2,
    pub inventory: Entity,
}

pub const ITEM_SIZE: f32 = 16.0 * 4.0;
pub const SLOT_SIZE: f32 = 24.0 * 4.0;

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

#[derive(Component)]
pub struct SlotUi {
    pub slot: Option<Entity>,
    pub inventory: Entity,
    pub index: usize,
}

pub struct SpawnSlotUi {
    pub parent_ui: Entity,
    pub offset: Vec2,
    pub slot: Option<Entity>,
    pub inventory: Entity,
    pub index: usize,
}

pub fn nearest_sampler(settings: &mut ImageLoaderSettings) {
    settings.sampler = ImageSampler::nearest()
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

#[derive(Component)]
pub struct ItemUi(Entity);

pub struct SpawnItemUi {
    pub parent_ui: Entity,
    pub item: Entity,
}

impl Command for SpawnItemUi {
    fn apply(self, world: &mut World) {
        let mut state = SystemState::<(Commands, Items<&Icon>, Items<&Count>)>::new(world);
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
                RelativeCursorPosition::default(),
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

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(item_tooltip_spawn);
        app.add_observer(item_tooltip_follow);
        app.add_observer(item_tooltip_despawn);
        app.add_observer(pickup_item);
        app.add_observer(drop_item);
        app.add_observer(inventory_changed);
        app.add_observer(item_changed);
        app.add_observer(slot_changed);
        app.add_systems(Update, pickup_follow);
    }
}

#[derive(Component)]
struct Tooltip;

fn item_tooltip_spawn(
    trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    items: Query<(&RelativeCursorPosition, &ItemUi), Without<Pickup>>,
    names: Items<&Name>,
) {
    let Ok((cursor, item)) = items.get(trigger.entity()) else {
        return;
    };
    let item = names.extended_get(item.0).unwrap().unwrap();
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(cursor.normalized.unwrap_or_default().x * 100.0),
                top: Val::Percent(cursor.normalized.unwrap_or_default().y * 100.0),
                ..default()
            },
            Text::new(item.as_str()),
            Tooltip,
            PickingBehavior::IGNORE,
            GlobalZIndex(128),
            BackgroundColor(Color::Srgba(Srgba::new(0.3, 0.3, 0.3, 0.3))),
        ))
        .set_parent(trigger.entity());
}

fn item_tooltip_follow(
    trigger: Trigger<Pointer<Move>>,
    items: Query<(&RelativeCursorPosition, &Children), With<ItemUi>>,
    mut tooltips: Query<&mut Node, With<Tooltip>>,
) {
    let Ok((cursor, children)) = items.get(trigger.entity()) else {
        return;
    };
    for &child in children {
        let Ok(mut node) = tooltips.get_mut(child) else {
            continue;
        };
        node.left = Val::Percent(cursor.normalized.unwrap().x * 100.0);
        node.top = Val::Percent(cursor.normalized.unwrap().y * 100.0);
    }
}

fn item_tooltip_despawn(
    trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
    items: Query<&Children, With<ItemUi>>,
    tooltips: Query<(), With<Tooltip>>,
) {
    let Ok(children) = items.get(trigger.entity()) else {
        return;
    };
    for &child in children {
        let Ok(_) = tooltips.get(child) else {
            continue;
        };
        commands.entity(child).despawn_recursive();
    }
}

#[derive(Component)]
pub struct PickedUp;

#[derive(Component)]
pub struct Pickup;

fn pickup_item(
    trigger: Trigger<Pointer<Down>>,
    mut commands: Commands,
    mut items: Query<(&Children, &UiImage, &Node, &mut Visibility), With<ItemUi>>,
    tooltips: Query<(), With<Tooltip>>,
    pickup: Option<Single<(), With<Pickup>>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let Ok((children, image, node, mut visibility)) = items.get_mut(trigger.entity()) else {
        return;
    };
    if pickup.is_some() {
        return;
    }
    for &child in children {
        if let Ok(()) = tooltips.get(child) {
            commands.entity(child).despawn_recursive();
        }
    }
    commands.entity(trigger.entity()).insert(PickedUp);
    *visibility = Visibility::Hidden;

    let cursor = window.cursor_position().unwrap();
    commands.spawn((
        UiImage::new(image.image.clone()),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(cursor.x - ITEM_SIZE / 2.0),
            top: Val::Px(cursor.y - ITEM_SIZE / 2.0),
            height: node.height,
            width: node.width,
            ..default()
        },
        Pickup,
        PickingBehavior::IGNORE,
    ));
}

fn pickup_follow(
    window: Single<&Window, With<PrimaryWindow>>,
    mut pickup: Single<&mut Node, With<Pickup>>,
) {
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    pickup.left = Val::Px(cursor.x - ITEM_SIZE / 2.0);
    pickup.top = Val::Px(cursor.y - ITEM_SIZE / 2.0);
}

fn drop_item(
    trigger: Trigger<Pointer<Down>>,
    mut commands: Commands,
    slots: Query<&SlotUi>,
    mut item: Single<(Entity, &mut Visibility, &Parent), (With<ItemUi>, With<PickedUp>)>,
    pickup: Single<Entity, With<Pickup>>,
    mut inventories: Query<&mut Inventory>,
) {
    let dst = trigger.entity();
    let Ok(dst_slot) = slots.get(dst) else {
        return;
    };
    let (ref item, ref mut visibility, parent) = &mut *item;
    let src = parent.get();
    let src_slot = slots.get(src).unwrap();
    let no_operation = src == dst;
    let destination_full = dst_slot.slot.is_some();
    if !no_operation && destination_full {
        // TODO: Interaction with a different filled slot
        return;
    }

    **visibility = Visibility::Inherited;
    commands.entity(*item).remove::<PickedUp>();
    commands.entity(*pickup).despawn_recursive();

    // interaction between empty and filled slot
    if src_slot.inventory == dst_slot.inventory {
        if src_slot.index != dst_slot.index {
            let mut inv = inventories.get_mut(src_slot.inventory).unwrap();
            let (a, b) = inv.0.split_at_mut(src_slot.index.max(dst_slot.index));
            std::mem::swap(&mut a[src_slot.index.min(dst_slot.index)], &mut b[0]);
            commands.trigger_targets(InventoryChanged, src_slot.inventory);
        }
    } else {
        let [mut src_inv, mut dst_inv] = inventories
            .get_many_mut([src_slot.inventory, dst_slot.inventory])
            .unwrap();
        std::mem::swap(
            &mut src_inv.0[src_slot.index],
            &mut dst_inv.0[dst_slot.index],
        );
        commands.trigger_targets(InventoryChanged, src_slot.inventory);
        commands.trigger_targets(InventoryChanged, dst_slot.inventory);
    }
}

#[derive(Event)]
struct InventoryChanged;

fn inventory_changed(
    trigger: Trigger<InventoryChanged>,
    invs: Query<&Inventory>,
    inv_uis: Query<(&InventoryUi, &Children)>,
    slot_uis: Query<&SlotUi>,
    mut commands: Commands,
) {
    let inv_entity = trigger.entity();
    let inv = invs.get(inv_entity).unwrap();
    // For all UIs of this inventory.
    for (inv_ui, children) in &inv_uis {
        if inv_ui.0 != inv_entity {
            continue;
        }
        // Update outdated UI slots.
        for &child in children {
            let Ok(slot) = slot_uis.get(child) else {
                continue;
            };
            let content = inv.0[slot.index];
            if slot.slot != content {
                commands.trigger_targets(SlotChanged(content), child);
            }
        }
    }
}

#[derive(Event)]
struct ItemChanged;

fn item_changed(
    trigger: Trigger<ItemChanged>,
    slot_uis: Query<(Entity, &SlotUi)>,
    items: Query<(Entity, &Extends), With<Item>>,
    mut commands: Commands,
) {
    let item = trigger.entity();
    let slot = Some(trigger.entity());
    // Update slots that contain this item.
    for (slot_entity, slot_ui) in &slot_uis {
        if slot_ui.slot != slot {
            continue;
        }
        commands.trigger_targets(SlotChanged(slot), slot_entity);
    }
    // Update items that depend on this item.
    for (entity, extends) in &items {
        if extends.0 == item {
            commands.trigger_targets(ItemChanged, entity);
        }
    }
}

#[derive(Event)]
struct SlotChanged(Option<Entity>);

fn slot_changed(
    trigger: Trigger<SlotChanged>,
    mut slots: Query<(&mut SlotUi, Option<&Children>)>,
    items: Query<(), With<ItemUi>>,
    mut commands: Commands,
) {
    let slot_entity = trigger.entity();
    let content = trigger.event().0;
    let (mut slot, children) = slots.get_mut(slot_entity).unwrap();
    slot.slot = content;
    if let Some(children) = children {
        for &child in children {
            if items.contains(child) {
                commands.entity(child).despawn_recursive();
            }
        }
    }
    if let Some(item) = content {
        commands.queue(SpawnItemUi {
            parent_ui: slot_entity,
            item,
        });
    }
}
