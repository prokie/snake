use bevy::prelude::*;

use crate::food::FoodEaten;
use crate::state::GameState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Startup, spawn_score_ui)
            .add_systems(OnEnter(GameState::Playing), reset_score)
            .add_observer(on_food_eaten_update_score);
    }
}

#[derive(Resource, Default)]
pub struct Score {
    pub current: u32,
    pub high_score: u32,
}

#[derive(Component)]
struct ScoreText;

fn spawn_score_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(24.0),
            ..default()
        },
        Text::new("Score: 0\nHigh: 0"),
        TextFont {
            font_size: FontSize::Px(28.0),
            ..default()
        },
        TextColor(Color::WHITE),
        ScoreText,
    ));
}

fn reset_score(mut score: ResMut<Score>, mut text: Single<&mut Text, With<ScoreText>>) {
    score.current = 0;
    text.0 = format!("Score: {}\nHigh: {}", score.current, score.high_score);
}

fn on_food_eaten_update_score(
    food_eaten: On<FoodEaten>,
    mut score: ResMut<Score>,
    mut text: Single<&mut Text, With<ScoreText>>,
) {
    score.current += food_eaten.points;
    if score.current > score.high_score {
        score.high_score = score.current;
    }

    text.0 = format!("Score: {}\nHigh: {}", score.current, score.high_score);
}
