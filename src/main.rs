mod camera;
mod movement;
mod player;
mod tilemap;

use bevy::prelude::*;
use movement::MovementPlugin;
use player::PlayerPlugin;
use tilemap::TilemapPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            MovementPlugin,
            PlayerPlugin,
            TilemapPlugin,
        ))
        .run();
}
