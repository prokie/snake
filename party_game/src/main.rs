// party_game/src/main.rs
use bevy::{prelude::*, window::WindowResolution};
use lightyear::prelude::{
    client::{ClientPlugins, InputDelayConfig, InputTimelineConfig, input::*},
    input::native::*,
    server::ServerPlugins,
    *,
};
use pong::PongPlugin;
use protocol::{ChangeGameMode, Direction, GameFlowChannel, GameMode, Inputs, ProtocolPlugin};
use std::time::Duration;

const FIXED_TIMESTEP_HZ: f64 = 64.0;

fn main() {
    let tick_duration = Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ);

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pontus Party".into(),
                        resolution: WindowResolution::new(1280, 720),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((
            ClientPlugins { tick_duration },
            ServerPlugins { tick_duration },
        ))
        .add_plugins((ProtocolPlugin, PongPlugin))
        .add_systems(Startup, configure_input_delay)
        .add_systems(
            FixedPreUpdate,
            buffer_client_inputs.in_set(InputSystems::WriteClientInputs),
        )
        .add_systems(OnEnter(GameMode::Lobby), setup_lobby)
        .add_systems(Update, (host_switch_game_mode, client_receive_game_mode))
        .add_observer(handle_new_client_link)
        .add_observer(handle_controlled_entity_spawn)
        .run();
}

fn configure_input_delay(mut commands: Commands) {
    commands.insert_resource(
        InputTimelineConfig::default().with_input_delay(InputDelayConfig::no_input_delay()),
    );
}

/// Server: attach ReplicationSender to every new client connection link
fn handle_new_client_link(trigger: On<Add<LinkOf>>, mut commands: Commands) {
    commands
        .entity(trigger.entity)
        .insert((ReplicationSender, Name::from("ClientLink")));
}

/// Client: automatically attach InputMarker<Inputs> to any entity the server gives us control over
fn handle_controlled_entity_spawn(trigger: On<Add<Controlled>>, mut commands: Commands) {
    commands
        .entity(trigger.entity)
        .insert(InputMarker::<Inputs>::default());
}

/// Client: sample keyboard every tick and write into ActionState<Inputs>
fn buffer_client_inputs(
    mut query: Query<&mut ActionState<Inputs>, With<InputMarker<Inputs>>>,
    keypress: Option<Res<ButtonInput<KeyCode>>>,
) {
    let Some(keypress) = keypress else { return };
    for mut action_state in &mut query {
        let mut dir = Direction::default();
        if keypress.pressed(KeyCode::KeyW) || keypress.pressed(KeyCode::ArrowUp) {
            dir.up = true;
        }
        if keypress.pressed(KeyCode::KeyS) || keypress.pressed(KeyCode::ArrowDown) {
            dir.down = true;
        }
        if keypress.pressed(KeyCode::KeyA) || keypress.pressed(KeyCode::ArrowLeft) {
            dir.left = true;
        }
        if keypress.pressed(KeyCode::KeyD) || keypress.pressed(KeyCode::ArrowRight) {
            dir.right = true;
        }
        if keypress.pressed(KeyCode::Space) {
            dir.action = true;
        }
        action_state.0 = Inputs::Direction(dir);
    }
}

fn setup_lobby(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)));
    commands.spawn((DespawnOnExit(GameMode::Lobby), Camera2d));
    commands.spawn((
        DespawnOnExit(GameMode::Lobby),
        Text2d::new(
            "PONTUS PARTY - LOBBY\n\nPress [1] to start Pong\nPress [Esc] to return to Lobby",
        ),
        TextFont {
            font_size: FontSize::Px(32.0),
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::justify(Justify::Center),
    ));
}

fn host_switch_game_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_mode: Res<State<GameMode>>,
    mut next_mode: ResMut<NextState<GameMode>>,
    mut sender: ServerMultiMessageSender,
    server: Query<&Endpoint>,
) {
    let target = if keyboard.just_pressed(KeyCode::Digit1) && *current_mode.get() != GameMode::Pong
    {
        Some(GameMode::Pong)
    } else if keyboard.just_pressed(KeyCode::Escape) && *current_mode.get() != GameMode::Lobby {
        Some(GameMode::Lobby)
    } else {
        None
    };

    if let Some(mode) = target {
        next_mode.set(mode);
        if let Ok(endpoint) = server.single() {
            let _ = sender.send::<_, GameFlowChannel>(
                &ChangeGameMode(mode),
                endpoint,
                &NetworkTarget::All,
            );
        }
    }
}

fn client_receive_game_mode(
    mut receivers: Query<&mut MessageReceiver<ChangeGameMode>>,
    mut next_mode: ResMut<NextState<GameMode>>,
) {
    for mut receiver in &mut receivers {
        for msg in receiver.receive() {
            next_mode.set(msg.0);
        }
    }
}
