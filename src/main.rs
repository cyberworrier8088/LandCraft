// src/main.rs


mod player;
mod world;

use bevy::prelude::*;

fn main() {

    App::new().add_plugins(DefaultPlugins).add_systems(Startup, (player::setup_player, world::setup_world, player::lock_cursor,))
    .add_systems(Update, (player::player_movement, player::mouse_look, player::detect_block, (player::apply_gravity, player::apply_velocity, player::ground_collision,).chain()))
    .run();
}
