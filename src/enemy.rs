use super::Player;
use bevy::prelude::*;
use heron::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub enum Direction {
    Left,
    Right,
}

pub fn spawn_enemy(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(200.0, 230.0, 5.0),
                scale: Vec3::new(20.0, 40.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy)
        .insert(Direction::Left)
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(20.0 / 2.0, 40.0 / 2.0, 0.0),
            border_radius: None,
        })
        .insert(Velocity::from(Vec3::new(0.0, 0.0, 0.0)))
        .insert(RotationConstraints::lock())
        .insert(PhysicMaterial {
            restitution: 0.2,
            ..Default::default()
        });
}

pub fn enemy_move(
    mut enemy: Query<(&Transform, &mut Velocity, &mut Direction), With<Enemy>>,
    player: Query<&Transform, With<Player>>,
) {
    let (enemy_transform, mut enemy_vel, mut direction) = enemy.single_mut();
    let player = player.single();
    match *direction {
        Direction::Left => enemy_vel.linear[0] = -100.0,
        Direction::Right => enemy_vel.linear[0] = 100.0,
    }
    if enemy_transform.translation.x < 100.0 {
        *direction = Direction::Right;
    } else if enemy_transform.translation.x > 300.0 {
        *direction = Direction::Left;
    }

    if player.translation.y - enemy_transform.translation.y > -70.0
        && player.translation.y - enemy_transform.translation.y < 70.0
    {
        match (player.translation.x - enemy_transform.translation.x) as i32 {
            -150..=0 => *direction = Direction::Left,
            1..=150 => *direction = Direction::Right,
            _ => (),
        }
    }
}
