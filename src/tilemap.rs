use bevy::prelude::*;
use std::path::Path;

pub const TILE_SIZE: usize = 16;

// TODO: get assets path from asset system, or even use asset system for loading
const MAP_PATH: &str = "assets/terrain.tmx";
const TILESET_ASSET: &str = "terrain.png";

const COLLISION_LAYER_NAME: &str = "collision";

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

#[derive(Component)]
pub struct TilemapContainer {
    pub width: usize,
    pub height: usize,
}

pub fn load_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let map = tiled::parse_file(Path::new(MAP_PATH)).unwrap();
    let texture = asset_server.load(TILESET_ASSET);
    let texture_atlas = TextureAtlas::from_grid(
        texture,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        TILESET_WIDTH,
        TILESET_HEIGHT,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut collision_tiles = CollisionTiles::new(map.width as usize, map.height as usize);

    let width = map.width as usize;
    let height = map.height as usize;

    let tilemap_container = commands
        .spawn()
        .insert(TilemapContainer { width, height })
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .id();

    let mut layer_index = 0;
    for layer in map.layers {
        layer_index += 1;
        let is_collision_layer = layer.name == COLLISION_LAYER_NAME;
        if let tiled::LayerData::Finite(tiles) = layer.tiles {
            for row in 0..height {
                for col in 0..width {
                    let tile = tiles[row][col];

                    if tile.gid != 0 {
                        create_tile_sprite(
                            &mut commands,
                            tilemap_container.clone(),
                            texture_atlas_handle.clone(),
                            height,
                            row,
                            col,
                            layer_index,
                            tile.gid,
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

    commands.insert_resource(collision_tiles);
}

fn create_tile_sprite(
    commands: &mut Commands,
    tilemap_container: Entity,
    texture_atlas_handle: Handle<TextureAtlas>,
    height: usize,
    row: usize,
    col: usize,
    order: u32,
    tile_id: u32,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: (tile_id as usize) - 1,
                ..Default::default()
            },
            transform: Transform::from_xyz(
                (col * TILE_SIZE) as f32,
                ((height - row - 1) * TILE_SIZE) as f32,
                order as f32,
            ),
            ..Default::default()
        })
        .insert(Parent(tilemap_container));
}
