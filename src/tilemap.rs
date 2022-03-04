use benimator::*;
use bevy::prelude::*;
use heron::*;

pub const TILE_SIZE: usize = 16;

const TILESET_ASSET: &str = "terrain.png";
static TILEMAPS_TMX: [&[u8]; 1] = [include_bytes!("../assets/levels/level3.tmx")];

const COLLISION_LAYER_NAME: &str = "collision";
const OBJ_TYPE_PLAYER_START: &str = "player_start";

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

fn clear_map(commands: &mut Commands, map_query: &Query<Entity, With<Map>>) {
    let map = map_query
        .get_single()
        .expect("Map must be loaded (and only single instance)");
    commands.entity(map).despawn_recursive();
}

pub fn load_initial_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    load_map(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &mut animations,
        0,
    );
}

pub fn change_map(
    commands: &mut Commands,
    map_query: &Query<Entity, With<Map>>,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    animations: &mut ResMut<Assets<SpriteSheetAnimation>>,
    index: usize,
) {
    clear_map(commands, map_query);
    load_map(commands, asset_server, texture_atlases, animations, index);
}

fn load_map(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    animations: &mut ResMut<Assets<SpriteSheetAnimation>>,
    index: usize,
) {
    let map = tiled::parse(TILEMAPS_TMX[index]).unwrap();
    let texture_atlas_handle = create_tilemap_atlas(asset_server, texture_atlases);

    let mut collision_tiles = CollisionTiles::new(map.width as usize, map.height as usize);

    let width = map.width as usize;
    let height = map.height as usize;

    let map_entity = commands
        .spawn()
        .insert(Map { width, height })
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .id();

    let mut layer_index = 0;
    for layer in map.layers {
        layer_index += 1;
        if layer.name == "water" {
            continue;
        }
        let is_collision_layer = layer.name == COLLISION_LAYER_NAME;
        if let tiled::LayerData::Finite(tiles) = layer.tiles {
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
    for object_group in map.object_groups {
        for object in object_group.objects {
            if object.obj_type == OBJ_TYPE_PLAYER_START {
                crate::player::spawn(
                    commands,
                    &asset_server,
                    texture_atlases,
                    animations,
                    Vec2::new(object.x, object.y),
                );
                has_player_start = true;
            }
        }
    }
    if !has_player_start {
        panic!("player_start not found in level #{index}");
    }

    commands.insert_resource(collision_tiles);
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
        (col * TILE_SIZE) as f32,
        ((height - row - 1) * TILE_SIZE) as f32,
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
