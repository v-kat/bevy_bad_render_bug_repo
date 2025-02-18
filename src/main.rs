use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Test".to_string(),
                    ..default()
                }),
                ..default()
            }),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, test_system)
        .run();

}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn test_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut img_handle = asset_server.load("tester.webp");
    for x in 10..20 {
        img_handle = asset_server.load(format!("tester{}.webp", x));
    }

    commands.spawn(SpriteBundle {
        texture: img_handle,
        ..default()
    });
}
