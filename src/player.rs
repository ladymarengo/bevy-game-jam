use crate::advantage::{Advantage, PlayerAdvantage};
use benimator::*;
use bevy::prelude::*;
use heron::*;
use std::time::Duration;

#[derive(Component)]
pub struct Player;

#[derive(Default)]
pub struct Jump(pub u8);

pub fn spawn(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    animations: &mut ResMut<Assets<SpriteSheetAnimation>>,
    position: Vec2,
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
            transform: Transform::from_translation(Vec3::new(position.x, position.y, 5.0)),
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

pub fn r#move(
    mut commands: Commands,
    mut player: Query<(Entity, &mut Velocity), With<Player>>,
    keys: Res<Input<KeyCode>>,
    mut jump: ResMut<Jump>,
    mut adv: ResMut<Advantage>,
) {
    let (id, mut player) = player.single_mut();
    let max_jumps = if matches!(adv.as_ref(), Advantage::Player(PlayerAdvantage::DoubleJump)) {
        2
    } else {
        1
    };

    commands.entity(id).remove::<Play>();
    let can_jump = jump.0 < max_jumps;
    let is_not_jumping = jump.0 == 0;

    if keys.just_pressed(KeyCode::W) && can_jump {
        player.linear[1] = 600.0;
        jump.0 += 1;
    }
    if keys.pressed(KeyCode::A) {
        player.linear[0] = -200.0;
        if is_not_jumping {
            commands.entity(id).insert(Play);
        }
    }
    if keys.pressed(KeyCode::D) {
        player.linear[0] = 200.0;
        if is_not_jumping {
            commands.entity(id).insert(Play);
        }
    }
    if keys.pressed(KeyCode::S) {
        player.linear[1] = -400.0;
    }

    if option_env!("CHEATS").is_some() && keys.just_pressed(KeyCode::R) {
        *adv = Advantage::random();
    }
}
