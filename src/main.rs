// src/main.rs


mod player;
mod world;
mod exit;
mod ui;
mod player_model;

use bevy::prelude::*;

fn main() {

    App::new().add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())).add_systems(Startup, (player::setup_player, world::setup_world, player::lock_cursor, ui::setup_crosshair, player::setup_block_highlight, player_model::setup_player_model,))
    .add_systems(
        Update, (
            player::player_movement,
            player::mouse_look,
            player_model::sync_player_model,
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
