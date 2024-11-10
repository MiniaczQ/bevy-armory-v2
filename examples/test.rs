use bevy::prelude::*;
use bevy_armory::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, ItemPlugin));
    app.add_systems(Startup, setup);
    app.run();
}

#[derive(Component)]
struct CanMineBlocks;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // Define items
    let pickaxe = commands
        .spawn((
            Item,
            Name::new("item::pickaxe"),
            Icon(asset_server.load_with_settings("pickaxe.png", nearest_sampler)),
            CanMineBlocks,
        ))
        .id();

    let stone = commands
        .spawn((
            Item,
            Name::new("item::stone"),
            Icon(asset_server.load_with_settings("stone.png", nearest_sampler)),
        ))
        .id();
    let stored_stone1 = commands.spawn((Item, Template(stone), Count(2))).id();
    let stored_stone2 = commands.spawn((Item, Template(stone), Count(5))).id();

    // Spawn inventory
    let mut inv = Inventory::new::<4>();
    inv.0[0] = Some(pickaxe);
    inv.0[1] = Some(stored_stone1);
    let inv1 = commands.spawn((inv, Name::new("Inventory 1"))).id();

    let mut inv = Inventory::new::<6>();
    inv.0[3] = Some(stored_stone2);
    let inv2 = commands.spawn((inv, Name::new("Inventory 2"))).id();

    let inv = Inventory::new::<3>();
    let inv3 = commands.spawn((inv, Name::new("Inventory 3"))).id();

    // Spawn UI
    let ui_root = commands.spawn(Node::DEFAULT).id();

    commands.queue(SpawnInventoryUi {
        parent: ui_root,
        offset: SLOT_SIZE * Vec2::new(0.0, 0.0),
        inventory: InventoryUi { data: inv1 },
    });
    commands.queue(SpawnInventoryUi {
        parent: ui_root,
        offset: SLOT_SIZE * Vec2::new(4.0, 0.0),
        inventory: InventoryUi { data: inv2 },
    });
    commands.queue(SpawnInventoryUi {
        parent: ui_root,
        offset: SLOT_SIZE * Vec2::new(8.0, 3.0),
        inventory: InventoryUi { data: inv3 },
    });
}
