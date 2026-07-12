// src/player.rs

use bevy::prelude::*;

use bevy::input::mouse::MouseMotion;
use bevy::window::{CursorGrabMode, CursorOptions};

// import our custom block mesh creator and types
use crate::mesh::{create_block_mesh, create_chunk_mesh, BlockType};
use crate::noise::terrain_height;

// import chunk structures
use crate::world::Chunk;

// import inventory
use crate::inventory::Inventory;



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


const COLLISION_EPSILON: f32 = 0.001;
const GROUND_PROXIMITY_THRESHOLD: f32 = 0.05;

fn is_solid_block(
    pos: IVec3,
    chunks: &Query<(&Transform, &Chunk), (Without<Player>, With<Chunk>)>,
) -> bool {
    let chunk_coord = IVec3::new(
        (pos.x as f32 / 16.0).floor() as i32,
        (pos.y as f32 / 16.0).floor() as i32,
        (pos.z as f32 / 16.0).floor() as i32,
    );
    let chunk_pos_block = chunk_coord * 16;
    for (chunk_transform, chunk) in chunks.iter() {
        if chunk_transform.translation.round().as_ivec3() == chunk_pos_block {
            let lx = pos.x - chunk_pos_block.x;
            let ly = pos.y - chunk_pos_block.y;
            let lz = pos.z - chunk_pos_block.z;
            if lx >= 0 && lx < 16 && ly >= 0 && ly < 16 && lz >= 0 && lz < 16 {
                let idx = (lx as usize) + (ly as usize) * 16 + (lz as usize) * 256;
                return chunk.blocks[idx] != BlockType::Air;
            }
        }
    }
    false
}

fn swept_aabb(
    p_start: Vec3,
    half_m: Vec3,
    dp: Vec3,
    b_center: Vec3,
    half_s: Vec3,
) -> Option<(f32, Vec3)> {
    let min_target = b_center - (half_s + half_m);
    let max_target = b_center + (half_s + half_m);

    let mut t_near = f32::NEG_INFINITY;
    let mut t_far = f32::INFINITY;
    let mut normal = Vec3::ZERO;

    for i in 0..3 {
        if dp[i] == 0.0 {
            if p_start[i] <= min_target[i] || p_start[i] >= max_target[i] {
                return None;
            }
        } else {
            let mut t1 = (min_target[i] - p_start[i]) / dp[i];
            let mut t2 = (max_target[i] - p_start[i]) / dp[i];

            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }

            if t1 > t_near {
                t_near = t1;
                normal = Vec3::ZERO;
                normal[i] = if dp[i] > 0.0 { -1.0 } else { 1.0 };
            }
            if t2 < t_far {
                t_far = t2;
            }
        }
    }

    if t_near > t_far {
        return None;
    }

    if t_near >= 1.0 || t_far <= 0.0 {
        return None;
    }

    let t_hit = t_near.max(0.0);
    Some((t_hit, normal))
}

fn raycast_down(
    origin: Vec3,
    chunks: &Query<(&Transform, &Chunk), (Without<Player>, With<Chunk>)>,
    max_distance: f32,
) -> Option<f32> {
    let bx = origin.x.round() as i32;
    let bz = origin.z.round() as i32;
    let start_y = origin.y.round() as i32;
    let end_y = (origin.y - max_distance).round() as i32 - 1;

    let mut highest_y: Option<i32> = None;
    for by in (end_y..=start_y).rev() {
        if is_solid_block(IVec3::new(bx, by, bz), chunks) {
            let top_surface = by as f32 + 0.5;
            if top_surface <= origin.y {
                highest_y = Some(by);
                break;
            }
        }
    }

    if let Some(by) = highest_y {
        let top_surface = by as f32 + 0.5;
        let distance = origin.y - top_surface;
        if distance <= max_distance {
            return Some(distance);
        }
    }
    None
}

