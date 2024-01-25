mod movement;
mod player;

use bevy::prelude::*;
use movement::MovementPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            MovementPlugin,
            PlayerPlugin,
        ))
        .add_systems(Startup, (spawn_scene))
        .run();
}

fn spawn_scene(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            scale: 0.2,
            ..Default::default()
        },
        ..Default::default()
    });
}
