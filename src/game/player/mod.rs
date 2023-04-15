use bevy::prelude::*;

use systems::*;

use crate::AppState;
use super::SimulationState;

pub mod components;
mod systems;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]

pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        // Configure System Sets
        .configure_set(MovementSystemSet.before(ConfinementSystemSet))
        // On Enter State
        .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
        // Systems
        .add_system(player_movement
            .in_set(MovementSystemSet)
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running))
        )
        .add_system(confine_player_movemet
            .in_set(ConfinementSystemSet)
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running))
        )
        .add_systems(
            (
                enemy_hit_player,
                player_hit_star,
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running))

        )
        // On Exit State
        .add_system(despawn_player.in_schedule(OnExit(AppState::Game)))

        ;
    }
}