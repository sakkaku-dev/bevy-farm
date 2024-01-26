use bevy::prelude::*;

use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_scene)
            .add_systems(Update, camera_follow_player);
    }
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
