use bevy::prelude::*;

use crate::ball::PointScored;
use crate::constants::WINNING_SCORE;
use crate::paddle::PlayerSide;
use crate::state::GameState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Startup, spawn_scoreboard)
            .add_systems(OnEnter(GameState::Playing), reset_score)
            .add_observer(on_point_scored);
    }
}

#[derive(Resource, Default)]
pub struct Score {
    pub left: u32,
    pub right: u32,
    pub winner: Option<PlayerSide>,
}

#[derive(Component, Default, Clone)]
struct LeftScoreText;

#[derive(Component, Default, Clone)]
struct RightScoreText;

/// Spawns a full-width top bar with Left and Right score texts using Bevy 0.20's bsn! syntax
fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(24.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceAround,
        }
        Children [
            Text("0")
            TextFont { font_size: FontSize::Rem(3.5) }
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.75))
            LeftScoreText
            --
            Text("0")
            TextFont { font_size: FontSize::Rem(3.5) }
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.75))
            RightScoreText
        ]
    });
}

/// Resets scores to 0 whenever a new match starts
fn reset_score(
    mut score: ResMut<Score>,
    mut left_text: Single<&mut Text, (With<LeftScoreText>, Without<RightScoreText>)>,
    mut right_text: Single<&mut Text, (With<RightScoreText>, Without<LeftScoreText>)>,
) {
    score.left = 0;
    score.right = 0;
    score.winner = None;
    left_text.0 = "0".into();
    right_text.0 = "0".into();
}

/// Observer: Reacts to PointScored, updates the UI, and checks for an 11-point win
fn on_point_scored(
    point: On<PointScored>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GameState>>,
    mut left_text: Single<&mut Text, (With<LeftScoreText>, Without<RightScoreText>)>,
    mut right_text: Single<&mut Text, (With<RightScoreText>, Without<LeftScoreText>)>,
) {
    match point.scorer {
        PlayerSide::Left => {
            score.left += 1;
            left_text.0 = score.left.to_string();
            if score.left >= WINNING_SCORE {
                score.winner = Some(PlayerSide::Left);
                next_state.set(GameState::GameOver);
            }
        }
        PlayerSide::Right => {
            score.right += 1;
            right_text.0 = score.right.to_string();
            if score.right >= WINNING_SCORE {
                score.winner = Some(PlayerSide::Right);
                next_state.set(GameState::GameOver);
            }
        }
    }
}
