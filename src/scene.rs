use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};

pub struct ScenePlugin;
const TILE_COUNT: u32 = 34;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .add_systems(Startup, setup_scene)
            .add_systems(Update, random);
    }
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("scene/scene_tilesheet.png");

    let map_size = TilemapSize { x: 320, y: 320 };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    for x in 0..320u32 {
        for y in 0..320u32 {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        ..Default::default()
                    },
                    LastUpdate::default(),
                ))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, -1.0),
        ..Default::default()
    });
}

#[derive(Default, Component)]
struct LastUpdate {
    value: f64,
}

// In this example it's better not to use the default `MapQuery` SystemParam as
// it's faster to do it this way:
fn random(time: ResMut<Time>, mut query: Query<(&mut TileTextureIndex, &mut LastUpdate)>) {
    let current_time = time.elapsed_seconds_f64();
    let mut random = thread_rng();
    for (mut tile, mut last_update) in query.iter_mut() {
        if (current_time - last_update.value) > 1.0 {
            tile.0 = random.gen_range(0..TILE_COUNT);
            last_update.value = current_time;
        }
    }
}
