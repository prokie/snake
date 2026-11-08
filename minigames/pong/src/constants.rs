use bevy::prelude::*;

pub const ARENA_WIDTH: f32 = 1280.0;
pub const ARENA_HEIGHT: f32 = 720.0;
pub const CAMERA_MARGIN: f32 = 40.0;

pub const BG_COLOR: Color = Color::srgb(0.05, 0.05, 0.08);
pub const COURT_COLOR: Color = Color::srgb(0.10, 0.10, 0.14);

pub const PADDLE_WIDTH: f32 = 20.0;
pub const PADDLE_HEIGHT: f32 = 120.0;
pub const PADDLE_SPEED: f32 = 550.0;
pub const PADDLE_X_OFFSET: f32 = (ARENA_WIDTH * 0.5) - 50.0;

pub const BALL_SIZE: f32 = 18.0;
pub const INITIAL_BALL_SPEED: f32 = 500.0;
pub const BALL_SPEED_INCREMENT: f32 = 1.05; // 5% faster per paddle hit

pub const WINNING_SCORE: u32 = 11;
