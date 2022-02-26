use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_startup_system(init)
        .add_startup_system(spawn_player)
        .add_startup_system(set_window_resolution)
        .run()
}

fn init(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

}

fn set_window_resolution(mut windows: ResMut<Windows>) {
    windows
        .get_primary_mut()
        .unwrap()
        .set_resolution(1024.0, 860.0);
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("f.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(75.0, 50.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}
