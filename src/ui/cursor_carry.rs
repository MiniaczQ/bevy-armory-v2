use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::Inventory;

use super::{
    change_propagation::InventoryChanged, item::ItemUi, layout::CenterPosition, slot::SlotUi,
    tooltip::Tooltip,
};

pub fn plugin(app: &mut App) {
    app.add_observer(pickup_item);
    app.add_observer(drop_item);
    app.add_systems(Update, pickup_follow);
}

#[derive(Component)]
pub struct BeingCarried;

#[derive(Component)]
pub struct CursorCarry;

fn pickup_item(
    trigger: Trigger<Pointer<Down>>,
    mut commands: Commands,
    mut items: Query<(&Children, &UiImage, &Node, &mut Visibility), With<ItemUi>>,
    tooltips: Query<(), With<Tooltip>>,
    pickup: Option<Single<(), With<CursorCarry>>>,
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
    commands.entity(trigger.entity()).insert(BeingCarried);
    *visibility = Visibility::Hidden;

    let cursor = window.cursor_position().unwrap();
    commands.spawn((
        UiImage::new(image.image.clone()),
        Node {
            position_type: PositionType::Absolute,
            height: node.height,
            width: node.width,
            ..default()
        },
        CursorCarry,
        PickingBehavior::IGNORE,
        CenterPosition { position: cursor },
    ));
}

fn pickup_follow(
    window: Single<&Window, With<PrimaryWindow>>,
    mut pickup: Single<&mut CenterPosition, With<CursorCarry>>,
) {
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    pickup.position = cursor;
}

fn drop_item(
    trigger: Trigger<Pointer<Down>>,
    mut commands: Commands,
    slots: Query<&SlotUi>,
    mut item: Single<(Entity, &mut Visibility, &Parent), (With<ItemUi>, With<BeingCarried>)>,
    pickup: Single<Entity, With<CursorCarry>>,
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
    let destination_full = dst_slot.data.is_some();
    if !no_operation && destination_full {
        // TODO: Interaction with a different filled slot
        return;
    }

    **visibility = Visibility::Inherited;
    commands.entity(*item).remove::<BeingCarried>();
    commands.entity(*pickup).despawn_recursive();

    // Interaction between empty and filled slot
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
