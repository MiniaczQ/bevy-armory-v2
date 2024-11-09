use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};
use bevy_armory_v2::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, ItemPlugin));
    app.add_systems(Startup, setup);
    app.run();
}

#[derive(Component)]
struct CanMineBlocks;

fn nearest_sampler(settings: &mut ImageLoaderSettings) {
    settings.sampler = ImageSampler::nearest()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // Define items
    let precursor = commands.spawn((Item, Name::new("item::precursor"))).id();
    let pickaxe = commands
        .spawn((
            Item,
            Extends(precursor),
            Name::new("item::pickaxe"),
            Icon(asset_server.load_with_settings("pickaxe.png", nearest_sampler)),
            CanMineBlocks,
        ))
        .id();
    let stone = commands
        .spawn((
            Item,
            Extends(precursor),
            Name::new("item::stone"),
            Icon(asset_server.load_with_settings("stone.png", nearest_sampler)),
        ))
        .id();
    let stored_stone = commands.spawn((Item, Extends(stone), Count(2))).id();

    // Spawn UI
    let ui_root = commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            ..default()
        })
        .id();
    commands.queue(SpawnItemUi {
        parent_ui: ui_root,
        item: pickaxe,
    });
    commands.queue(SpawnItemUi {
        parent_ui: ui_root,
        item: stored_stone,
    });
}
