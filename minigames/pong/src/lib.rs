use crate::{
    ball::BallPlugin,
    constants::{ARENA_HEIGHT, ARENA_WIDTH, BG_COLOR, CAMERA_MARGIN, COURT_COLOR},
    paddle::PaddlePlugin,
    physics::PhysicsPlugin,
    score::ScorePlugin,
    state::StatePlugin,
};
use bevy::{camera::ScalingMode, prelude::*};
use protocol::GameMode;

pub mod ball;
pub mod constants;
pub mod paddle;
pub mod physics;
pub mod score;
pub mod state;

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameMode::Pong),
            (setup_pong_environment, setup_camera, spawn_court),
        )
        .add_plugins((
            StatePlugin,
            PaddlePlugin,
            PhysicsPlugin,
            BallPlugin,
            ScorePlugin,
        ));
    }
}

fn setup_pong_environment(mut commands: Commands) {
    commands.insert_resource(ClearColor(BG_COLOR));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameMode::Pong),
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: ARENA_WIDTH + CAMERA_MARGIN,
                min_height: ARENA_HEIGHT + CAMERA_MARGIN,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn spawn_court(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameMode::Pong),
        Sprite {
            color: COURT_COLOR,
            custom_size: Some(Vec2::new(ARENA_WIDTH, ARENA_HEIGHT)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));

    commands.spawn((
        DespawnOnExit(GameMode::Pong),
        Sprite {
            color: Color::srgba(1.0, 1.0, 1.0, 0.2),
            custom_size: Some(Vec2::new(4.0, ARENA_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -0.5),
    ));
}
