use benimator::*;
use bevy::prelude::*;
use heron::*;
use hud::{spawn_hp_meter, update_hp_meter};
use instant::Instant;

mod player;
mod enemy;
mod hud;
mod tilemap;

#[derive(Component)]
struct MainCamera;

#[derive(Default)]
pub struct Hit(bool);

pub struct HitTime(Instant);

pub struct Hp(pub u8);

const PIXEL_MULTIPLIER: f32 = 3.0;

fn main() {
    App::new()
        .init_resource::<enemy::Animations>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(AnimationPlugin::default())
        .add_system(bevy::input::system::exit_on_esc_system)
        .insert_resource(ClearColor(Color::hex("29366f").unwrap()))
        .insert_resource(Gravity::from(Vec2::new(0.0, -1500.0)))
        .insert_resource(player::Jump(false))
        .insert_resource(Hit(false))
        .insert_resource(HitTime(Instant::now()))
        .insert_resource(Hp(20))
        .add_startup_system(init)
        .add_startup_system(set_window_resolution)
        .add_startup_system(tilemap::load_initial_map)
        .add_system(player::r#move)
        .add_system(check_collisions)
        .add_startup_system(enemy::spawn_enemy)
        .add_system(enemy::enemy_move)
        .add_system(cameraman)
        .add_system(check_hits)
        .add_startup_system(spawn_hp_meter)
        .add_system(update_hp_meter)
        .run()
}

fn init(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 1.0 / PIXEL_MULTIPLIER;
    camera_bundle.transform.translation.x = tilemap::TILE_SIZE as f32 * 8.0;
    camera_bundle.transform.translation.y = tilemap::TILE_SIZE as f32 * 11.0;
    commands.spawn_bundle(camera_bundle).insert(MainCamera);
}

fn set_window_resolution(mut windows: ResMut<Windows>) {
    windows
        .get_primary_mut()
        .unwrap()
        .set_resolution(341.0 * PIXEL_MULTIPLIER, 256.0 * PIXEL_MULTIPLIER);
}

fn check_collisions(
    mut events: EventReader<CollisionEvent>,
    mut jump: ResMut<player::Jump>,
    mut hit: ResMut<Hit>,
    player: Query<Entity, With<player::Player>>,
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
    jump: &mut ResMut<player::Jump>,
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

fn check_hits(hit: ResMut<Hit>, mut hit_time: ResMut<HitTime>, mut hp: ResMut<Hp>) {
    if hit.0 && hit_time.0.elapsed().as_millis() > 300 {
        hp.0 -= 1;
        hit_time.0 = Instant::now();
    }
}

#[allow(clippy::type_complexity)]
fn cameraman(
    mut position_queries: QuerySet<(
        QueryState<&mut Transform, With<MainCamera>>,
        QueryState<&Transform, With<player::Player>>,
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
            c.translation.x = player_pos.x.round();
            c.translation.y = player_pos.y.round();
        }
        Err(e) => {
            info!("Querying camera errored with {:?}", e);
        }
    }
}
