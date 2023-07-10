use bevy::{prelude::*, window::PresentMode};
use bevy_prototype_debug_lines::*;
use collider::ColliderPugin;
use map::MapPugin;
use player::PlayerPugin;

mod collider;
mod map;
mod player;
mod utils;

#[derive(Resource, Default)]
pub struct DebugMode {
    pub on: bool,
}

fn main() {
    App::new()
        .init_resource::<DebugMode>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Tiny zombies".into(),
                        resolution: (1400., 800.).into(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugin(MapPugin)
        .add_plugin(PlayerPugin)
        .add_plugin(ColliderPugin)
        .add_plugin(DebugLinesPlugin::default())
        .add_startup_system(setup)
        .add_system(toggle_vsync)
        .add_system(toggle_debug_mode)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn toggle_vsync(input: Res<Input<KeyCode>>, mut windows: Query<&mut Window>) {
    if input.just_pressed(KeyCode::V) {
        let mut window = windows.single_mut();

        window.present_mode = if matches!(window.present_mode, PresentMode::AutoVsync) {
            PresentMode::AutoNoVsync
        } else {
            PresentMode::AutoVsync
        };
        info!("PRESENT_MODE: {:?}", window.present_mode);
    }
}

fn toggle_debug_mode(input: Res<Input<KeyCode>>, mut debug_mode: ResMut<DebugMode>) {
    if input.just_pressed(KeyCode::B) {
        debug_mode.on = !debug_mode.on;
        info!(
            "DEBUG_MODE: {}",
            match debug_mode.on {
                true => "ON",
                false => "OFF",
            }
        );
    }
}
