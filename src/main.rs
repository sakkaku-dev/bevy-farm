mod movement;
mod player;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use movement::MovementPlugin;
use player::{Player, PlayerPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            MovementPlugin,
            PlayerPlugin,
            LdtkPlugin,
        ))
        .add_systems(Startup, spawn_scene)
        .add_systems(Update, camera_follow_player)
        .insert_resource(LevelSelection::Uid(0))
        .run();
}

fn spawn_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            scale: 0.2,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("tilemap/main.ldtk"),
        ..Default::default()
    });
}

fn camera_follow_player(
    mut query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player) = player_query.get_single() else {
        return;
    };
    let Ok(mut camera) = query.get_single_mut() else {
        return;
    };

    camera.translation = player.translation;
}
