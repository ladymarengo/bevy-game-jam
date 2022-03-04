use benimator::*;
use bevy::prelude::*;
use heron::*;
use instant::Instant;
use std::time::Duration;

mod tilemap;

mod enemy;

#[derive(Component)]
pub struct Player;

#[derive(Default)]
struct Jump(bool);

#[derive(Default)]
pub struct Hit(bool);

pub struct HitTime(Instant);

const PIXEL_MULTIPLIER: f32 = 4.0;

fn main() {
    App::new()
        .init_resource::<enemy::Animations>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(AnimationPlugin::default())
        .add_system(bevy::input::system::exit_on_esc_system)
        .insert_resource(Gravity::from(Vec2::new(0.0, -2000.0)))
        .insert_resource(Jump(false))
        .insert_resource(Hit(false))
        .insert_resource(HitTime(Instant::now()))
        .add_startup_system(init)
        .add_startup_system(spawn_player)
        .add_startup_system(set_window_resolution)
        .add_startup_system(tilemap::load_initial_map)
        .add_system(player_move)
        .add_system(check_collisions)
        .add_startup_system(enemy::spawn_enemy)
        .add_system(enemy::enemy_move)
        .add_system(cameraman)
        .add_system(check_hits)
        .run()
}

fn init(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 1.0 / PIXEL_MULTIPLIER;
    camera_bundle.transform.translation.x = tilemap::TILE_SIZE as f32 * 8.0;
    camera_bundle.transform.translation.y = tilemap::TILE_SIZE as f32 * 11.0;
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
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    let texture = asset_server.load("ferris-Sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(32.0, 32.0), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let animation_handle = animations.add(SpriteSheetAnimation::from_range(
        0..=3,
        Duration::from_millis(100),
    ));

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(150.0, 230.0, 5.0)),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(32.0 / 2.0, 16.0 / 2.0, 0.0),
            border_radius: None,
        })
        .insert(Velocity::from(Vec3::new(0.0, 0.0, 0.0)))
        .insert(RotationConstraints::lock())
        .insert(PhysicMaterial {
            restitution: 0.2,
            ..Default::default()
        })
        .insert(Player)
        .insert(animation_handle)
        .insert(Play);
}

fn player_move(
    mut commands: Commands,
    mut player: Query<(Entity, &mut Velocity), With<Player>>,
    keys: Res<Input<KeyCode>>,
    mut jump: ResMut<Jump>,
) {
    let (id, mut player) = player.single_mut();

    commands.entity(id).remove::<Play>();

    if keys.pressed(KeyCode::W) && !jump.0 {
        player.linear[1] = 800.0;
        jump.0 = true;
    }
    if keys.pressed(KeyCode::A) {
        player.linear[0] = -200.0;
        if !jump.0 {
            commands.entity(id).insert(Play);
        }
    }
    if keys.pressed(KeyCode::D) {
        player.linear[0] = 200.0;
        if !jump.0 {
            commands.entity(id).insert(Play);
        }
    }
    if keys.pressed(KeyCode::S) {
        player.linear[1] = -500.0;
    }
}

fn check_collisions(
    mut events: EventReader<CollisionEvent>,
    mut jump: ResMut<Jump>,
    mut hit: ResMut<Hit>,
    player: Query<Entity, With<Player>>,
    enemy: Query<Entity, With<enemy::Enemy>>,
    mut hit_time: ResMut<HitTime>,
) {
    let id = player.single();
    for event in events.iter() {
        match event {
            CollisionEvent::Started(player_c, other_c) if player_c.rigid_body_entity() == id => {
                handle_player_collision(
                    player_c,
                    other_c,
                    &mut jump,
                    &enemy,
                    &mut hit,
                    "started",
                    &mut hit_time,
                );
            }
            CollisionEvent::Started(other_c, player_c) if player_c.rigid_body_entity() == id => {
                handle_player_collision(
                    player_c,
                    other_c,
                    &mut jump,
                    &enemy,
                    &mut hit,
                    "started",
                    &mut hit_time,
                );
            }
            CollisionEvent::Stopped(player_c, other_c) if player_c.rigid_body_entity() == id => {
                handle_player_collision(
                    player_c,
                    other_c,
                    &mut jump,
                    &enemy,
                    &mut hit,
                    "stopped",
                    &mut hit_time,
                );
            }
            CollisionEvent::Stopped(other_c, player_c) if player_c.rigid_body_entity() == id => {
                handle_player_collision(
                    player_c,
                    other_c,
                    &mut jump,
                    &enemy,
                    &mut hit,
                    "stopped",
                    &mut hit_time,
                );
            }
            _ => (),
        }
    }
}

fn handle_player_collision(
    player: &CollisionData,
    other: &CollisionData,
    jump: &mut ResMut<Jump>,
    enemy: &Query<Entity, With<enemy::Enemy>>,
    hit: &mut ResMut<Hit>,
    state: &str,
    hit_time: &mut ResMut<HitTime>,
) {
    if player.normals().iter().any(|normal| normal.y >= 0.9) {
        jump.0 = false;
    }

    for enemy in enemy.iter() {
        if other.rigid_body_entity() == enemy {
            if state == "started" {
                hit.0 = true;
                hit_time.0 = Instant::now();
                // println!("started");
            } else {
                hit.0 = false;
                // println!("stopped");
            }
        }
    }
}

fn check_hits(hit: ResMut<Hit>, mut hit_time: ResMut<HitTime>) {
    if hit.0 && hit_time.0.elapsed().as_millis() > 300 {
        println!("hit");
        hit_time.0 = Instant::now();
    }
}

#[allow(clippy::type_complexity)]
fn cameraman(
    mut position_queries: QuerySet<(
        QueryState<&mut Transform, With<Camera>>,
        QueryState<&Transform, With<Player>>,
    )>,
) {
    let player_pos = {
        match position_queries.q1().get_single() {
            Ok(p) => p.translation,
            Err(e) => {
                info!("Querying player errored with {:?}", e);
                return;
            }
        }
    };

    match position_queries.q0().get_single_mut() {
        Ok(mut c) => {
            c.translation.x = player_pos.x;
            // c.translation.y = player_pos.y;
        }
        Err(e) => {
            info!("Querying camera errored with {:?}", e);
        }
    }
}
