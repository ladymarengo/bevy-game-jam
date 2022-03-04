use super::Hit;
use crate::player::Player;
use benimator::*;
use bevy::prelude::*;
use heron::*;
use std::time::Duration;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Default)]
pub struct Animations {
    left: Handle<SpriteSheetAnimation>,
    right: Handle<SpriteSheetAnimation>,
    bite_left: Handle<SpriteSheetAnimation>,
    bite_right: Handle<SpriteSheetAnimation>,
}

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
    mut handles: ResMut<Animations>,
) {
    let texture = asset_server.load("enemy.png");
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(64.0, 64.0), 22, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    handles.left = animations.add(SpriteSheetAnimation::from_range(
        6..=13,
        Duration::from_millis(100),
    ));

    handles.right = animations.add(SpriteSheetAnimation::from_range(
        14..=21,
        Duration::from_millis(100),
    ));

    handles.bite_left = animations.add(SpriteSheetAnimation::from_range(
        0..=2,
        Duration::from_millis(100),
    ));

    handles.bite_right = animations.add(SpriteSheetAnimation::from_range(
        3..=5,
        Duration::from_millis(100),
    ));

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(200.0, 230.0, 5.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy)
        .insert(Direction::Left)
        .insert(RigidBody::Dynamic)
        .with_children(|children| {
            children.spawn_bundle((
                CollisionShape::Cuboid {
                    half_extends: Vec3::new(20.0 / 2.0, 40.0 / 2.0, 0.0),
                    border_radius: None,
                },
                Transform::default(),
                GlobalTransform::default(),
            ));

            children.spawn_bundle((
                SensorShape,
                CollisionShape::Cuboid {
                    half_extends: Vec3::new(80.0 / 2.0, 40.0 / 2.0, 0.0),
                    border_radius: None,
                },
            ));
        })
        .insert(Velocity::from(Vec3::new(0.0, 0.0, 0.0)))
        .insert(RotationConstraints::lock())
        .insert(PhysicMaterial {
            restitution: 0.2,
            ..Default::default()
        })
        .insert(handles.left.clone())
        .insert(Play);
}

pub fn enemy_move(
    mut enemy: Query<
        (
            &Transform,
            &mut Velocity,
            &mut Direction,
            &mut Handle<SpriteSheetAnimation>,
        ),
        With<Enemy>,
    >,
    player: Query<&Transform, With<Player>>,
    animations: Res<Animations>,
    hit: ResMut<Hit>,
) {
    let player = player.single();

    for (enemy_transform, mut enemy_vel, mut direction, mut animation) in enemy.iter_mut() {
    
        match *direction {
            Direction::Left => enemy_vel.linear[0] = -100.0,
            Direction::Right => enemy_vel.linear[0] = 100.0,
        }
        if enemy_transform.translation.x < 100.0 {
            *animation = animations.right.clone();
            *direction = Direction::Right;
        } else if enemy_transform.translation.x > 300.0 {
            *animation = animations.left.clone();
            *direction = Direction::Left;
        }

        if player.translation.y - enemy_transform.translation.y > -70.0
            && player.translation.y - enemy_transform.translation.y < 70.0
        {
            match (player.translation.x - enemy_transform.translation.x) as i32 {
                -50..=0 if hit.0 => {
                    *animation = animations.bite_left.clone();
                    *direction = Direction::Left
                }
                1..=50 if hit.0 => {
                    *animation = animations.bite_right.clone();
                    *direction = Direction::Right
                }
                -150..=0 => {
                    *animation = animations.left.clone();
                    *direction = Direction::Left
                }
                1..=150 => {
                    *animation = animations.right.clone();
                    *direction = Direction::Right
                }
                _ => (),
            }
        }
    }
}
