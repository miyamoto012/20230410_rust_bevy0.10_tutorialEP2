use bevy::prelude::*;

use systems::*;

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_player)
        .add_system(player_movement)
        .add_system(confine_player_movemet)
        .add_system(enemy_hit_player)
        .add_system(player_hit_star);
    }
}