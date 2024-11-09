use bevy::prelude::*;
use bevy_armory_v2::prelude::*;

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
    let precursor = commands.spawn((Item, Name::new("item::precursor"))).id();
    let pickaxe = commands
        .spawn((
            Item,
            Template(precursor),
            Name::new("item::pickaxe"),
            Icon(asset_server.load_with_settings("pickaxe.png", nearest_sampler)),
            CanMineBlocks,
        ))
        .id();
    let stone = commands
        .spawn((
            Item,
            Template(precursor),
            Name::new("item::stone"),
            Icon(asset_server.load_with_settings("stone.png", nearest_sampler)),
        ))
        .id();
    let stored_stone = commands.spawn((Item, Template(stone), Count(2))).id();

    // Spawn inventory
    let mut inventory = Inventory::new::<4>();
    inventory.0[0] = Some(pickaxe);
    inventory.0[1] = Some(stored_stone);
    let source = commands
        .spawn((inventory, Name::new("Source Inventory")))
        .id();
    let destination = commands
        .spawn((Inventory::new::<4>(), Name::new("Destination Inventory")))
        .id();

    // Spawn UI
    let ui_root = commands.spawn(Node::DEFAULT).id();

    commands.queue(SpawnInventoryUi {
        parent: ui_root,
        offset: Vec2::new(0.0, 0.0),
        inventory: InventoryUi { data: source },
    });
    commands.queue(SpawnInventoryUi {
        parent: ui_root,
        offset: Vec2::new(SLOT_SIZE * 2.0, 0.0),
        inventory: InventoryUi { data: destination },
    });
}
