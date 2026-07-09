use std::{f32::consts::PI, time::Duration};

use bevy::{prelude::*, world_serialization::WorldInstanceReady};
use crate::player::{Player, LookAngles, Velocity};

const PLAYER_MODEL: &str = "player/minecraft_player_wide_rigged_with_outer_layer.glb";

#[derive(Component)]
pub struct PlayerRoot;

#[derive(Component)]
pub struct PlayerAnimation {
    graph: Handle<AnimationGraph>,
    idle: AnimationNodeIndex,
    run: AnimationNodeIndex,
}

pub fn setup_player_model(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let (graph, clips) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(PLAYER_MODEL)),
        asset_server.load(GltfAssetLabel::Animation(1).from_asset(PLAYER_MODEL)),
    ]);

    commands.spawn((
        PlayerRoot,
        PlayerAnimation {
            graph: graphs.add(graph),
            idle: clips[0],
            run: clips[1],
        },
        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(PLAYER_MODEL))),
        Transform::from_xyz(8.0, 8.0, 8.0),
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
                let mut transitions = AnimationTransitions::new();
                transitions.play(&mut player, animation.idle, Duration::ZERO).repeat();
                commands.entity(child)
                    .insert(AnimationGraphHandle(animation.graph.clone()))
                    .insert(transitions);
            }
        }
    }
}

pub fn sync_player_model(
    player_q: Query<(&Transform, &LookAngles, &Velocity), With<Player>>,
    mut model_q: Query<(&mut Transform, &PlayerAnimation), (With<PlayerRoot>, Without<Player>)>,
    mut animation_q: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
) {
    if let Ok((player_transform, look_angles, velocity)) = player_q.single() {
        if let Ok((mut model_transform, animation)) = model_q.single_mut() {
            model_transform.translation = player_transform.translation - Vec3::Y * 0.9;
            model_transform.rotation = Quat::from_rotation_y(look_angles.yaw + PI);

            if let Ok((mut player, mut transitions)) = animation_q.single_mut() {
                let next = if velocity.value.x * velocity.value.x + velocity.value.z * velocity.value.z > 0.1 {
                    animation.run
                } else {
                    animation.idle
                };

                if !player.playing_animations().any(|(&playing, _)| playing == next) {
                    transitions.play(&mut player, next, Duration::from_millis(120)).repeat();
                }
            }
        }
    }
}
