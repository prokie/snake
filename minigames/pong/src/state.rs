use bevy::{prelude::*, state::app::AppExtStates};
use protocol::GameMode;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<GameState>()
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over_ui)
            .add_systems(OnEnter(GameState::Paused), spawn_pause_ui)
            .add_systems(
                Update,
                (
                    restart_on_r.run_if(in_state(GameState::GameOver)),
                    pause_on_p.run_if(in_state(GameState::Playing)),
                    resume_on_p.run_if(in_state(GameState::Paused)),
                ),
            );
    }
}

#[derive(SubStates, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[source(GameMode = GameMode::Pong)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
    GameOver,
}

fn spawn_game_over_ui(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::GameOver),
        Text2d::new("GAME OVER\nPress R to Restart"),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.3, 0.3)),
        TextLayout::justify(Justify::Center),
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));
}

fn restart_on_r(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Playing);
    }
}

fn pause_on_p(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        next_state.set(GameState::Paused);
    }
}

fn resume_on_p(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        next_state.set(GameState::Playing);
    }
}

fn spawn_pause_ui(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::Paused),
        Text2d::new("PAUSED\nPress P to Resume"),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 0.3)),
        TextLayout::justify(Justify::Center),
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));
}
