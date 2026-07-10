// src/player.rs

use bevy::prelude::*;

use bevy::input::mouse::MouseMotion;
use bevy::window::{CursorGrabMode, CursorOptions};

// Import our custom block mesh creator and types
use crate::mesh::{create_block_mesh, create_chunk_mesh, BlockType};
use crate::noise::terrain_height;

// Import chunk structures
use crate::world::Chunk;


#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct CameraPivot;

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct CameraView {
    third_person: bool,
}





#[derive(Component)]
pub struct BlockHighlight;

// volocity 
#[derive(Component)]
pub struct Velocity {
    pub value: Vec3,
}

const JUMP_FORCE: f32 = 8.0;
const PLAYER_WIDTH: f32 = 0.6;
const PLAYER_HEIGHT: f32 = 1.8;
// onground
#[derive(Component)]
pub struct OnGround {
    pub value: bool,
}

#[derive(Component)]
pub struct LookAngles {
    pub yaw: f32, // yaw means left and right
    pub pitch: f32, // pitch means up and down
}



// setup player function 
pub fn setup_player(mut commands: Commands) {

    // player position spawn
    let spawn_x = 8;
    let spawn_z = 8;
    let spawn_y = terrain_height(spawn_x, spawn_z) + 2;
    // spawn player root with camera and look components
    commands.spawn((
        Player,
        Velocity {
            value: Vec3::ZERO,
        },
        OnGround {
            value: false,
        },
        LookAngles {
            yaw: 0.0,
            pitch: 0.0,
        },
        Transform::from_xyz(spawn_x as f32, spawn_y as f32, spawn_z as f32),

    ))
    .with_children(|parent| {
        parent.spawn((
            CameraPivot,
            Transform::from_xyz(0.0, 0.7, 0.0),
        )).with_children(|pivot| {
            pivot.spawn((
                Camera3d::default(),
                GameCamera,
                CameraView {
                    third_person: false,
                },
                Transform::from_xyz(0.0, 0.1, -0.35),
            ));
        });
    }); 

    commands.spawn((
        DirectionalLight {
            illuminance: 15000.0,
            ..default()
        },

        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -1.0,
            -1.0,
            0.0,
        )),
    ));
}



// this function for player movement controll users easly. 
// for WASD in key
// w means forward
// s means backward
// a means left
// d means right
pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&Transform, &mut Velocity, &OnGround), With<Player>>,
) {
    let (transform, mut velocity, on_ground) = player.single_mut().unwrap();
    let mut direction = Vec3::ZERO;
    let mut forward = *transform.forward();
    forward.y = 0.0;
    forward = forward.normalize_or_zero();
    let mut right = *transform.right();
    right.y = 0.0;
    right = right.normalize_or_zero();

    if keyboard.pressed(KeyCode::KeyW) { direction += forward; }
    if keyboard.pressed(KeyCode::KeyS) { direction -= forward; }
    if keyboard.pressed(KeyCode::KeyA) { direction -= right; }
    if keyboard.pressed(KeyCode::KeyD) { direction += right; }
    if keyboard.just_pressed(KeyCode::Space) && on_ground.value { velocity.value.y = JUMP_FORCE; }

    let target = direction.normalize_or_zero() * 5.0;
    let blend = (time.delta_secs() * 18.0).min(1.0);
    velocity.value.x += (target.x - velocity.value.x) * blend;
    velocity.value.z += (target.z - velocity.value.z) * blend;
}
pub fn toggle_camera_view(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera: Query<(&mut Transform, &mut CameraView), With<GameCamera>>,
) {
    if !keyboard.just_pressed(KeyCode::F2) {
        return;
    }

    if let Ok((mut transform, mut view)) = camera.single_mut() {
        view.third_person = !view.third_person;

        *transform = if view.third_person {
            Transform::from_xyz(0.0, 0.0, 4.0)
        } else {
            Transform::from_xyz(0.0, 0.1, -0.35)
        };
    }
}


// function for mouse look means a player can look around fixed.
pub fn mouse_look(
    mut mouse_events: MessageReader<MouseMotion>,
    mut player: Query<(&mut Transform, &mut LookAngles), With<Player>>,
    mut pivot: Query<&mut Transform, (With<CameraPivot>, Without<Player>)>
) {
    let delta = mouse_events.read().fold(Vec2::ZERO, |sum, event| sum + event.delta);
    if delta == Vec2::ZERO {
        return;
    }

    let (mut player_transform, mut angles) = player.single_mut().unwrap();
    let mut pivot_transform = pivot.single_mut().unwrap();

    angles.yaw -= delta.x * 0.0025;
    angles.pitch = (angles.pitch - delta.y * 0.0025).clamp(-1.54, 1.54);

    player_transform.rotation = Quat::from_rotation_y(angles.yaw);
    pivot_transform.rotation = Quat::from_rotation_x(angles.pitch);
}
// function for lock cursor means mouse cursor lock not visible :) 
pub fn lock_cursor(
    mut cursor_options: Single<&mut CursorOptions>,
) {
    cursor_options.grab_mode = CursorGrabMode::Locked;
    cursor_options.visible = false; // cursor hidding
}


