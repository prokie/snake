use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy::window::WindowResolution;

pub mod board;
pub mod constants;
pub mod food;
pub mod snake;
pub mod state;

use board::BoardPlugin;
use food::FoodPlugin;
use snake::SnakePlugin;

use crate::constants::{BOARD_HEIGHT, BOARD_WIDTH, CELL_SIZE};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "assets".into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Snake".into(),
                        // Set window to 1920x1080 (or use `mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current)` for fullscreen)
                        resolution: WindowResolution::new(1920, 1080),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup_camera)
        .add_plugins((BoardPlugin, SnakePlugin, FoodPlugin))
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Total world size of the board (e.g., 20 * 16.0 = 320.0 units) plus a 2-cell border margin
    let view_width = (BOARD_WIDTH as f32 + 2.0) * CELL_SIZE;
    let view_height = (BOARD_HEIGHT as f32 + 2.0) * CELL_SIZE;

    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            // Automatically scales the board to fit the window while maintaining aspect ratio, with a minimum size of view_width x view_height
            scaling_mode: ScalingMode::AutoMin {
                min_width: view_width,
                min_height: view_height,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
