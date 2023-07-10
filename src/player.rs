use crate::collider::Collider;
use crate::map;
use crate::map::FromTile;
use bevy::prelude::*;

pub struct PlayerPugin;

const PLAYER_INDEX: usize = 84;
pub const PLAYER_SPEED: f32 = 200.;

#[derive(Component, Debug, Default)]
pub struct Player {
    pub direction: Vec2,
    pub velocity: Vec2,
}

impl Plugin for PlayerPugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player.after(map::setup_map))
            .add_system(move_player_system);
    }
}

fn setup_player(mut commands: Commands, map_descriptor: ResMut<map::MapDescriptor>) {
    commands.spawn((
        SpriteBundle {
            texture: map_descriptor.textures[PLAYER_INDEX].clone(),
            transform: Transform::from_tile(
                map_descriptor.width / 2,
                map_descriptor.height / 2 + 2,
                1,
                &map_descriptor,
            ),
            ..default()
        },
        Player { ..default() },
        Collider,
    ));
}

pub fn move_player_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_position: Query<(&mut Player, &mut Transform)>,
) {
    for (mut player, mut transform) in &mut player_position {
        let dt = time.delta_seconds();
        player.direction.x = 0.;
        player.direction.y = 0.;

        if keyboard_input.pressed(KeyCode::Q) {
            player.direction.x = -1.;
        }
        if keyboard_input.pressed(KeyCode::D) {
            player.direction.x = 1.;
        }
        if keyboard_input.pressed(KeyCode::S) {
            player.direction.y = -1.;
        }
        if keyboard_input.pressed(KeyCode::Z) {
            player.direction.y = 1.;
        }

        player.velocity.x = player.direction.x * PLAYER_SPEED;
        player.velocity.y = player.direction.y * PLAYER_SPEED;

        transform.translation.x += player.velocity.x * dt;
        transform.translation.y += player.velocity.y * dt;
    }
}
