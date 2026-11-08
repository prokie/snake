use bevy::prelude::*;
use lightyear::connection::client::Connected;
use lightyear::connection::host::HostServer;
use lightyear::prelude::{input::native::*, server::*, *};
use protocol::{Inputs, PaddleSide, PlayerId, PlayerSide, Position2D};

use crate::constants::*;
use crate::physics::{Collider, PhysicsSystems};
use crate::state::GameState;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PaddleSystems;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), server_spawn_paddles)
            .add_systems(
                FixedUpdate,
                shared_move_paddles
                    .in_set(PaddleSystems)
                    .before(PhysicsSystems)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_observer(attach_paddle_visuals);
    }
}

/// Server spawns paddles for connected clients when Pong starts
fn server_spawn_paddles(
    mut commands: Commands,
    server: Query<(), With<Endpoint>>,
    clients: Query<(Entity, &RemoteId), (With<ClientOf>, With<Connected>)>,
) {
    if server.is_empty() {
        return;
    }

    let mut client_list: Vec<(Entity, PeerId)> = clients.iter().map(|(e, r)| (e, r.0)).collect();
    client_list.sort_by_key(|(_, peer_id)| peer_id.to_bits());

    let sides = [
        (PlayerSide::Left, -PADDLE_X_OFFSET),
        (PlayerSide::Right, PADDLE_X_OFFSET),
    ];

    for (idx, (side, x)) in sides.into_iter().enumerate() {
        let mut entity_cmds = commands.spawn((
            DespawnOnExit(GameState::Playing),
            PaddleSide(side),
            Position2D(Vec2::new(x, 0.0)),
            Collider::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            Transform::from_xyz(x, 0.0, 1.0),
            Replicate::to_clients(NetworkTarget::All),
        ));

        if let Some(&(client_entity, peer_id)) = client_list.get(idx) {
            entity_cmds.insert((
                PlayerId(peer_id),
                PredictionTarget::to_clients(NetworkTarget::Single(peer_id)),
                InterpolationTarget::to_clients(NetworkTarget::AllExceptSingle(peer_id)),
                ControlledBy {
                    owner: client_entity,
                    lifetime: Default::default(),
                },
            ));
        } else {
            entity_cmds.insert(InterpolationTarget::to_clients(NetworkTarget::All));
        }
    }
}

/// Client observer: attaches Sprite + Collider whenever a Predicted or Interpolated paddle spawns
fn attach_paddle_visuals(
    trigger: On<Add<PaddleSide>>,
    mut commands: Commands,
    query: Query<&Position2D, Or<(With<Predicted>, With<Interpolated>, With<HostServer>)>>,
) {
    let entity = trigger.entity;
    let pos = query.get(entity).copied().unwrap_or(Position2D(Vec2::ZERO));

    commands.entity(entity).insert((
        DespawnOnExit(GameState::Playing),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(pos.x, pos.y, 1.0),
        Collider::new(PADDLE_WIDTH, PADDLE_HEIGHT),
    ));
}

/// Runs on both Client (for Predicted paddle) and Server (for authoritative paddle)
fn shared_move_paddles(
    time: Res<Time>,
    host_server: Query<(), With<HostServer>>,
    mut query: Query<(&mut Position2D, &ActionState<Inputs>, Has<Predicted>), With<PaddleSide>>,
) {
    let is_host_server = !host_server.is_empty();
    let dt = time.delta_secs();
    let max_y = (ARENA_HEIGHT * 0.5) - (PADDLE_HEIGHT * 0.5);

    for (mut pos, input, is_predicted) in &mut query {
        // In HostServer mode, prevent moving the host's entity twice
        if is_host_server && is_predicted {
            continue;
        }
        let Inputs::Direction(dir) = input.0;
        let mut move_dir = 0.0;
        if dir.up {
            move_dir += 1.0;
        }
        if dir.down {
            move_dir -= 1.0;
        }
        if move_dir != 0.0 {
            pos.y = (pos.y + move_dir * PADDLE_SPEED * dt).clamp(-max_y, max_y);
        }
    }
}
