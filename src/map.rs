mod tile;

use crate::collider::Collider;
use crate::player::Player;
use crate::DebugMode;
use bevy::{app::AppExit, prelude::*};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Split;

pub const TILE_PIXEL_SIZE: usize = 16;
pub const CLEAR_COLOR: Color = Color::rgb(118.0 / 255.0, 59.0 / 255.0, 54.0 / 255.0);

#[derive(Debug)]
pub struct BadMapDescriptor;
pub struct MapPugin;

#[derive(Component)]
pub struct TileScale(f32);

#[derive(Resource, Default)]
pub struct MapDescriptor {
    pub width: usize,
    pub height: usize,
    pub tile_scale: usize,
    pub textures: Vec<Handle<Image>>,
}

impl Plugin for MapPugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(CLEAR_COLOR))
            .init_resource::<MapDescriptor>()
            .add_startup_system(setup_map);
    }
}

pub trait FromTile {
    fn from_tile(x: usize, y: usize, z: usize, map_descriptor: &ResMut<MapDescriptor>) -> Self;
}

impl FromTile for Transform {
    fn from_tile(x: usize, y: usize, z: usize, map_descriptor: &ResMut<MapDescriptor>) -> Self {
        let tile_size: f32 = (TILE_PIXEL_SIZE * map_descriptor.tile_scale) as f32;

        Self {
            scale: Vec3::splat(map_descriptor.tile_scale as f32),
            translation: Vec3::new(
                tile_size * (x as f32 - (map_descriptor.width as f32 / 2.0) + 0.5),
                tile_size * ((map_descriptor.height as f32 / 2.0) - y as f32 - 0.5),
                z as f32,
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
) -> Result<(), BadMapDescriptor> {
    for (y, line) in BufReader::new(map_file).lines().enumerate() {
        if let Ok(line) = line {
            if y == 0 {
                parse_map_header(line, map_descriptor)?;
                continue;
            }

            // let mut sprite_index;
            for (x, tile_char) in line.chars().enumerate() {
                let mut tile = commands.spawn(SpriteBundle {
                    texture: map_descriptor.textures
                        [tile::ctotile(tile_char).unwrap_or_default() as usize]
                        .clone(),
                    transform: Transform::from_tile(x, y - 1, 0, &map_descriptor),
                    ..default()
                });
                if tile::collide(tile_char) {
                    tile.insert(Collider);
                }
            }
        }
    }
    Ok(())
}

pub fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut app_exit_events: EventWriter<AppExit>,
    mut map_descriptor: ResMut<MapDescriptor>,
) {
    map_descriptor.textures = (0..131)
        .map(|tile_index| {
            let path = format!("Tiles/tile_{tile_index:0>4}.png");
            asset_server.load::<Image, String>(path)
        })
        .collect();

    let map_desc_file = match File::open("assets/map.txt") {
        Ok(file) => file,
        Err(error) => {
            error!("Problem opening the map descriptor: {:?}", error);
            app_exit_events.send(AppExit);
            return;
        }
    };
    match parse_map_descriptor(&map_desc_file, &mut commands, &mut map_descriptor) {
        Err(_) => {
            app_exit_events.send(AppExit);
            return;
        }
        _ => (),
    }
}
