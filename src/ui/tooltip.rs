//! Item tooltip display.

use bevy::prelude::*;

use crate::params::ItemData;

use super::{cursor_carry::CursorCarry, item::ItemUi, ITEM_SIZE};

pub fn plugin(app: &mut App) {
    app.add_observer(item_tooltip_spawn);
    app.add_observer(item_tooltip_follow);
    app.add_observer(item_tooltip_despawn);
}

/// Marker component for tooltip UI root.
#[derive(Component)]
pub struct Tooltip;

fn item_tooltip_spawn(
    trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    items: Query<(&ItemUi, &GlobalTransform), Without<CursorCarry>>,
    names: ItemData<&Name>,
) {
    let Ok((item, transform)) = items.get(trigger.entity()) else {
        return;
    };
    let item = names.extended_get(item.data).unwrap().unwrap();
    let position = trigger.pointer_location.position - transform.translation().xy()
        + Vec2::splat(ITEM_SIZE / 2.0);
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(position.x),
                top: Val::Px(position.y),
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
    items: Query<(&Children, &GlobalTransform), With<ItemUi>>,
    mut tooltips: Query<&mut Node, With<Tooltip>>,
) {
    let Ok((children, transform)) = items.get(trigger.entity()) else {
        return;
    };
    for &child in children {
        let Ok(mut node) = tooltips.get_mut(child) else {
            continue;
        };
        let position = trigger.pointer_location.position - transform.translation().xy()
            + Vec2::splat(ITEM_SIZE / 2.0);
        node.left = Val::Px(position.x);
        node.top = Val::Px(position.y);
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
