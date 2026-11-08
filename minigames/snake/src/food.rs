use bevy::prelude::*;
use rand::seq::IndexedRandom;

use crate::board::GridPosition;
use crate::constants::{BOARD_HEIGHT, BOARD_WIDTH, CELL_SIZE};
use crate::snake::{SnakeHead, SnakeSystems};
use crate::state::GameState;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (check_food_eaten, spawn_food_if_missing)
                .chain()
                .after(SnakeSystems)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

/// Global Observer Event broadcast whenever a piece of food is eaten
#[derive(Event, Clone, Copy, Debug)]
pub struct FoodEaten {
    pub points: u32,
    pub growth: usize,
}

#[derive(Component)]
pub struct Food {
    pub points: u32,
    pub growth: usize,
}

impl Default for Food {
    fn default() -> Self {
        Self {
            points: 10,
            growth: 1,
        }
    }
}

fn check_food_eaten(
    mut commands: Commands,
    head_pos: Single<&GridPosition, (With<SnakeHead>, Changed<GridPosition>)>,
    food_query: Query<(Entity, &GridPosition, &Food)>,
) {
    for (food_entity, food_pos, food) in &food_query {
        if *food_pos == **head_pos {
            commands.entity(food_entity).despawn();
            commands.trigger(FoodEaten {
                points: food.points,
                growth: food.growth,
            });
        }
    }
}

fn spawn_food_if_missing(
    mut commands: Commands,
    food_query: Query<(), With<Food>>,
    occupied_query: Query<&GridPosition>,
) {
    if !food_query.is_empty() {
        return;
    }

    let occupied: Vec<GridPosition> = occupied_query.iter().copied().collect();

    let mut free_cells = Vec::new();
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            let pos = GridPosition::new(x, y);
            if !occupied.contains(&pos) {
                free_cells.push(pos);
            }
        }
    }

    let mut rng = rand::rng();
    if let Some(&spawn_pos) = free_cells.choose(&mut rng) {
        commands.spawn((
            DespawnOnExit(GameState::Playing),
            Sprite {
                color: Color::srgb(0.9, 0.2, 0.2),
                custom_size: Some(Vec2::splat(CELL_SIZE)),
                ..default()
            },
            Transform::default(),
            spawn_pos,
            Food::default(),
        ));
    }
}
