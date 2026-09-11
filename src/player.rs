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
    let Ok((transform, mut velocity, on_ground)) = player.single_mut() else {
        return;
    };
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

    let Ok((mut player_transform, mut angles)) = player.single_mut() else {
        return;
    };
    let Ok(mut pivot_transform) = pivot.single_mut() else {
        return;
    };

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


type ChunkQuery<'w, 's> = Query<'w, 's, (&'static Transform, &'static Chunk), (Without<Player>, With<Chunk>)>;

fn to_block(v: f32) -> i32 {
    (v + 0.5).floor() as i32
}

fn is_solid_block(pos: IVec3, chunks: &ChunkQuery) -> bool {
    let chunk_coord = IVec3::new(
        pos.x.div_euclid(16),
        pos.y.div_euclid(16),
        pos.z.div_euclid(16),
    );
    let chunk_pos_block = chunk_coord * 16;
    for (chunk_transform, chunk) in chunks.iter() {
        if chunk_transform.translation.round().as_ivec3() == chunk_pos_block {
            let lx = pos.x.rem_euclid(16) as usize;
            let ly = pos.y.rem_euclid(16) as usize;
            let lz = pos.z.rem_euclid(16) as usize;
            let idx = lx + ly * 16 + lz * 256;
            return chunk.blocks[idx] != BlockType::Air;
        }
    }
    false
}

