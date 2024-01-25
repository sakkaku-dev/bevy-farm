use bevy::prelude::*;

#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_scene, spawn_player))
        .add_systems(Update, (update_position, print_position))
        .run();
}

fn spawn_scene(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        ..Default::default()
    });
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(150., 150.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Velocity {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        },
    ));
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
        transform.translation.z += velocity.z;
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!(
            "Entity {:?} is at position {:?}",
            entity, transform.translation
        )
    }
}
