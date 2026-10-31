use bevy::prelude::*;

mod board;
mod constants;
mod food;
mod snake;

use crate::{board::BoardPlugin, food::FoodPlugin, snake::SnakePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "assets".into(),
            ..default()
        }))
        .add_systems(Startup, setup_camera)
        .add_plugins((BoardPlugin, SnakePlugin, FoodPlugin))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
