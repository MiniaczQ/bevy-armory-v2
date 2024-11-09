use bevy::prelude::*;

use crate::{
    components::{Extends, Inventory, Item},
    ui::{
        inventory::InventoryUi,
        item::{ItemUi, SpawnItemUi},
        slot::SlotUi,
    },
};

pub fn plugin(app: &mut App) {
    app.add_observer(inventory_changed);
    app.add_observer(item_changed);
    app.add_observer(slot_changed);
}

#[derive(Event)]
pub struct InventoryChanged;

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
pub struct ItemChanged;

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
pub struct SlotChanged(pub Option<Entity>);

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
