use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
		.add_startup_system(set_window_resolution)
		.add_startup_system(init)
		.add_startup_system(spawn_enemy)
		.add_system(enemy_move)
        .run()
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
enum Direction
{
	Left,
	Right,
}

#[derive(Component)]
struct MainCamera;

fn init(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
	.insert(MainCamera);
}

fn set_window_resolution(mut windows: ResMut<Windows>) {
    windows
        .get_primary_mut()
        .unwrap()
        .set_resolution(1024.0, 860.0);
}

fn spawn_enemy(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(200.0, -100.0, 0.0),
                scale: Vec3::new(50.0, 100.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy)
		.insert(Direction::Left);
}

fn enemy_move(mut enemy: Query<(&mut Transform, &mut Direction), With<Enemy>>, wnds: Res<Windows>,
q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>) {
	let (mut enemy, mut direction) = enemy.single_mut();
	match *direction {
		Direction::Left => enemy.translation.x -= 5.0,
		Direction::Right => enemy.translation.x += 5.0,
	}
	if enemy.translation.x < -100.0 {
		*direction = Direction::Right;
	} else if enemy.translation.x > 300.0 {
		*direction = Direction::Left;
	}

	// get cursor coordinates

	let (camera, camera_transform) = q_camera.single();
    let wnd = wnds.get(camera.window).unwrap();
    
	if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec2 = world_pos.truncate();

        // eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);
		match (world_pos.x - enemy.translation.x) as i32 {
			-200..=0 => *direction = Direction::Left,
			1..=200 => *direction = Direction::Right,
			_ => (),
		}
    }
}
