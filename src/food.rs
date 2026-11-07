use bevy::prelude::*;
use rand::seq::IndexedRandom;

use crate::board::GridPosition;
use crate::constants::{BOARD_HEIGHT, BOARD_WIDTH, CELL_SIZE};
use crate::snake::SnakeHead;
use crate::state::GameState;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (check_food_eaten, spawn_food_if_missing)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Food;

fn check_food_eaten(
    mut commands: Commands,
    head_query: Single<(&GridPosition, &mut SnakeHead), Changed<GridPosition>>,
    food_query: Query<(Entity, &GridPosition), With<Food>>,
) {
    let (head_pos, mut head) = head_query.into_inner();

    for (food_entity, food_pos) in &food_query {
        if *food_pos == *head_pos {
            commands.entity(food_entity).despawn();
            head.pending_growth += 1;
        }
    }
}

/// Spawns a new food entity on a free grid cell whenever none exists on the board
fn spawn_food_if_missing(
    mut commands: Commands,
    food_query: Query<(), With<Food>>,
    occupied_query: Query<&GridPosition>,
    // Uncomment asset_server if you have a food.png / apple.png sprite:
    // asset_server: Res<AssetServer>,
) {
    if !food_query.is_empty() {
        return;
    }

    // Collect all grid positions currently occupied by the snake (head + future body segments)
    let occupied: Vec<GridPosition> = occupied_query.iter().copied().collect();

    // Build a list of all free cells on the board
    let mut free_cells = Vec::new();
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            let pos = GridPosition::new(x, y);
            if !occupied.contains(&pos) {
                free_cells.push(pos);
            }
        }
    }

    // Pick a random free cell and spawn the food
    let mut rng = rand::rng();
    if let Some(&spawn_pos) = free_cells.choose(&mut rng) {
        commands.spawn((
            DespawnOnExit(GameState::Playing),
            Sprite {
                // Swap color for `image: asset_server.load("apple.png"),` if you have a sprite
                color: Color::srgb(0.9, 0.2, 0.2),
                custom_size: Some(Vec2::splat(CELL_SIZE)),
                ..default()
            },
            Transform::default(),
            spawn_pos,
            Food,
        ));
    }
}