pub fn apply_velocity(
    time: Res<Time>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut OnGround), With<Player>>,
    chunks: Query<(&Transform, &Chunk), (Without<Player>, With<Chunk>)>,
) {
    let (mut transform, mut velocity, mut on_ground) = player.single_mut().unwrap();
    let dt = time.delta_secs().min(0.03);
    let half = Vec3::new(PLAYER_WIDTH * 0.5, PLAYER_HEIGHT * 0.5, PLAYER_WIDTH * 0.5);
    on_ground.value = false;
    velocity.value.y = (velocity.value.y - 24.0 * dt).max(-30.0);

    // Collision checking helper function
    let check_collision_axis = |pos: Vec3, chunks_query: &Query<(&Transform, &Chunk), (Without<Player>, With<Chunk>)>| -> Option<Vec3> {
        for (chunk_transform, chunk) in chunks_query.iter() {
            let local_pos = pos - chunk_transform.translation;
            // Check if the player bounding box could overlap this chunk
            // Chunk AABB is from -0.5 to 15.5 relative to chunk origin.
            // Player AABB relative to chunk origin is local_pos - half to local_pos + half.
            let min_local = local_pos - half;
            let max_local = local_pos + half;

            if max_local.x < -0.5 || min_local.x > 15.5 ||
               max_local.y < -0.5 || min_local.y > 15.5 ||
               max_local.z < -0.5 || min_local.z > 15.5 {
                continue;
            }

            // Find overlapping block coordinates in chunk local space
            let start_x = (min_local.x - 0.5).floor().max(0.0).min(15.0) as usize;
            let end_x = (max_local.x + 0.5).ceil().max(0.0).min(15.0) as usize;
            let start_y = (min_local.y - 0.5).floor().max(0.0).min(15.0) as usize;
            let end_y = (max_local.y + 0.5).ceil().max(0.0).min(15.0) as usize;
            let start_z = (min_local.z - 0.5).floor().max(0.0).min(15.0) as usize;
            let end_z = (max_local.z + 0.5).ceil().max(0.0).min(15.0) as usize;

            for lx in start_x..=end_x {
                for ly in start_y..=end_y {
                    for lz in start_z..=end_z {
                        let idx = lx + ly * 16 + lz * 256;
                        if chunk.blocks[idx] != BlockType::Air {
                            let block_world_pos = chunk_transform.translation + Vec3::new(lx as f32, ly as f32, lz as f32);
                            let d = pos - block_world_pos;
                            if d.x.abs() < half.x + 0.5 && d.y.abs() < half.y + 0.5 && d.z.abs() < half.z + 0.5 {
                                return Some(block_world_pos);
                            }
                        }
                    }
                }
            }
        }
        None
    };

    if velocity.value.x != 0.0 {
        transform.translation.x += velocity.value.x * dt;
        if let Some(block_world_pos) = check_collision_axis(transform.translation, &chunks) {
            transform.translation.x = block_world_pos.x - velocity.value.x.signum() * (half.x + 0.5);
            velocity.value.x = 0.0;
        }
    }

    transform.translation.y += velocity.value.y * dt;
    if let Some(block_world_pos) = check_collision_axis(transform.translation, &chunks) {
        on_ground.value = velocity.value.y < 0.0;
        transform.translation.y = block_world_pos.y - velocity.value.y.signum() * (half.y + 0.5);
        velocity.value.y = 0.0;
    }

    if velocity.value.z != 0.0 {
        transform.translation.z += velocity.value.z * dt;
        if let Some(block_world_pos) = check_collision_axis(transform.translation, &chunks) {
            transform.translation.z = block_world_pos.z - velocity.value.z.signum() * (half.z + 0.5);
            velocity.value.z = 0.0;
        }
    }
}

#[derive(Resource, Default)]
pub struct SelectedBlock {
    pub pos: Option<Vec3>,
}

