use advantage::{Advantage, EnemyAdvantage};
use benimator::*;
use bevy::prelude::*;
use heron::*;
use hud::{spawn_hud, update_advantage, update_hp_meter, fade_out_hint};
use instant::Instant;
use std::env;
use std::time::Duration;

mod advantage;
mod enemy;
mod goal;
mod hud;
mod player;
mod tilemap;
mod bubble;

#[derive(Component)]
struct MainCamera;

#[derive(Default)]
pub struct Hit(bool);

pub struct HitTime(Instant);

pub struct Hp(pub u8);

const PIXEL_MULTIPLIER: f32 = 3.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    InGame,
    Died,
    Won,
}

#[derive(Debug, Clone)]
enum PlayerCollisionEventType {
    Started,
    Stopped,
}

#[derive(Debug, Clone)]
struct PlayerCollision {
    pub player: CollisionData,
    pub other: CollisionData,
    pub event_type: PlayerCollisionEventType,
}

fn main() {
    App::new()
        .add_state(AppState::InGame)
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
        .insert_resource(Hp(5))
        .insert_resource(Advantage::random())
        .add_startup_system(init)
        .add_startup_system(set_window_resolution)
        .add_startup_system(tilemap::load_initial_map)
        .add_event::<PlayerCollision>()
        .add_event::<tilemap::ChangeMap>()
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(player::r#move)
                .with_system(check_collisions.label("collisions"))
                .with_system(enemy::r#move)
                .with_system(cameraman)
                .with_system(check_hits)
                .with_system(bubble::process_bubble_generators)
                .with_system(bubble::process_bubbles)
                .with_system(handle_player_collisions.after("collisions"))
                .with_system(tilemap::handle_change_map.after("collisions")),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Died)
                .with_system(on_die)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Won)
                .with_system(on_win)
        )

        // HUD
        .add_startup_system(spawn_hud)
        .add_system(update_hp_meter)
        .add_system(update_advantage)
        .add_system(fade_out_hint)
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
    player_entity: Query<Entity, With<player::Player>>,
    mut player_collision_writer: EventWriter<PlayerCollision>,
) {
    let id = player_entity.single();
    for event in events.iter() {
        match event {
            CollisionEvent::Started(player, other) if player.rigid_body_entity() == id => {
                player_collision_writer.send(PlayerCollision {
                    player: player.clone(),
                    other: other.clone(),
                    event_type: PlayerCollisionEventType::Started,
                })
            }
            CollisionEvent::Started(other, player) if player.rigid_body_entity() == id => {
                player_collision_writer.send(PlayerCollision {
                    player: player.clone(),
                    other: other.clone(),
                    event_type: PlayerCollisionEventType::Started,
                })
            }
            CollisionEvent::Stopped(player, other) if player.rigid_body_entity() == id => {
                player_collision_writer.send(PlayerCollision {
                    player: player.clone(),
                    other: other.clone(),
                    event_type: PlayerCollisionEventType::Stopped,
                })
            }
            CollisionEvent::Stopped(other, player) if player.rigid_body_entity() == id => {
                player_collision_writer.send(PlayerCollision {
                    player: player.clone(),
                    other: other.clone(),
                    event_type: PlayerCollisionEventType::Stopped,
                })
            }
            _ => (),
        }
    }
}

fn handle_player_collisions(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    mut jump: ResMut<player::Jump>,
    mut hit: ResMut<Hit>,
    mut hit_time: ResMut<HitTime>,
    mut hp: ResMut<Hp>,
    adv: Res<Advantage>,
    enemy: Query<Entity, With<enemy::Enemy>>,
    stars: Query<Entity, With<Star>>,
    goals: Query<&goal::Goal>,
    map: Query<&tilemap::Map>,
    mut player_collision_reader: EventReader<PlayerCollision>,
    mut change_map_writer: EventWriter<tilemap::ChangeMap>,
) {
    for PlayerCollision {
        player,
        other,
        event_type,
    } in player_collision_reader.iter()
    {
        if player.normals().iter().any(|normal| normal.y >= 0.9) {
            jump.0 = 0;
        }

        let other_entity = other.rigid_body_entity();
        if enemy.get(other_entity).is_ok() {
            match event_type {
                PlayerCollisionEventType::Started => {
                    hit.0 = true;
                    hit_time.0 = Instant::now();
                }
                PlayerCollisionEventType::Stopped => {
                    hit.0 = false;
                }
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

        if goals.get(other_entity).is_ok() {
            let map_component = map.single();
            info!("Goal reached, changing map to {}", map_component.index + 1);

            if env::var_os("CHANGE_MAP").is_some() {
                // TODO: changing map does not work, hangs
                change_map_writer.send(tilemap::ChangeMap {
                    index: (map_component.index + 1) % tilemap::MAPS_COUNT,
                })
            } else {
                app_state.set(AppState::Won).unwrap();
            }
        }
    }
}

fn check_hits(
    hit: ResMut<Hit>,
    mut hit_time: ResMut<HitTime>,
    mut hp: ResMut<Hp>,
    advantage: Res<Advantage>,
    mut app_state: ResMut<State<AppState>>
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

        if hp.0 > bite_strength {
            hp.0 -= bite_strength;
        } else {
            hp.0 = 0;
            app_state.set(AppState::Died).unwrap();
        }

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

fn spawn_stars(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec2,
    textures: &mut ResMut<Assets<TextureAtlas>>,
    animations: &mut ResMut<Assets<SpriteSheetAnimation>>,
) {

    let animation_handle = animations.add(SpriteSheetAnimation::from_range(
        0..=2,
        Duration::from_millis(100),
    ));

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("star.png"),
                Vec2::new(15.0, 15.0),
                3,
                1,
            )),
            transform: Transform {
                translation: Vec3::new(position.x, position.y, 4.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Star)
        .insert(RigidBody::Static)
        .insert(animation_handle)
        .insert(Play)
        .with_children(|children| {
            children.spawn_bundle((SensorShape, CollisionShape::Sphere { radius: 5.0 }));
        });
}

fn on_die(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(10000.0, 10000.0)),
            ..Default::default()
        },
        transform:  Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
        ..Default::default()
    });
    commands.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
            ..Default::default()
        },
        text: Text::with_section(
            "You died",
            TextStyle {
                font: asset_server.load("PublicPixel-0W6DP.ttf"),
                font_size: 30.0,
                color: Color::RED,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    });
}

fn on_win(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::BEIGE,
            custom_size: Some(Vec2::new(10000.0, 10000.0)),
            ..Default::default()
        },
        transform:  Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
        ..Default::default()
    });
    
    commands.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
            ..Default::default()
        },
        text: Text::with_section(
            "You won",
            TextStyle {
                font: asset_server.load("PublicPixel-0W6DP.ttf"),
                font_size: 30.0,
                color: Color::LIME_GREEN,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    });
}
