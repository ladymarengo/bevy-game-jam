use benimator::*;
use bevy::prelude::*;
use heron::*;
use rand::Rng;

pub const TILE_SIZE: usize = 16;

const TILESET_ASSET: &str = "terrain.png";
static TILEMAPS_TMX: &[&[u8]] = &[
    include_bytes!("../assets/levels/level3.tmx"),
    include_bytes!("../assets/levels/level2.tmx"),
];
pub const MAPS_COUNT: usize = 2;

const COLLISION_LAYER_NAME: &str = "collision";
const OBJ_TYPE_PLAYER_START: &str = "player_start";
const OBJ_TYPE_ANGLERFISH: &str = "anglerfish";
const OBJ_TYPE_SAWFISH: &str = "sawfish";
const OBJ_TYPE_STAR: &str = "star";
const OBJ_TYPE_GOAL: &str = "goal";
const OBJ_TYPE_BUBBLE_GENERATOR: &str = "bubble_generator";

const TILESET_WIDTH: usize = 16;
const TILESET_HEIGHT: usize = 5;

#[derive(Clone)]
pub enum CollisionTile {
    Empty,
    Full,
}

pub struct CollisionTiles {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<CollisionTile>>,
}

impl CollisionTiles {
    fn new(width: usize, height: usize) -> Self {
        let row = vec![CollisionTile::Empty; width];
        let mut tiles = Vec::with_capacity(height);
        for _ in 0..height {
            tiles.push(row.clone());
        }
        CollisionTiles {
            width,
            height,
            tiles,
        }
    }
}

#[derive(Component, Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub index: usize,
}

pub struct ChangeMap {
    pub index: usize,
}

fn create_tilemap_atlas(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    let texture = asset_server.load(TILESET_ASSET);
    let texture_atlas = TextureAtlas::from_grid(
        texture,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        TILESET_WIDTH,
        TILESET_HEIGHT,
    );
    texture_atlases.add(texture_atlas)
}

fn clear_map(
    commands: &mut Commands,
    map_query: &Query<Entity, With<Map>>,
    player_query: &Query<Entity, With<crate::player::Player>>,
    enemy_query: &Query<Entity, With<crate::enemy::Enemy>>,
) {
    let map = map_query
        .get_single()
        .expect("Map must be loaded (and only single instance)");
    commands.entity(map).despawn_recursive();
    for entity in player_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn();
    }

    info!("Cleared map");
}

pub fn load_initial_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
    mut animation_handles: ResMut<crate::enemy::Animations>,
) {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..2);
    load_map(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &mut animations,
        &mut animation_handles,
        index,
    );
}

pub fn handle_change_map(
    mut commands: Commands,
    map_query: Query<Entity, With<Map>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
    mut animation_handles: ResMut<crate::enemy::Animations>,
    player_query: Query<Entity, With<crate::player::Player>>,
    enemy_query: Query<Entity, With<crate::enemy::Enemy>>,
    mut change_map_reader: EventReader<ChangeMap>,
) {
    for event in change_map_reader.iter() {
        clear_map(&mut commands, &map_query, &player_query, &enemy_query);
        load_map(
            &mut commands,
            &asset_server,
            &mut texture_atlases,
            &mut animations,
            &mut animation_handles,
            event.index,
        );
    }
}

