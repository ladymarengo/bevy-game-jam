use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod tilemap;

#[derive(Component)]
struct Player;

const PIXEL_MULTIPLIER: f32 = 4.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_startup_system(init)
        .add_startup_system(spawn_player)
        .add_startup_system(set_window_resolution)
        .add_startup_system(tilemap::load_map)
        .add_system(player_move)
        .run()
}

fn init(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 1.0 / PIXEL_MULTIPLIER;
    camera_bundle.transform.translation.x = tilemap::TILE_SIZE as f32 * 8.0;
    camera_bundle.transform.translation.y = tilemap::TILE_SIZE as f32 * 6.0;
    commands.spawn_bundle(camera_bundle);
}

fn set_window_resolution(mut windows: ResMut<Windows>) {
    windows
        .get_primary_mut()
        .unwrap()
        .set_resolution(256.0 * PIXEL_MULTIPLIER, 215.0 * PIXEL_MULTIPLIER);
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("f.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(75.0, 50.0)),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
        ..Default::default()
    }).insert(Player);
}

fn player_move(mut player: Query<&mut Transform, With<Player>>, keys: Res<Input<KeyCode>>) {
    let mut player_transform = player.single_mut();

    if keys.pressed(KeyCode::W) {
        player_transform.translation.y += 1.0;
    }
    if keys.pressed(KeyCode::S) {
        player_transform.translation.y -= 1.0;
    }
    if keys.pressed(KeyCode::A) {
        player_transform.translation.x -= 1.0;
    }
    if keys.pressed(KeyCode::D) {
        player_transform.translation.x += 1.0;
    }

}
