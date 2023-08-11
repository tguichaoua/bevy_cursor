use bevy::math::Vec4Swizzles;
use bevy::prelude::*;
use bevy_cursor::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

mod utils;
use utils::change_detection::DetectChangesMutExt;

const MAP_SIZE: TilemapSize = TilemapSize { x: 20, y: 20 };

fn main() {
    App::new()
        //
        .add_event::<TileHoverEvent>()
        //
        .insert_resource(TilemapRenderSettings {
            y_sort: true,
            render_chunk_size: UVec2::new(MAP_SIZE.x, 1),
        })
        .init_resource::<HoveredTile>()
        //
        .add_plugins((
            DefaultPlugins,
            CursorInfoPlugin,
            TilemapPlugin,
            PanCamPlugin,
        ))
        //
        .add_systems(Startup, setup)
        .add_systems(
            First,
            update_hovered_tile
                .after(UpdateCursorInfo)
                .run_if(resource_changed::<CursorInfo>()),
        )
        .add_systems(Update, colorize_tile_on_hover)
        .run();
}

// =============================================================================

/// The currently hovered tile entity, if any.
#[derive(Resource, Default)]
pub struct HoveredTile(pub Option<Entity>);

/// Event emitted when the cursor enter or leave a tile.
#[derive(Event)]
pub enum TileHoverEvent {
    Leave(Entity),
    Enter(Entity),
}

/// The original [`TileTextureIndex`] value of the tile.
#[derive(Component)]
pub struct BaseTileTextureIndex(TileTextureIndex);

// =============================================================================

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    array_texture_loader: Res<ArrayTextureLoader>,
) {
    // Spawn a camera
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());

    // Spawn a tilemap
    let texture_handle: Handle<Image> = asset_server.load("isometric-sheet.png");
    let texture = TilemapTexture::Single(texture_handle);

    let map_size = MAP_SIZE;
    let grid_size = TilemapGridSize { x: 64.0, y: 32.0 };
    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);
    let tile_size = TilemapTileSize { x: 64.0, y: 64.0 };

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let position = TilePos { x, y };

            let texture_index = 5;

            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(texture_index),
                        ..default()
                    },
                    BaseTileTextureIndex(TileTextureIndex(texture_index)),
                ))
                .id();

            tile_storage.set(&position, tile_entity);
        }
    }

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        map_type,
        storage: tile_storage,
        texture: texture.clone(),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..default()
    });

    array_texture_loader.add(TilemapArrayTexture {
        texture,
        tile_size,
        ..default()
    });
}

fn update_hovered_tile(
    cursor: Res<CursorInfo>,
    hovered_tile: ResMut<HoveredTile>,
    mut hover_tile_event_writer: EventWriter<TileHoverEvent>,

    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
) {
    let mut hovered_tile = hovered_tile.map_unchanged(|x| &mut x.0);

    if let Some(cursor_position) = cursor.position() {
        for (map_size, grid_size, map_type, tile_storage, map_transform) in tilemap_q.iter() {
            // We need to make sure that the cursor's world position is correct relative to the map
            // due to any map transformation.
            let cursor_in_map_pos: Vec2 = {
                // Extend the cursor_pos vec3 by 0.0 and 1.0
                let cursor_pos = Vec4::from((cursor_position, 0.0, 1.0));
                let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
                cursor_in_map_pos.xy()
            };

            // Fix the gap due the dimond grid.
            let cursor_in_map_pos = cursor_in_map_pos - Vec2::new(0.0, grid_size.y / 2.0);

            // Once we have a world position we can transform it into a possible tile position.
            if let Some(tile_pos) =
                TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
            {
                if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                    if let Some(previous_tile) = hovered_tile.replace_if_neq(Some(tile_entity)) {
                        if let Some(previous_tile) = previous_tile {
                            hover_tile_event_writer.send(TileHoverEvent::Leave(previous_tile));
                        }
                        hover_tile_event_writer.send(TileHoverEvent::Enter(tile_entity));
                    }

                    return;
                }
            }
        }
    }

    // If the cursor is not in any window or didn't hover a tile, set the value to None.
    if let Some(Some(previous_tile)) = hovered_tile.replace_if_neq(None) {
        hover_tile_event_writer.send(TileHoverEvent::Leave(previous_tile));
    }
}

pub fn colorize_tile_on_hover(
    mut query: Query<(&mut TileTextureIndex, &BaseTileTextureIndex)>,
    mut tile_hovered_event: EventReader<TileHoverEvent>,
) {
    for event in tile_hovered_event.iter() {
        match event {
            TileHoverEvent::Leave(tile) => match query.get_mut(*tile) {
                Ok((mut index, base_index)) => {
                    *index = base_index.0;
                }
                Err(error) => error!("{error}"),
            },
            TileHoverEvent::Enter(tile) => match query.get_mut(*tile) {
                Ok((mut index, _)) => index.0 = 3,
                Err(error) => error!("{error}"),
            },
        }
    }
}
