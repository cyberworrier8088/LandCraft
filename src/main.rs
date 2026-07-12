// src/main.rs


mod player;
mod world;
mod exit;
mod ui;
mod player_model;
mod mesh;
mod noise;
mod inventory;


use bevy::prelude::*;

fn main() {

    App::new().insert_resource(world::LoadedChunks::default()).insert_resource(inventory::Inventory::default()).add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())).add_systems(Startup, (player::setup_player, world::setup_world, player::lock_cursor, ui::setup_crosshair, ui::setup_hotbar, player::setup_block_highlight, player_model::setup_player_model,))
    .add_systems(
        Update, (
            ui::update_hotbar,
            ui::update_hotbar_icons,
            inventory::change_selected_slot,
            world::update_chunks,
            (
                player::mouse_look,
                player::player_movement,
                player::apply_velocity,
                player_model::sync_player_model,
            ).chain(),
            player::toggle_camera_view,
            (
                player::select_block,
                player::update_block_highlight,
            ).chain(),
            exit::close_on_escape,
        ),
    )
    .run();
}