// src/main.rs


mod player;
mod world;
mod exit;
mod ui;

use bevy::prelude::*;

fn main() {

    App::new().add_plugins(DefaultPlugins).add_systems(Startup, (player::setup_player, world::setup_world, player::lock_cursor, ui::setup_crosshair, player::setup_block_highlight,))
    .add_systems(
        Update, (
            player::player_movement,
            player::mouse_look,
            (
                player::select_block,
                player::detect_block,
                player::update_block_highlight,
            ).chain(),
            exit::close_on_escape,
            (
                player::apply_gravity,
                player::apply_velocity,
                player::ground_collision,
            ).chain()
        ),
    )
    .run();
}
