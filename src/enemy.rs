use super::Hit;
use crate::advantage::{Advantage, EnemyAdvantage};
use crate::player::Player;
use benimator::*;
use bevy::prelude::*;
use heron::*;
use std::time::Duration;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub enum FishType {
    Anglerfish,
    Sawfish,
}

#[derive(Component)]
pub struct Borders {
    left: f32,
    right: f32,
}

#[derive(Component)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Default)]
pub struct Animations {
    a_left: Handle<SpriteSheetAnimation>,
    a_right: Handle<SpriteSheetAnimation>,
    a_bite_left: Handle<SpriteSheetAnimation>,
    a_bite_right: Handle<SpriteSheetAnimation>,
    s_left: Handle<SpriteSheetAnimation>,
    s_right: Handle<SpriteSheetAnimation>,
    s_bite_left: Handle<SpriteSheetAnimation>,
    s_bite_right: Handle<SpriteSheetAnimation>,
}

pub fn spawn_anglerfish(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    animations: &mut ResMut<Assets<SpriteSheetAnimation>>,
    handles: &mut ResMut<Animations>,
    position: Vec2,
) {
    let texture = asset_server.load("enemy.png");
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(64.0, 64.0), 22, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    handles.a_left = animations.add(SpriteSheetAnimation::from_range(
        6..=13,
        Duration::from_millis(100),
    ));

    handles.a_right = animations.add(SpriteSheetAnimation::from_range(
        14..=21,
        Duration::from_millis(100),
    ));

    handles.a_bite_left = animations.add(SpriteSheetAnimation::from_range(
        0..=2,
        Duration::from_millis(100),
    ));

    handles.a_bite_right = animations.add(SpriteSheetAnimation::from_range(
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
                translation: Vec3::new(position.x, position.y, 5.0),
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
        .insert(handles.a_left.clone())
        .insert(Play)
        .insert(Borders{left: position.x - 70.0, right: position.x + 70.0})
        .insert(FishType::Anglerfish);
}

pub fn spawn_sawfish(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    animations: &mut ResMut<Assets<SpriteSheetAnimation>>,
    handles: &mut ResMut<Animations>,
    position: Vec2,
) {
    let texture = asset_server.load("enemy2.png");
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(96.0, 48.0), 12, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    handles.s_left = animations.add(SpriteSheetAnimation::from_range(
        4..=7,
        Duration::from_millis(100),
    ));

    handles.s_right = animations.add(SpriteSheetAnimation::from_range(
        8..=11,
        Duration::from_millis(100),
    ));

    handles.s_bite_left = animations.add(SpriteSheetAnimation::from_range(
        0..=1,
        Duration::from_millis(100),
    ));

    handles.s_bite_right = animations.add(SpriteSheetAnimation::from_range(
        2..=3,
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
                translation: Vec3::new(position.x, position.y, 5.0),
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
        .insert(handles.s_left.clone())
        .insert(Play)
        .insert(Borders{left: position.x - 70.0, right: position.x + 70.0})
        .insert(FishType::Sawfish);
}

pub fn r#move(
    mut enemy: Query<
        (
            &Transform,
            &mut Velocity,
            &mut Direction,
            &mut Handle<SpriteSheetAnimation>,
            &FishType,
            &Borders,
        ),
        With<Enemy>,
    >,
    player: Query<&Transform, With<Player>>,
    animations: Res<Animations>,
    hit: ResMut<Hit>,
    adv: Res<Advantage>,
) {
    let player = player.single();
    let enemy_speed = if matches!(
        adv.as_ref(),
        Advantage::Enemy(EnemyAdvantage::DoubleSpeed)
    ) {
        170.0
    } else {
        100.0
    };

    for (enemy_transform, mut enemy_vel, mut direction, mut animation, fishtype, borders) in enemy.iter_mut() {
        match *direction {
            Direction::Left => enemy_vel.linear[0] = -enemy_speed,
            Direction::Right => enemy_vel.linear[0] = enemy_speed,
        }

        if player.translation.y - enemy_transform.translation.y > -70.0
            && player.translation.y - enemy_transform.translation.y < 70.0
        {
            match (player.translation.x - enemy_transform.translation.x) as i32 {
                -50..=0 if hit.0 => {
                    match *fishtype {
                        FishType::Anglerfish => *animation = animations.a_bite_left.clone(),
                        FishType::Sawfish => *animation = animations.s_bite_left.clone(),
                    }
                    *direction = Direction::Left
                }
                1..=50 if hit.0 => {
                    match *fishtype {
                        FishType::Anglerfish => *animation = animations.a_bite_right.clone(),
                        FishType::Sawfish => *animation = animations.s_bite_right.clone(),
                    }
                    *direction = Direction::Right
                }
                -70..=0 => {
                    match *fishtype {
                        FishType::Anglerfish => *animation = animations.a_left.clone(),
                        FishType::Sawfish => *animation = animations.s_left.clone(),
                    }
                    *direction = Direction::Left
                }
                1..=70 => {
                    match *fishtype {
                        FishType::Anglerfish => *animation = animations.a_right.clone(),
                        FishType::Sawfish => *animation = animations.s_right.clone(),
                    }
                    *direction = Direction::Right
                }
                _ => (),
            }
        }
        
        if enemy_transform.translation.x < borders.left {
            match *fishtype {
                FishType::Anglerfish => *animation = animations.a_right.clone(),
                FishType::Sawfish => *animation = animations.s_right.clone(),
            }
            
            *direction = Direction::Right;
        } else if enemy_transform.translation.x > borders.right {
            match *fishtype {
                FishType::Anglerfish => *animation = animations.a_left.clone(),
                FishType::Sawfish => *animation = animations.s_left.clone(),
            }
            *direction = Direction::Left;
        }
    }
}