fn check_grounded(
    position: Vec3,
    chunks: &Query<(&Transform, &Chunk), (Without<Player>, With<Chunk>)>,
) -> Option<f32> {
    let max_dist = PLAYER_HEIGHT * 0.5 + GROUND_PROXIMITY_THRESHOLD;
    if let Some(dist) = raycast_down(position, chunks, max_dist) {
        return Some(dist);
    }

    let half_w = PLAYER_WIDTH * 0.5;
    let offsets = [
        Vec3::new(-half_w, 0.0, -half_w),
        Vec3::new(-half_w, 0.0, half_w),
        Vec3::new(half_w, 0.0, -half_w),
        Vec3::new(half_w, 0.0, half_w),
    ];

    let mut min_dist: Option<f32> = None;
    for offset in offsets {
        if let Some(dist) = raycast_down(position + offset, chunks, max_dist) {
            if min_dist.is_none() || dist < min_dist.unwrap() {
                min_dist = Some(dist);
            }
        }
    }
    min_dist
}

pub fn apply_velocity(
    time: Res<Time>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut OnGround), With<Player>>,
    chunks: Query<(&Transform, &Chunk), (Without<Player>, With<Chunk>)>,
) {
    let (mut transform, mut velocity, mut on_ground) = player.single_mut().unwrap();
    let dt = time.delta_secs().min(0.03);
    let half = Vec3::new(PLAYER_WIDTH * 0.5, PLAYER_HEIGHT * 0.5, PLAYER_WIDTH * 0.5);

    // Apply gravity
    velocity.value.y = (velocity.value.y - 24.0 * dt).max(-30.0);

    let mut position = transform.translation;
    let mut current_velocity = velocity.value;
    let mut dp = current_velocity * dt;

    let mut resolved = false;
    for _iteration in 0..4 {
        if dp.length_squared() < 1e-8 {
            resolved = true;
            break;
        }

        let min_pos = Vec3::min(position - half, position + dp - half);
        let max_pos = Vec3::max(position + half, position + dp + half);

        let start_x = (min_pos.x - 0.5).floor() as i32 - 1;
        let end_x = (max_pos.x + 0.5).ceil() as i32 + 1;
        let start_y = (min_pos.y - 0.5).floor() as i32 - 1;
        let end_y = (max_pos.y + 0.5).ceil() as i32 + 1;
        let start_z = (min_pos.z - 0.5).floor() as i32 - 1;
        let end_z = (max_pos.z + 0.5).ceil() as i32 + 1;

        let mut earliest_collision: Option<(f32, Vec3)> = None;

        for bx in start_x..=end_x {
            for by in start_y..=end_y {
                for bz in start_z..=end_z {
                    if is_solid_block(IVec3::new(bx, by, bz), &chunks) {
                        let b_center = Vec3::new(bx as f32, by as f32, bz as f32);
                        let half_s = Vec3::splat(0.5);
                        if let Some((t, normal)) = swept_aabb(position, half, dp, b_center, half_s) {
                            if earliest_collision.is_none() || t < earliest_collision.unwrap().0 {
                                earliest_collision = Some((t, normal));
                            }
                        }
                    }
                }
            }
        }

        if let Some((t, normal)) = earliest_collision {
            position = position + dp * t + normal * COLLISION_EPSILON;

            let dp_remaining = dp * (1.0 - t);
            dp = dp_remaining - dp_remaining.dot(normal) * normal;
            current_velocity = current_velocity - current_velocity.dot(normal) * normal;
        } else {
            position += dp;
            resolved = true;
            break;
        }
    }

    if !resolved {
        position += dp;
    }

    let mut grounded = false;
    if current_velocity.y <= 0.0 {
        if let Some(dist) = check_grounded(position, &chunks) {
            grounded = true;
            current_velocity.y = 0.0;
            position.y = position.y - dist + (PLAYER_HEIGHT * 0.5);
        }
    }

    on_ground.value = grounded;
    velocity.value = current_velocity;
    transform.translation = position;
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
    inventory: Res<Inventory>,
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
                                            if let Some(block) = inventory.slots[inventory.selected_slot] {
                                                place_chunk.blocks[p_idx] = block;
                                            }
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


