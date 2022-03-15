use bevy::math::Affine2;
use bevy::prelude::*;

#[derive(Component)]
pub struct Bubble {
    /// Center of rotation of bubble (world coordinates)
    pub center_position: Vec2,
    /// Offset from rotation center (x, virtual z)
    pub offset_position: Vec2,
    pub lifetime: Timer,
}

#[derive(Bundle)]
pub struct BubbleBundle {
    pub bubble: Bubble,
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub parent: Parent,
}

#[derive(Component)]
pub struct BubbleGenerator {
    pub timer: Timer,
}

impl BubbleBundle {
    fn new(sprite_index: usize, texture_atlas: Handle<TextureAtlas>, parent: Entity) -> Self {
        BubbleBundle {
            bubble: Bubble {
                center_position: Vec2::ZERO,
                offset_position: Vec2::new(0.0, 5.0),
                lifetime: Timer::from_seconds(rand::random::<f32>() * 10.0, false),
            },
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas,
                sprite: TextureAtlasSprite {
                    index: sprite_index,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::ZERO),
                ..Default::default()
            },
            parent: Parent(parent),
        }
    }
}

pub fn process_bubble_generators(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    mut bubble_generators_query: Query<(Entity, &mut BubbleGenerator)>,
) {
    let texture_atlas = textures.add(TextureAtlas::from_grid(
        asset_server.load("bubble.png"),
        Vec2::new(8.0, 8.0),
        2,
        1,
    ));

    for (entity, mut bubble_generator) in bubble_generators_query.iter_mut() {
        bubble_generator.timer.tick(time.delta());
        if bubble_generator.timer.just_finished() {
            commands.spawn_bundle(BubbleBundle::new(
                rand::random::<usize>() % 2,
                texture_atlas.clone(),
                entity.clone(),
            ));
        }
    }
}

pub fn process_bubbles(
    mut commands: Commands,
    mut bubbles_query: Query<(Entity, &mut Bubble, &mut Transform)>,
    time: Res<Time>,
) {
    let center_transform = Affine2::from_translation(Vec2::new(0.0, 0.3));
    let offset_transform = Affine2::from_angle(0.1);

    for (entity, mut bubble, mut transform) in bubbles_query.iter_mut() {
        bubble.lifetime.tick(time.delta());
        if bubble.lifetime.finished() {
            commands.entity(entity).despawn();
            continue;
        }

        bubble.center_position = center_transform.transform_point2(bubble.center_position);
        bubble.offset_position = offset_transform.transform_point2(bubble.offset_position);

        transform.translation = Vec3::new(
            (bubble.center_position.x + bubble.offset_position.x).floor(),
            bubble.center_position.y.floor(),
            transform.translation.z,
        );
    }
}

pub fn spawn_bubble_generator(commands: &mut Commands, position: Vec2) {
    let timer = Timer::from_seconds(rand::random::<f32>() * 4.0 + 3.0, true);
    commands.spawn_bundle((
        BubbleGenerator { timer },
        Transform {
            translation: position.extend(0.0),
            ..Default::default()
        },
        GlobalTransform::default(),
    ));
}
