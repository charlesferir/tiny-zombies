use bevy::prelude::*;

pub struct PlayerPugin;

use crate::map;
use crate::map::FromTile;

const PLAYER_INDEX: usize = 84;

#[derive(Component)]
struct Player;

impl Plugin for PlayerPugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player.after(map::setup_map))
            .add_system(move_player_system);
    }
}

fn setup_player(
    mut commands: Commands,
    main_atlas: Res<map::MainAtlas>,
    map_descriptor: ResMut<map::MapDescriptor>,
) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: main_atlas.handle.clone(),
            sprite: TextureAtlasSprite::new(PLAYER_INDEX),
            transform: Transform::from_tile(
                map_descriptor.width / 2,
                map_descriptor.height / 2 + 1,
                &map_descriptor,
            ),
            ..default()
        },
        Player {},
    ));
}

fn move_player_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_position: Query<(&mut Player, &mut Transform)>,
) {
    for (_, mut transform) in &mut player_position {
        let mut x_dir = 0;
        let mut y_dir = 0;
        let dt = time.delta_seconds();

        if keyboard_input.pressed(KeyCode::Q) {
            x_dir = -1;
        }
        if keyboard_input.pressed(KeyCode::D) {
            x_dir = 1;
        }
        if keyboard_input.pressed(KeyCode::S) {
            y_dir = -1;
        }
        if keyboard_input.pressed(KeyCode::Z) {
            y_dir = 1;
        }

        transform.translation.x += x_dir as f32 * 300. * dt;
        transform.translation.y += y_dir as f32 * 300. * dt;
    }
}
