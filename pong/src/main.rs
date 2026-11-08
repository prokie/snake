use crate::{
    ball::BallPlugin,
    constants::{ARENA_HEIGHT, ARENA_WIDTH, BG_COLOR, CAMERA_MARGIN, COURT_COLOR},
    paddle::PaddlePlugin,
    physics::PhysicsPlugin,
    score::ScorePlugin,
    state::StatePlugin,
};
use bevy::{camera::ScalingMode, prelude::*, window::WindowResolution};

pub mod ball;
pub mod constants;
pub mod paddle;
pub mod physics;
pub mod score;
pub mod state;

fn main() {
    App::new()
        .insert_resource(ClearColor(BG_COLOR))
        .add_plugins(
            DefaultPlugins
                // Use nearest-neighbor scaling for pixel art
                .set(ImagePlugin::default_nearest())
                // Set up the window with a title and resolution
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pong".into(),
                        resolution: WindowResolution::new(1920, 1080),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, (setup_camera, spawn_court))
        .add_plugins((
            StatePlugin,
            PaddlePlugin,
            PhysicsPlugin,
            BallPlugin,
            ScorePlugin,
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
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
    // Spawn a rectangle to represent the court
    commands.spawn((
        Sprite {
            color: COURT_COLOR,
            custom_size: Some(Vec2::new(ARENA_WIDTH, ARENA_HEIGHT)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));

    // Spawn center divider line
    commands.spawn((
        Sprite {
            color: Color::srgba(1.0, 1.0, 1.0, 0.2),
            custom_size: Some(Vec2::new(4.0, ARENA_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -0.5),
    ));
}