pub fn select_block(
    mouse: Res<ButtonInput<MouseButton>>,
    camera: Query<&GlobalTransform, With<GameCamera>>,
    mut chunks: Query<(Entity, &Transform, &mut Chunk, &Mesh3d), With<Chunk>>,
    player: Query<&Transform, (With<Player>, Without<GameCamera>)>,
    mut selected_block: ResMut<SelectedBlock>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    selected_block.pos = None;

    let camera = camera.single().unwrap();
    let forward = camera.forward();
    let left = mouse.just_pressed(MouseButton::Left);
    let right = mouse.just_pressed(MouseButton::Right);
    let mut last = camera.translation();
    let mut distance = 0.0;

    let player_transform = player.single().unwrap();

    while distance <= 6.0 {
        let point = camera.translation() + forward * distance;
        let block_pos = point.round();

        let chunk_x = (block_pos.x / 16.0).floor() as i32;
        let chunk_y = (block_pos.y / 16.0).floor() as i32;
        let chunk_z = (block_pos.z / 16.0).floor() as i32;
        let chunk_coord = IVec3::new(chunk_x, chunk_y, chunk_z);

        let mut hit = false;
        for (_entity, chunk_transform, mut chunk, mesh3d) in chunks.iter_mut() {
            let chunk_pos_block = chunk_coord * 16;
            if chunk_transform.translation.round().as_ivec3() == chunk_pos_block {
                let lx = (block_pos.x as i32 - chunk_pos_block.x) as usize;
                let ly = (block_pos.y as i32 - chunk_pos_block.y) as usize;
                let lz = (block_pos.z as i32 - chunk_pos_block.z) as usize;

                if lx < 16 && ly < 16 && lz < 16 {
                    let idx = lx + ly * 16 + lz * 256;
                    if chunk.blocks[idx] != BlockType::Air {
                        hit = true;
                        selected_block.pos = Some(block_pos);

                        if left {
                            chunk.blocks[idx] = BlockType::Air;
                            if let Some(mut mesh) = meshes.get_mut(&mesh3d.0) {
                                *mesh = create_chunk_mesh(&chunk.blocks);
                            }
                        } else if right {
                            let place = last.round();
                            let d = player_transform.translation - place;
                            if d.x.abs() >= PLAYER_WIDTH * 0.5 + 0.5 
                                || d.y.abs() >= PLAYER_HEIGHT * 0.5 + 0.5 
                                || d.z.abs() >= PLAYER_WIDTH * 0.5 + 0.5 
                            {
                                let place_chunk_x = (place.x / 16.0).floor() as i32;
                                let place_chunk_y = (place.y / 16.0).floor() as i32;
                                let place_chunk_z = (place.z / 16.0).floor() as i32;
                                let place_chunk_coord = IVec3::new(place_chunk_x, place_chunk_y, place_chunk_z);

                                for (_place_entity, place_chunk_transform, mut place_chunk, place_mesh3d) in chunks.iter_mut() {
                                    let place_chunk_pos_block = place_chunk_coord * 16;
                                    if place_chunk_transform.translation.round().as_ivec3() == place_chunk_pos_block {
                                        let plx = (place.x as i32 - place_chunk_pos_block.x) as usize;
                                        let ply = (place.y as i32 - place_chunk_pos_block.y) as usize;
                                        let plz = (place.z as i32 - place_chunk_pos_block.z) as usize;

                                        if plx < 16 && ply < 16 && plz < 16 {
                                            let p_idx = plx + ply * 16 + plz * 256;
                                            place_chunk.blocks[p_idx] = BlockType::Grass;
                                            if let Some(mut mesh) = meshes.get_mut(&place_mesh3d.0) {
                                                *mesh = create_chunk_mesh(&place_chunk.blocks);
                                            }
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }

        if hit {
            return;
        }

        last = point;
        distance += 0.1;
    }
}

pub fn setup_block_highlight(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let highlight_mesh = meshes.add(create_block_mesh(BlockType::Grass));

    let highlight_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.25),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    commands.spawn((
        BlockHighlight,
        Mesh3d(highlight_mesh),
        MeshMaterial3d(highlight_material),
        Transform::from_scale(Vec3::splat(1.02)),
        Visibility::Hidden,
    ));

    commands.insert_resource(SelectedBlock::default());
}

pub fn update_block_highlight(
    selected_block: Res<SelectedBlock>,
    mut highlight: Query<
        (&mut Transform, &mut Visibility),
        With<BlockHighlight>,
    >,
) {
    let (mut highlight_transform, mut visibility) =
        highlight.single_mut().unwrap();

    if let Some(pos) = selected_block.pos {
        highlight_transform.translation = pos;
        *visibility = Visibility::Visible;
    } else {
        *visibility = Visibility::Hidden;
    }
}


