use bevy::prelude::*;

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

/// Constant size container of items.
#[derive(Component)]
pub struct Inventory(pub Box<[Option<Entity>]>);

impl Inventory {
    pub fn new<const N: usize>() -> Self {
        Self(Box::new(core::array::from_fn::<_, N, _>(|_| None)))
    }
}
