use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Fence;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct FenceBundle {
    fence: Fence,
}

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .add_systems(Startup, spawn_map)
            .insert_resource(LevelSelection::Uid(0));
    }
}

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("tilemap/main.ldtk"),
        ..Default::default()
    });
}