fn load_map(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    animations: &mut ResMut<Assets<SpriteSheetAnimation>>,
    animation_handles: &mut ResMut<crate::enemy::Animations>,
    index: usize,
) {
    let map = tiled::parse(TILEMAPS_TMX[index]).unwrap();
    let texture_atlas_handle = create_tilemap_atlas(asset_server, texture_atlases);

    let mut collision_tiles = CollisionTiles::new(map.width as usize, map.height as usize);

    let width = map.width as usize;
    let height = map.height as usize;

    let map_entity = commands
        .spawn()
        .insert(Map {
            width,
            height,
            index,
        })
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .id();

    let mut layer_index = 0;
    for layer in &map.layers {
        layer_index += 1;
        if layer.name == "water" {
            continue;
        }
        let is_collision_layer = layer.name == COLLISION_LAYER_NAME;
        if let tiled::LayerData::Finite(tiles) = &layer.tiles {
            for row in 0..height {
                for col in 0..width {
                    let tile = tiles[row][col];

                    if tile.gid != 0 {
                        create_tile_sprite(
                            commands,
                            map_entity.clone(),
                            texture_atlas_handle.clone(),
                            height,
                            row,
                            col,
                            layer_index,
                            tile.gid,
                            is_collision_layer,
                        );

                        if is_collision_layer {
                            collision_tiles.tiles[row][col] = CollisionTile::Full;
                        }
                    }
                }
            }
        } else {
            unimplemented!();
        }
    }

    let mut has_player_start = false;
    for object_group in &map.object_groups {
        for object in &object_group.objects {
            if object.obj_type == OBJ_TYPE_PLAYER_START {
                crate::player::spawn(
                    commands,
                    &asset_server,
                    texture_atlases,
                    animations,
                    position_tmx_to_world(&map, object),
                );
                has_player_start = true;
            } else if object.obj_type == OBJ_TYPE_ANGLERFISH {
                crate::enemy::spawn_anglerfish(
                    commands,
                    asset_server,
                    texture_atlases,
                    animations,
                    animation_handles,
                    position_tmx_to_world(&map, object),
                );
            } else if object.obj_type == OBJ_TYPE_SAWFISH {
                crate::enemy::spawn_sawfish(
                    commands,
                    asset_server,
                    texture_atlases,
                    animations,
                    animation_handles,
                    position_tmx_to_world(&map, object),
                );
            } else if object.obj_type == OBJ_TYPE_STAR {
                crate::spawn_stars(
                    commands,
                    asset_server,
                    position_tmx_to_world(&map, object),
                    texture_atlases,
                    animations,
                );
            } else if object.obj_type == OBJ_TYPE_GOAL {
                if let tiled::ObjectShape::Rect { width, height } = object.shape {
                    crate::goal::spawn(
                        commands,
                        position_tmx_to_world(&map, object),
                        Vec2::new(width, height),
                    );
                } else {
                    panic!("Invalid goal shape, must be rectangle");
                }
            } else if object.obj_type == OBJ_TYPE_BUBBLE_GENERATOR {
                crate::bubble::spawn_bubble_generator(
                    commands,
                    position_tmx_to_world(&map, object),
                );
            }
        }
    }
    if !has_player_start {
        panic!("player_start not found in level #{index}");
    }

    commands.insert_resource(collision_tiles);
    info!("Loaded map {}", index);
}

fn create_tile_sprite(
    commands: &mut Commands,
    map: Entity,
    texture_atlas_handle: Handle<TextureAtlas>,
    height: usize,
    row: usize,
    col: usize,
    order: u32,
    tile_id: u32,
    has_collision: bool,
) {
    let position = Vec2::new(
        (col * TILE_SIZE) as f32 + (TILE_SIZE as f32 / 2.0),
        ((height - row) * TILE_SIZE) as f32 - (TILE_SIZE as f32 / 2.0),
    );
    let mut entity = commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        sprite: TextureAtlasSprite {
            index: (tile_id as usize) - 1,
            ..Default::default()
        },
        transform: Transform::from_xyz(position.x, position.y, order as f32),
        ..Default::default()
    });
    entity.insert(Parent(map));
    if has_collision {
        entity.insert(RigidBody::Static);
        entity.insert(CollisionShape::Cuboid {
            half_extends: (Vec3::new(TILE_SIZE as f32 / 2.0, TILE_SIZE as f32 / 2.0, 0.0)),
            border_radius: None,
        });
    }
}

fn position_tmx_to_world(map: &tiled::Map, object: &tiled::Object) -> Vec2 {
    let map_height = (map.height * (TILE_SIZE as u32)) as f32;

    Vec2::new(
        object.x + (object.width / 2.0),
        map_height - object.y - (object.height / 2.0),
    )
}
