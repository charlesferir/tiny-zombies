use bevy::prelude::*;

mod map;

use map::MapPugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            118.0 / 255.0,
            59.0 / 255.0,
            54.0 / 255.0,
        )))
        .init_resource::<map::MapDescriptor>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(MapPugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
