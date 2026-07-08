use bevy::{prelude::*, world_serialization::WorldInstanceReady};
use crate::player::{Player, LookAngles};

const PLAYER_MODEL: &str = "player/animated_humanoid_robot.glb";

#[derive(Component)]
pub struct PlayerRoot;

#[derive(Component)]
struct PlayerAnimation {
    graph: Handle<AnimationGraph>,
    index: AnimationNodeIndex,
}

pub fn setup_player_model(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let (graph, index) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(PLAYER_MODEL)),
    );

    commands.spawn((
        PlayerRoot,
        PlayerAnimation {
            graph: graphs.add(graph),
            index,
        },
        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(PLAYER_MODEL))),
        Transform::from_xyz(8.0, 8.0, 8.0).with_scale(Vec3::splat(0.12)),
    )).observe(play_player_animation);
}

fn play_player_animation(
    ready: On<WorldInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    animation: Query<&PlayerAnimation>,
    mut players: Query<&mut AnimationPlayer>,
) {
    if let Ok(animation) = animation.get(ready.entity) {
        for child in children.iter_descendants(ready.entity) {
            if let Ok(mut player) = players.get_mut(child) {
                player.play(animation.index).repeat();
                commands.entity(child).insert(AnimationGraphHandle(animation.graph.clone()));
            }
        }
    }
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
