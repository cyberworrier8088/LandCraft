mod player;
mod world;

use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_systems(Startup, (player::setup_player, world::setup_world))
    .add_systems(Update, player::player_movement,)
    .run();
}
