use bevy::{picking::pointer::PointerId, prelude::*, window::PrimaryWindow};

use crate::{
    components::Inventory,
    prelude::{Count, Template},
};

use super::{
    change_propagation::InventoryChanged,
    item::{spawn_item, ItemUi},
    layout::CenterPosition,
    slot::SlotUi,
    tooltip::Tooltip,
};

pub fn plugin(app: &mut App) {
    app.add_observer(carry_start);
    app.add_observer(carry_interact);
    app.add_systems(Update, carry_follow_mouse);
}

#[derive(Component)]
pub struct Carry {
    pointer_id: PointerId,
    item: Entity,
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

    commands.queue(move |world: &mut World| {
        let parent = world
            .spawn((
                Transform::from_translation(cursor.extend(0.0)),
                Carry { pointer_id, item },
            ))
            .id();
        spawn_item(world, item)
            .insert(PickingBehavior::IGNORE)
            .set_parent(parent);
    });
}

fn carry_follow_mouse(
    window: Single<&Window, With<PrimaryWindow>>,
    mut carriers: Query<(&mut Transform, &Carry)>,
) {
    let Some((mut transform, _)) = carriers
        .iter_mut()
        .find(|(_, carry)| carry.pointer_id == PointerId::Mouse)
    else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    transform.translation = cursor.extend(0.0);
}

fn carry_interact(
    trigger: Trigger<Pointer<Down>>,
    mut commands: Commands,
    slots: Query<&SlotUi>,
    carriers: Query<&mut Carry>,
) {
    let dst = trigger.entity();
    let Ok(slot) = slots.get(dst) else {
        return;
    };
}

pub struct InventoryInteraction {
    pointer: PointerId,
    button: PointerButton,
    src_inventory: Entity,
    src_index: usize,
    dst_inventory: Entity,
    dst_index: usize,
}

pub fn inventory_interaction(
    trigger: Trigger<InventoryInteraction>,
    mut commands: Commands,
    mut inventories: Query<&mut Inventory>,
    mut items: Query<(&mut Count, &Template)>,
) {
    let button = trigger.button;
    let same_inventory = trigger.src_inventory == trigger.dst_inventory;
    let same_slot = trigger.src_index == trigger.dst_index;
    match (button, same_inventory, same_slot) {
        (PointerButton::Primary, false, _) => {}
        (PointerButton::Primary, true, false) => {}
        (PointerButton::Primary, true, true) => {}
        (PointerButton::Secondary, false, _) => {}
        (PointerButton::Secondary, true, false) => {}
        (PointerButton::Secondary, true, true) => {}
        _ => {}
    }
}
