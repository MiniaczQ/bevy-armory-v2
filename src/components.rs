//! Core components for items.

use bevy::prelude::*;

/// Marker component for items.
#[derive(Component)]
pub struct Item;

/// Image handle for this item.
#[derive(Component)]
pub struct Icon(pub Handle<Image>);

/// Base this item on another item.
#[derive(Component)]
pub struct Template(pub Entity);

/// Stores data about amount of an item.
/// This component shouldn't be used in template items.
#[derive(Component)]
pub struct Count(pub u32);

/// Constant size container for items.
#[derive(Component)]
pub struct Inventory(pub Box<[Option<Entity>]>);

impl Inventory {
    pub fn new<const N: usize>() -> Self {
        Self(Box::new(core::array::from_fn::<_, N, _>(|_| None)))
    }
}
