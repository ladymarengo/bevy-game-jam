use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_startup_system(background)
        .add_startup_system(init)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_ground)
        .add_startup_system(set_window_resolution)
        .add_system(player_move)
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
        transform: Transform::from_translation(Vec3::new(0.0, -350.0, 2.0)),
        ..Default::default()
    }).insert(Player);
}

fn spawn_ground(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(2000.0, 100.0)),
            color: Color::rgb(0.76, 0.55, 0.10),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, -400.0, 1.0)),
        ..Default::default()
    });
}


fn background(mut commands: Commands, asset_server: Res<AssetServer>) {
    // FIXME: Needs to be synched with the camera
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("bg.png"),
        // transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
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