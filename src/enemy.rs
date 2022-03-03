use bevy::prelude::*;
use heron::*;
use super::Player;
use benimator::*;
use std::time::Duration;

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_system(bevy::input::system::exit_on_esc_system)
// 		.add_startup_system(set_window_resolution)
// 		.add_startup_system(init)
// 		.add_startup_system(spawn_enemy)
// 		.add_system(enemy_move)
//         .run()
// }

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub enum Direction
{
	Left,
	Right,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Default)]
pub struct Animations {
    left: Handle<SpriteSheetAnimation>,
    right: Handle<SpriteSheetAnimation>,
}

// fn init(mut commands: Commands) {
//     commands.spawn_bundle(OrthographicCameraBundle::new_2d())
// 	.insert(MainCamera);
// }

// fn set_window_resolution(mut windows: ResMut<Windows>) {
//     windows
//         .get_primary_mut()
//         .unwrap()
//         .set_resolution(1024.0, 860.0);
// }

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
    mut handles: ResMut<Animations>) {
    let texture = asset_server.load("enemy.png");
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(64.0, 64.0), 16, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    handles.left = animations.add(SpriteSheetAnimation::from_range(
        0..=7,
        Duration::from_millis(100),
    ));

    handles.right = animations.add(SpriteSheetAnimation::from_range(
        9..=15,
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
                // scale: Vec3::new(20.0, 40.0, 0.0),
                ..Default::default()
            },
            // sprite: Sprite {
            //     color: Color::rgb(1.0, 0.0, 0.0),
            //     ..Default::default()
            // },
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
            ..Default::default()})
        .insert(handles.left.clone())
        .insert(Play);
}

pub fn enemy_move(mut enemy: Query<(&Transform, &mut Velocity, &mut Direction, &mut Handle<SpriteSheetAnimation>), With<Enemy>>, player: Query<&Transform, With<Player>>, animations: Res<Animations>) {
	let (enemy_transform, mut enemy_vel, mut direction, mut animation) = enemy.single_mut();
    let player = player.single();
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

	// get cursor coordinates. Will change later to the main character coordinates

	// let (camera, camera_transform) = q_camera.single();
    // let wnd = wnds.get(camera.window).unwrap();
    
	// if let Some(screen_pos) = wnd.cursor_position() {
    //     let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
    //     let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
    //     let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
    //     let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    //     let world_pos: Vec2 = world_pos.truncate();

    //     // eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);
	// 	if world_pos.y - enemy.translation.y > -200.0 && world_pos.y - enemy.translation.y < 200.0 {
	// 		match (world_pos.x - enemy.translation.x) as i32 {
	// 			-200..=0 => *direction = Direction::Left,
	// 			1..=200 => *direction = Direction::Right,
	// 			_ => (),
	// 		}
	// 	}
    // }
    if player.translation.y - enemy_transform.translation.y > -70.0 && player.translation.y - enemy_transform.translation.y < 70.0 {
        match (player.translation.x - enemy_transform.translation.x) as i32 {
            -150..=0 => {
                *animation = animations.left.clone();
                *direction = Direction::Left},
            1..=150 => {
                *animation = animations.right.clone();
                *direction = Direction::Right},
            _ => (),
        }
    }
}
