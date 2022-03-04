use bevy::prelude::*;
use heron::prelude::*;

#[derive(Component)]
pub struct Goal;

pub fn spawn(commands: &mut Commands, position: Vec2, size: Vec2) {
    commands.spawn_bundle((
        RigidBody::Sensor,
        Transform {
            translation: position.extend(10.0),
            scale: size.extend(0.0),
            ..Default::default()
        },
        GlobalTransform::default(),
        CollisionShape::Cuboid {
            half_extends: Vec3::new(size.x / 2.0, size.y / 2.0, 0.0),
            border_radius: None,
        },
        Goal,
    ));
}
