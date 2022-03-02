use bevy::prelude::*;

mod tilemap;

#[derive(Component)]
struct Player;

const PIXEL_MULTIPLIER: f32 = 4.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture = asset_server.load("main_character.png");
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(32.0, 16.0), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
            ..Default::default()
        })
        .insert(Player);
}

fn player_move(
    mut player: Query<(&mut Transform, &mut TextureAtlasSprite), With<Player>>,
    keys: Res<Input<KeyCode>>,
) {
    let (mut player_transform, mut texture_atlas_sprite) = player.single_mut();
    let mut moved = false;

    if keys.pressed(KeyCode::W) {
        player_transform.translation.y += 1.0;
        moved = true;
    }
    if keys.pressed(KeyCode::S) {
        player_transform.translation.y -= 1.0;
        moved = true;
    }
    if keys.pressed(KeyCode::A) {
        player_transform.translation.x -= 1.0;
        moved = true;
    }
    if keys.pressed(KeyCode::D) {
        player_transform.translation.x += 1.0;
        moved = true;
    }

    // TODO: add timer instead of changing every frame
    if moved {
        texture_atlas_sprite.index = (texture_atlas_sprite.index + 1) % 4;
    }
}
