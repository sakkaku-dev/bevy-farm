use crate::movement::*;
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 2.0;
const PLAYER_SPRITESHEET: &str = "player.png";
const FRAMES_PER_ANIMATION: usize = 2;
const NUM_OF_ANIMATIONS: usize = 2;
const NUM_OF_DIRECTIONS: usize = 4;

#[derive(Component)]
struct Player {
    speed: f32,
    face_dir: Direction,
    anim: PlayerAnimation,
}

#[derive(Component, Deref, DerefMut)]
struct PlayerAnimationTimer(Timer);

#[derive(Clone, Copy)]
enum Direction {
    BOT,
    TOP,
    LEFT,
    RIGHT,
}

#[derive(Clone, Copy, Reflect)]
enum PlayerAnimation {
    IDLE = 0,
    RUN = 2,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, player_face, player_animation));
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    let player_spritesheet_handle = asset_server.load(PLAYER_SPRITESHEET);
    let player_spritesheet_atlas = TextureAtlas::from_grid(
        player_spritesheet_handle,
        Vec2::new(48.0, 48.0),
        NUM_OF_ANIMATIONS * FRAMES_PER_ANIMATION,
        NUM_OF_DIRECTIONS,
        None,
        None,
    );
    let player_spritesheet = textures.add(player_spritesheet_atlas);

    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: player_spritesheet,
            ..Default::default()
        })
        .insert(PlayerAnimationTimer(Timer::from_seconds(
            0.2,
            TimerMode::Repeating,
        )))
        .insert(Velocity::default())
        .insert(Player {
            speed: PLAYER_SPEED,
            face_dir: Direction::BOT,
            anim: PlayerAnimation::IDLE,
        });
}

fn player_animation(
    time: Res<Time>,
    mut query: Query<(&mut PlayerAnimationTimer, &mut TextureAtlasSprite, &Player)>,
) {
    if let Ok((mut timer, mut sprite, player)) = query.get_single_mut() {
        let frame_offset_x = (player.anim as usize);
        let frame_offset_y = (player.face_dir as usize) * NUM_OF_ANIMATIONS * FRAMES_PER_ANIMATION;
        let mut frame = sprite.index % FRAMES_PER_ANIMATION;

        timer.tick(time.delta());
        if timer.just_finished() {
            frame = (sprite.index + 1) % FRAMES_PER_ANIMATION;
        }

        sprite.index = frame_offset_y + frame_offset_x + frame;
    }
}

fn player_face(mut query: Query<(&Velocity, &mut Player)>) {
    if let Ok((vel, mut player)) = query.get_single_mut() {
        if vel.x == 0. && vel.y == 0. {
            player.anim = PlayerAnimation::IDLE;
        } else {
            player.anim = PlayerAnimation::RUN;
        }

        if vel.x > 0. {
            player.face_dir = Direction::RIGHT;
        } else if vel.x < 0. {
            player.face_dir = Direction::LEFT;
        } else if vel.y > 0. {
            player.face_dir = Direction::TOP;
        } else if vel.y < 0. {
            player.face_dir = Direction::BOT;
        }
    }
}

fn move_player(keys: Res<Input<KeyCode>>, mut query: Query<(&mut Velocity, &Player)>) {
    if let Ok((mut vel, player)) = query.get_single_mut() {
        let up = if keys.pressed(KeyCode::W) { 1. } else { 0. };
        let left = if keys.pressed(KeyCode::A) { 1. } else { 0. };
        let down = if keys.pressed(KeyCode::S) { 1. } else { 0. };
        let right = if keys.pressed(KeyCode::D) { 1. } else { 0. };
        let move_dir = Vec2::new(right - left, up - down).normalize_or_zero() * player.speed as f32;

        vel.x = move_dir.x;
        vel.y = move_dir.y;
    }
}
