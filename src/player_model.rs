use bevy::prelude::*;
use crate::player::{Player, LookAngles};

#[derive(Component)]
pub struct PlayerRoot;

pub fn setup_player_model(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        PlayerRoot,
        WorldAssetRoot(asset_server.load("player/player.glb#Scene0")),
        Transform::from_xyz(8.0, 8.0, 8.0).with_scale(Vec3::splat(0.35)),
    ));
}

pub fn sync_player_model(
    player_q: Query<(&Transform, &LookAngles), With<Player>>,
    mut model_q: Query<&mut Transform, (With<PlayerRoot>, Without<Player>)>,
) {
    if let Ok((player_transform, look_angles)) = player_q.single() {
        if let Ok(mut model_transform) = model_q.single_mut() {
            model_transform.translation = player_transform.translation;
            model_transform.rotation = Quat::from_rotation_y(look_angles.yaw);
        }
    }
}