use bevy::prelude::*;
use heron::*;

mod tilemap;

#[derive(Component)]
struct Player;

#[derive(Default)]
struct Jump(bool);

const PIXEL_MULTIPLIER: f32 = 4.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_system(bevy::input::system::exit_on_esc_system)
        .insert_resource(Gravity::from(Vec2::new(0.0, -2000.0)))
        .insert_resource(Jump(false))
        .add_startup_system(init)
        .add_startup_system(spawn_player)
        .add_startup_system(set_window_resolution)
        .add_startup_system(tilemap::load_map)
        .add_system(player_move)
        .add_system(stop_jump)
        .run()
}

fn init(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 4.0 / PIXEL_MULTIPLIER;
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

// fn spawn_player(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
// ) {
//     let texture = asset_server.load("main_character.png");
//     let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(32.0, 16.0), 4, 1);
//     let texture_atlas_handle = texture_atlases.add(texture_atlas);
//     commands
//         .spawn_bundle(SpriteSheetBundle {
//             texture_atlas: texture_atlas_handle.clone(),
//             sprite: TextureAtlasSprite {
//                 index: 0,
//                 ..Default::default()
//             },
//             transform: Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
//             ..Default::default()
//         },
//         transform: Transform::from_translation(Vec3::new(150.0, 230.0, 2.0)),
//         ..Default::default()
//     }).insert(RigidBody::Dynamic)
//     .insert(CollisionShape::Cuboid {
//         half_extends: Vec3::new(75.0 / 2.0, 50.0 / 2.0, 0.0),
//         border_radius: None,
//     })
//     .insert(Velocity::from(Vec3::new(0.0, 0.0, 0.0)))
//     .insert(RotationConstraints::lock())
//     .insert(PhysicMaterial {
//         restitution: 0.2,
//         ..Default::default()})
//     .insert(Player);
// }

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
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(75.0 / 2.0, 50.0 / 2.0, 0.0),
            border_radius: None,
        })
        .insert(Velocity::from(Vec3::new(0.0, 0.0, 0.0)))
        .insert(RotationConstraints::lock())
        .insert(PhysicMaterial {
            restitution: 0.2,
            ..Default::default()})
        .insert(Player);
}

fn player_move(mut player: Query<&mut Velocity, With<Player>>, keys: Res<Input<KeyCode>>, mut jump: ResMut<Jump>) {
    let mut player = player.single_mut();
	
    if keys.pressed(KeyCode::W) && !jump.0 {
        player.linear[1] = 800.0;
		jump.0 = true;
    }
    if keys.pressed(KeyCode::A) {
        player.linear[0] = -200.0;
    }
    if keys.pressed(KeyCode::D) {
        player.linear[0] = 200.0;
    }
    if keys.pressed(KeyCode::S) {
        player.linear[1] = -500.0;
    }
}

fn stop_jump(mut events: EventReader<CollisionEvent>, mut jump: ResMut<Jump>) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(_d1, _d2) => {
				jump.0 = false;
            }
            CollisionEvent::Stopped(_d1, _d2) => ()
        }
    }
}
