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
        .add_system(copy_physics_coordinates)
        .run()
}

fn init(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 4.0 / PIXEL_MULTIPLIER;
    camera_bundle.transform.translation.x = tilemap::TILE_SIZE as f32 * 8.0;
    camera_bundle.transform.translation.y = tilemap::TILE_SIZE as f32 * 6.0;
    commands.spawn_bundle(camera_bundle);
    rapier_config.gravity = Vec2::new(0.0, -5000.0).into();
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
        transform: Transform::from_translation(Vec3::new(150.0, 130.0, 2.0)),
        ..Default::default()
    }).insert(Player)
    .insert_bundle(RigidBodyBundle {
        // FIXME: center or corner?
        position: Vec2::new(150.0, 230.0).into(),
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        damping: RigidBodyDampingComponent(RigidBodyDamping {
            linear_damping: 0.8,
            angular_damping: 1.0,
        }),
        ..Default::default()
    })
    .insert_bundle(ColliderBundle {
        shape: ColliderShapeComponent( ColliderShape::cuboid(
            75.0,
            50.0,
        )),
        material: ColliderMaterialComponent( ColliderMaterial {
            restitution: 0.0,
            ..Default::default()
        }),
        // flags: ColliderFlags {
        //     collision_groups: InteractionGroups::new(
        //         PLAYER_COLLIDER_GROUP,
        //         u32::MAX,
        //     ),
        //     ..Default::default()
        // },
        ..Default::default()
    })
    .insert(ColliderPositionSync::Discrete);
}

fn copy_physics_coordinates(mut positions: Query<(&RigidBodyPositionComponent, &mut Transform)>)
{
    for (phys_pos, mut bevy_pos) in positions.iter_mut() {
        bevy_pos.translation.x = phys_pos.0.position.translation.x;
        bevy_pos.translation.y = phys_pos.0.position.translation.y;
    }
}

fn player_move(mut player: Query<&mut RigidBodyForcesComponent, With<Player>>, keys: Res<Input<KeyCode>>) {
    let mut player_forces = player.single_mut();

    if keys.pressed(KeyCode::W) {
        player_forces.force = Vec2::new(0.0, 500000000.0).into();
    }
    if keys.pressed(KeyCode::A) {
        player_forces.force = Vec2::new(-2e8, 0.0).into();
    }
    if keys.pressed(KeyCode::D) {
        player_forces.force = Vec2::new(2e8, 0.0).into();
    }
    // if keys.pressed(KeyCode::S) {
    //     player_forces.translation.y -= 1.0;
    // }
    // if keys.pressed(KeyCode::A) {
    //     player_forces.translation.x -= 1.0;
    // }
    // if keys.pressed(KeyCode::D) {
    //     player_forces.translation.x += 1.0;
    // }

}
