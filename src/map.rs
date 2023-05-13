mod tile;

use bevy::{app::AppExit, prelude::*};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Split;

const TILE_PIXEL_SIZE: usize = 16;

#[derive(Debug)]
pub struct BadMapDescriptor;
pub struct MapPugin;

#[derive(Component)]
pub struct TileScale(f32);

#[derive(Resource, Default)]
pub struct MapDescriptor {
    width: usize,
    height: usize,
    tile_scale: usize,
}

impl Plugin for MapPugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_map);
    }
}

trait FromTile {
    fn from_tile(x: usize, y: usize, map_descriptor: &ResMut<MapDescriptor>) -> Self;
}

impl FromTile for Transform {
    #[inline]
    fn from_tile(x: usize, y: usize, map_descriptor: &ResMut<MapDescriptor>) -> Self {
        let tile_size: f32 = (TILE_PIXEL_SIZE * map_descriptor.tile_scale) as f32;

        Self {
            scale: Vec3::splat(map_descriptor.tile_scale as f32),
            translation: Vec3::new(
                tile_size * (x as f32 - (map_descriptor.width as f32 / 2.0) + 0.5),
                // `+ y` solves strange lines in full screen
                (tile_size * ((map_descriptor.height as f32 / 2.0) - y as f32 - 0.5)) + y as f32,
                0.0,
            ),
            ..default()
        }
    }
}

fn parse_map_header_field(
    header_split: &mut Split<&str>,
    header_field: &mut usize,
) -> Result<(), BadMapDescriptor> {
    match header_split.next() {
        Some(value_str) => {
            *header_field = match value_str.parse::<usize>() {
                Ok(number) => number,
                Err(e) => {
                    error!("Bad map header: {}", e);
                    return Err(BadMapDescriptor);
                }
            };
        }
        None => {
            error!("Missing field in map header");
            return Err(BadMapDescriptor);
        }
    };
    Ok(())
}

fn parse_map_header(
    header: String,
    map_descriptor: &mut ResMut<MapDescriptor>,
) -> Result<(), BadMapDescriptor> {
    let mut header_split = header.split(",");

    parse_map_header_field(&mut header_split, &mut map_descriptor.width)?;
    parse_map_header_field(&mut header_split, &mut map_descriptor.height)?;
    parse_map_header_field(&mut header_split, &mut map_descriptor.tile_scale)?;
    Ok(())
}

fn parse_map_descriptor(
    map_file: &File,
    commands: &mut Commands,
    map_descriptor: &mut ResMut<MapDescriptor>,
    texture_atlas_handle: Handle<TextureAtlas>,
) -> Result<(), BadMapDescriptor> {
    for (y, line) in BufReader::new(map_file).lines().enumerate() {
        if let Ok(line) = line {
            if y == 0 {
                parse_map_header(line, map_descriptor)?;
                continue;
            }

            let mut sprite_index;
            for (x, tile_char) in line.chars().enumerate() {
                sprite_index = tile::ctotile(tile_char).unwrap_or_default();
                commands.spawn(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(sprite_index as usize),
                    transform: Transform::from_tile(x, y - 1, &map_descriptor),
                    ..default()
                });
            }
        }
    }
    Ok(())
}

fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut map_descriptor: ResMut<MapDescriptor>,
) {
    let tilemap_handle = asset_server.load("tilemap.png");
    let map_desc_file = match File::open("assets/map.txt") {
        Ok(file) => file,
        Err(error) => {
            error!("Problem opening the map descriptor: {:?}", error);
            app_exit_events.send(AppExit);
            return;
        }
    };

    let texture_atlas = TextureAtlas::from_grid(
        tilemap_handle,
        Vec2::splat(TILE_PIXEL_SIZE as f32),
        12,
        11,
        Some(Vec2::splat(1.0)),
        None,
    );

    match parse_map_descriptor(
        &map_desc_file,
        &mut commands,
        &mut map_descriptor,
        texture_atlases.add(texture_atlas),
    ) {
        Err(_) => {
            app_exit_events.send(AppExit);
            return;
        }
        _ => (),
    }
}