pub fn apply_velocity(
    time: Res<Time>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut OnGround), With<Player>>,
    chunks: ChunkQuery,
) {
    let Ok((mut transform, mut velocity, mut on_ground)) = player.single_mut() else {
        return;
    };
    let dt = time.delta_secs().min(0.03);
    let half = Vec3::new(PLAYER_WIDTH * 0.5, PLAYER_HEIGHT * 0.5, PLAYER_WIDTH * 0.5);

    // Apply gravity
    velocity.value.y = (velocity.value.y - 24.0 * dt).max(-30.0);

    let mut position = transform.translation;
    let mut current_velocity = velocity.value;
    let dp = current_velocity * dt;

    let mut grounded = false;
    const EPS: f32 = 0.001;

    // 1. Vertical resolution (Y axis)
    if dp.y != 0.0 {
        let min_x = to_block(position.x - half.x + EPS);
        let max_x = to_block(position.x + half.x - EPS);
        let min_z = to_block(position.z - half.z + EPS);
        let max_z = to_block(position.z + half.z - EPS);

        if dp.y < 0.0 {
            let start_by = to_block(position.y - half.y - EPS);
            let target_by = to_block(position.y + dp.y - half.y);
            let mut hit_top: Option<f32> = None;

            for by in (target_by..=start_by).rev() {
                for bx in min_x..=max_x {
                    for bz in min_z..=max_z {
                        if is_solid_block(IVec3::new(bx, by, bz), &chunks) {
                            hit_top = Some(by as f32 + 0.5);
                            break;
                        }
                    }
                    if hit_top.is_some() {
                        break;
                    }
                }
                if hit_top.is_some() {
                    break;
                }
            }

            if let Some(top) = hit_top {
                position.y = top + half.y;
                current_velocity.y = 0.0;
                grounded = true;
            } else {
                position.y += dp.y;
            }
        } else {
            let start_by = to_block(position.y + half.y + EPS);
            let target_by = to_block(position.y + dp.y + half.y);
            let mut hit_bottom: Option<f32> = None;

            for by in start_by..=target_by {
                for bx in min_x..=max_x {
                    for bz in min_z..=max_z {
                        if is_solid_block(IVec3::new(bx, by, bz), &chunks) {
                            hit_bottom = Some(by as f32 - 0.5);
                            break;
                        }
                    }
                    if hit_bottom.is_some() {
                        break;
                    }
                }
                if hit_bottom.is_some() {
                    break;
                }
            }

            if let Some(bottom) = hit_bottom {
                position.y = bottom - half.y;
                current_velocity.y = 0.0;
            } else {
                position.y += dp.y;
            }
        }
    } else {
        let min_x = to_block(position.x - half.x + EPS);
        let max_x = to_block(position.x + half.x - EPS);
        let min_z = to_block(position.z - half.z + EPS);
        let max_z = to_block(position.z + half.z - EPS);
        let check_by = to_block(position.y - half.y - 0.05);

        for bx in min_x..=max_x {
            for bz in min_z..=max_z {
                if is_solid_block(IVec3::new(bx, check_by, bz), &chunks) {
                    grounded = true;
                    break;
                }
            }
            if grounded {
                break;
            }
        }
    }

    // 2. Horizontal resolution (X axis)
    if dp.x != 0.0 {
        let min_y = to_block(position.y - half.y + EPS);
        let max_y = to_block(position.y + half.y - EPS);
        let min_z = to_block(position.z - half.z + EPS);
        let max_z = to_block(position.z + half.z - EPS);

        if dp.x < 0.0 {
            let start_bx = to_block(position.x - half.x - EPS);
            let target_bx = to_block(position.x + dp.x - half.x);
            let mut hit_right: Option<f32> = None;

            for bx in (target_bx..=start_bx).rev() {
                for by in min_y..=max_y {
                    for bz in min_z..=max_z {
                        if is_solid_block(IVec3::new(bx, by, bz), &chunks) {
                            hit_right = Some(bx as f32 + 0.5);
                            break;
                        }
                    }
                    if hit_right.is_some() {
                        break;
                    }
                }
                if hit_right.is_some() {
                    break;
                }
            }

            if let Some(right) = hit_right {
                position.x = right + half.x;
                current_velocity.x = 0.0;
            } else {
                position.x += dp.x;
            }
        } else {
            let start_bx = to_block(position.x + half.x + EPS);
            let target_bx = to_block(position.x + dp.x + half.x);
            let mut hit_left: Option<f32> = None;

            for bx in start_bx..=target_bx {
                for by in min_y..=max_y {
                    for bz in min_z..=max_z {
                        if is_solid_block(IVec3::new(bx, by, bz), &chunks) {
                            hit_left = Some(bx as f32 - 0.5);
                            break;
                        }
                    }
                    if hit_left.is_some() {
                        break;
                    }
                }
                if hit_left.is_some() {
                    break;
                }
            }

            if let Some(left) = hit_left {
                position.x = left - half.x;
                current_velocity.x = 0.0;
            } else {
                position.x += dp.x;
            }
        }
    }

    // 3. Horizontal resolution (Z axis)
    if dp.z != 0.0 {
        let min_x = to_block(position.x - half.x + EPS);
        let max_x = to_block(position.x + half.x - EPS);
        let min_y = to_block(position.y - half.y + EPS);
        let max_y = to_block(position.y + half.y - EPS);

        if dp.z < 0.0 {
            let start_bz = to_block(position.z - half.z - EPS);
            let target_bz = to_block(position.z + dp.z - half.z);
            let mut hit_front: Option<f32> = None;

            for bz in (target_bz..=start_bz).rev() {
                for bx in min_x..=max_x {
                    for by in min_y..=max_y {
                        if is_solid_block(IVec3::new(bx, by, bz), &chunks) {
                            hit_front = Some(bz as f32 + 0.5);
                            break;
                        }
                    }
                    if hit_front.is_some() {
                        break;
                    }
                }
                if hit_front.is_some() {
                    break;
                }
            }

            if let Some(front) = hit_front {
                position.z = front + half.z;
                current_velocity.z = 0.0;
            } else {
                position.z += dp.z;
            }
        } else {
            let start_bz = to_block(position.z + half.z + EPS);
            let target_bz = to_block(position.z + dp.z + half.z);
            let mut hit_back: Option<f32> = None;

            for bz in start_bz..=target_bz {
                for bx in min_x..=max_x {
                    for by in min_y..=max_y {
                        if is_solid_block(IVec3::new(bx, by, bz), &chunks) {
                            hit_back = Some(bz as f32 - 0.5);
                            break;
                        }
                    }
                    if hit_back.is_some() {
                        break;
                    }
                }
                if hit_back.is_some() {
                    break;
                }
            }

            if let Some(back) = hit_back {
                position.z = back - half.z;
                current_velocity.z = 0.0;
            } else {
                position.z += dp.z;
            }
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

    let Ok(camera) = camera.single() else {
        return;
    };
    let Ok(player_transform) = player.single() else {
        return;
    };

    let forward = camera.forward();
    let left = mouse.just_pressed(MouseButton::Left);
    let right = mouse.just_pressed(MouseButton::Right);
    let mut last = camera.translation();
    let mut distance = 0.0;

    while distance <= 6.0 {
        let point = camera.translation() + forward * distance;
        let block_i = IVec3::new(to_block(point.x), to_block(point.y), to_block(point.z));
        let block_pos = block_i.as_vec3();

        let chunk_coord = IVec3::new(
            block_i.x.div_euclid(16),
            block_i.y.div_euclid(16),
            block_i.z.div_euclid(16),
        );
        let lx = block_i.x.rem_euclid(16) as usize;
        let ly = block_i.y.rem_euclid(16) as usize;
        let lz = block_i.z.rem_euclid(16) as usize;

        let mut hit = false;
        for (_entity, chunk_transform, mut chunk, mesh3d) in chunks.iter_mut() {
            let chunk_pos_block = chunk_coord * 16;
            if chunk_transform.translation.round().as_ivec3() == chunk_pos_block {
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
                        let diff = last - block_pos;
                        let hit_normal = if diff.x.abs() >= diff.y.abs() && diff.x.abs() >= diff.z.abs() {
                            IVec3::new(diff.x.signum() as i32, 0, 0)
                        } else if diff.y.abs() >= diff.z.abs() {
                            IVec3::new(0, diff.y.signum() as i32, 0)
                        } else {
                            IVec3::new(0, 0, diff.z.signum() as i32)
                        };

                        let place_i = block_i + hit_normal;
                        let place = place_i.as_vec3();
                        let d = player_transform.translation - place;

                        let in_player = d.x.abs() < PLAYER_WIDTH * 0.5 + 0.5
                            && d.y.abs() < PLAYER_HEIGHT * 0.5 + 0.5
                            && d.z.abs() < PLAYER_WIDTH * 0.5 + 0.5;

                        if !in_player {
                            let place_chunk_coord = IVec3::new(
                                place_i.x.div_euclid(16),
                                place_i.y.div_euclid(16),
                                place_i.z.div_euclid(16),
                            );
                            let plx = place_i.x.rem_euclid(16) as usize;
                            let ply = place_i.y.rem_euclid(16) as usize;
                            let plz = place_i.z.rem_euclid(16) as usize;

                            for (_place_entity, place_chunk_transform, mut place_chunk, place_mesh3d) in chunks.iter_mut() {
                                let place_chunk_pos_block = place_chunk_coord * 16;
                                if place_chunk_transform.translation.round().as_ivec3() == place_chunk_pos_block {
                                    let p_idx = plx + ply * 16 + plz * 256;
                                    if let Some(block) = inventory.slots[inventory.selected_slot] {
                                        place_chunk.blocks[p_idx] = block;
                                    }
                                    if let Some(mut mesh) = meshes.get_mut(&place_mesh3d.0) {
                                        *mesh = create_chunk_mesh(&place_chunk.blocks);
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

        if hit {
            return;
        }

        last = point;
        distance += 0.05;
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
    let Ok((mut highlight_transform, mut visibility)) = highlight.single_mut() else {
        return;
    };

    if let Some(pos) = selected_block.pos {
        highlight_transform.translation = pos;
        *visibility = Visibility::Visible;
    } else {
        *visibility = Visibility::Hidden;
    }
}


