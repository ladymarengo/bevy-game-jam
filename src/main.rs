use advantage::{Advantage, EnemyAdvantage};
use benimator::*;
use bevy::prelude::*;
use heron::*;
use hud::{spawn_hud, update_advantage, update_hp_meter};
use instant::Instant;

mod player;
mod advantage;
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
        .insert_resource(player::Jump(0))
        .insert_resource(Hit(false))
        .insert_resource(HitTime(Instant::now()))
        .insert_resource(Hp(20))
        .insert_resource(Advantage::random())
        .add_startup_system(init)
        .add_startup_system(set_window_resolution)
        .add_startup_system(tilemap::load_initial_map)
        .add_system(player::r#move)
        .add_system(check_collisions)
        .add_system(enemy::r#move)
        .add_system(cameraman)
        .add_system(check_hits)
        .add_startup_system(spawn_hud)
        .add_system(update_hp_meter)
        .add_system(update_advantage)
        .add_startup_system(spawn_stars)
        // .add_system_to_stage(CoreStage::PostUpdate, delete_stars)
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
    stars: Query<Entity, With<Star>>,
    mut commands: Commands,
    mut hp: ResMut<Hp>,
    adv: Res<Advantage>,
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
                    &stars,
                    &mut commands,
                    &mut hp,
                    &adv,
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
                    &stars,
                    &mut commands,
                    &mut hp,
                    &adv,
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
                    &stars,
                    &mut commands,
                    &mut hp,
                    &adv,
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
                    &stars,
                    &mut commands,
                    &mut hp,
                    &adv,
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
    stars: &Query<Entity, With<Star>>,
    commands: &mut Commands,
    hp: &mut ResMut<Hp>,
    adv: &Res<Advantage>,
) {
    if player.normals().iter().any(|normal| normal.y >= 0.9) {
        jump.0 = 0;
    }

    let other_entity = other.rigid_body_entity();

    if enemy.get(other_entity).is_ok() {
        if state == "started" {
            hit.0 = true;
            hit_time.0 = Instant::now();
            // println!("started");
        } else {
            hit.0 = false;
            // println!("stopped");
        }
    }

    if stars.get(other_entity).is_ok() {
        if matches!(
            adv.as_ref(),
            Advantage::Player(advantage::PlayerAdvantage::DoubleInitialHp)
        ) {
            hp.0 += 2;
        } else {
            hp.0 += 1;
        };

        commands.entity(other_entity).despawn();
    }
}

fn check_hits(
    hit: ResMut<Hit>,
    mut hit_time: ResMut<HitTime>,
    mut hp: ResMut<Hp>,
    advantage: Res<Advantage>,
) {
    if hit.0 && hit_time.0.elapsed().as_millis() > 300 {
        let bite_strength = if matches!(
            advantage.as_ref(),
            Advantage::Enemy(EnemyAdvantage::DoubleBite)
        ) {
            3
        } else {
            1
        };
        hp.0 -= bite_strength;
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

#[derive(Component)]
pub struct Star;

fn spawn_stars(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(tilemap::TILE_SIZE as f32 * 20.0,
                    tilemap::TILE_SIZE as f32 * 32.0,
                    4.0,),
                scale: Vec3::new(10.0, 10.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Star)
        .insert(RigidBody::Static)
        .with_children(|children| {
            children.spawn_bundle((
                SensorShape,
                CollisionShape::Sphere {
                    radius: 5.0,
                },
            ));
        });
}
