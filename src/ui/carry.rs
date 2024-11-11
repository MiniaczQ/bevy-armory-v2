use std::marker::PhantomData;

use bevy::{picking::pointer::PointerId, prelude::*, window::PrimaryWindow};

use crate::components::Inventory;

use super::{
    change_propagation::InventoryChanged,
    item::spawn_item,
    layout::CenterPosition,
    prelude::{InventoryUi, SlotChanged},
    slot::SlotUi,
};

pub fn plugin(app: &mut App) {
    app.add_observer(carry_start);
    app.add_systems(Update, carry_follow_mouse);
    app.add_observer(carry_interact);
    app.add_observer(swap);
    app.add_observer(carry_despawn);
}

#[derive(Component)]
pub struct Carry {
    pub pointer_id: PointerId,
    pub item: Entity,
}

fn carry_start(
    trigger: Trigger<Pointer<Down>>,
    mut commands: Commands,
    mut slots: Query<&SlotUi>,
    carriers: Query<&Carry>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut inventories: Query<&mut Inventory>,
) {
    let pointer_id = trigger.pointer_id;
    let pointer_in_use = !carriers.iter().all(|c| c.pointer_id != pointer_id);
    if pointer_in_use {
        return;
    }
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let Ok(slot) = slots.get_mut(trigger.entity()) else {
        return;
    };

    let mut inventory = inventories.get_mut(slot.inventory).unwrap();
    let Some(item) = inventory.0[slot.index].take() else {
        return;
    };
    commands.trigger_targets(InventoryChanged, slot.inventory);

    let mut inventory = Inventory::new::<1>();
    inventory.0[0] = Some(item);
    commands.queue(move |world: &mut World| {
        let carry = world
            .spawn((
                inventory,
                Carry { pointer_id, item },
                CenterPosition { position: cursor },
                PickingBehavior::IGNORE,
                Node::default(),
            ))
            .id();
        let inventory = world
            .spawn((
                InventoryUi { data: carry },
                PickingBehavior::IGNORE,
                Node::default(),
            ))
            .set_parent(carry)
            .id();
        let slot = world
            .spawn((
                SlotUi {
                    data: Some(item),
                    inventory: carry,
                    index: 0,
                },
                PickingBehavior::IGNORE,
                Node::default(),
            ))
            .set_parent(inventory)
            .id();
        spawn_item(world, item)
            .insert(PickingBehavior::IGNORE)
            .set_parent(slot);
    });
}

fn carry_follow_mouse(
    window: Single<&Window, With<PrimaryWindow>>,
    mut carriers: Query<(&mut CenterPosition, &Carry)>,
) {
    let Some((mut center, _)) = carriers
        .iter_mut()
        .find(|(_, carry)| carry.pointer_id == PointerId::Mouse)
    else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    center.position = cursor;
}

fn carry_interact(
    trigger: Trigger<Pointer<Down>>,
    mut commands: Commands,
    slots: Query<&SlotUi>,
    carriers: Query<(&Carry, Entity)>,
) {
    let pointer_id = trigger.pointer_id;
    let carry = carriers.iter().find(|c| c.0.pointer_id == pointer_id);
    let Some((_, carry_entity)) = carry else {
        return;
    };
    let slot_entity = trigger.entity();
    let Ok(slot) = slots.get(slot_entity) else {
        return;
    };
    match trigger.button {
        PointerButton::Primary => {
            commands.trigger(Interaction::<Swap> {
                action: PhantomData::default(),
                data: InteractionData::new(carry_entity, 0, slot.inventory, slot.index),
            });
        }
        _ => {}
    }
}

#[derive(Event)]
pub struct Interaction<A: Action> {
    pub action: PhantomData<A>,
    pub data: InteractionData,
}

pub enum InteractionData {
    SameSlot {
        inventory: Entity,
        index: usize,
    },
    SameInventory {
        inventory: Entity,
        index_a: usize,
        index_b: usize,
    },
    Different {
        inventory_a: Entity,
        index_a: usize,
        inventory_b: Entity,
        index_b: usize,
    },
}

impl InteractionData {
    pub fn new(inventory_a: Entity, index_a: usize, inventory_b: Entity, index_b: usize) -> Self {
        if inventory_a == inventory_b {
            if index_a == index_b {
                Self::SameSlot {
                    inventory: inventory_a,
                    index: index_a,
                }
            } else {
                Self::SameInventory {
                    inventory: inventory_a,
                    index_a,
                    index_b,
                }
            }
        } else {
            Self::Different {
                inventory_a,
                index_a,
                inventory_b,
                index_b,
            }
        }
    }
}

pub trait Action {}

pub struct Swap;

impl Action for Swap {}

pub fn swap(
    trigger: Trigger<Interaction<Swap>>,
    mut inventories: Query<&mut Inventory>,
    mut commands: Commands,
) {
    match trigger.data {
        InteractionData::SameSlot {
            inventory: _,
            index: _,
        } => {
            return;
        }
        InteractionData::SameInventory {
            inventory,
            index_a,
            index_b,
        } => {
            let mut inv_data = inventories.get_mut(inventory).unwrap();
            let (a, b) = inv_data.0.split_at_mut(index_a.max(index_b));
            let (slot_a, slot_b) = (&mut a[index_a.min(index_b)], &mut b[0]);
            std::mem::swap(slot_a, slot_b);
            commands.trigger_targets(InventoryChanged, inventory);
        }
        InteractionData::Different {
            inventory_a,
            index_a,
            inventory_b,
            index_b,
        } => {
            let [mut inv_data_a, mut inv_data_b] = inventories
                .get_many_mut([inventory_a, inventory_b])
                .unwrap();
            let (slot_a, slot_b) = (&mut inv_data_a.0[index_a], &mut inv_data_b.0[index_b]);
            std::mem::swap(slot_a, slot_b);
            commands.trigger_targets(InventoryChanged, inventory_a);
            commands.trigger_targets(InventoryChanged, inventory_b);
        }
    }
}

pub fn carry_despawn(
    trigger: Trigger<SlotChanged>,
    slots: Query<&SlotUi>,
    carriers: Query<(), With<Carry>>,
    mut commands: Commands,
) {
    let Ok(slot) = slots.get(trigger.entity()) else {
        return;
    };
    if !carriers.contains(slot.inventory) {
        return;
    }
    if trigger.0.is_some() {
        return;
    }
    commands.entity(slot.inventory).despawn_recursive();
}
