// src/player.rs

use bevy::prelude::*;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::MessageReader;
use bevy::window::{CursorGrabMode, CursorOptions};

// add cube id
use bevy::math::primitives::Cuboid;


// for add a terrain height
use crate::world::{
    spawn_block,
    terrain_height,
    Block,
    BlockAssets,
};


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



// struct for higlyting the player sellect and break or add block. :)
#[derive(Component)]
pub struct SellectBlock;

#[derive(Component)]
pub struct BlockHighlight;

// volocity 
#[derive(Component)]
pub struct Velocity {
    pub value: Vec3,
}

const JUMP_FORCE: f32 = 18.5;
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
            Transform::default(),
        )).with_children(|pivot| {
            pivot.spawn((
                Camera3d::default(),
                GameCamera,
                CameraView {
                    third_person: false,
                },
                Transform::from_xyz(0.0, 1.6, 0.0),
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

    let direction = direction.normalize_or_zero() * 5.0;
    velocity.value.x = direction.x;
    velocity.value.z = direction.z;
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
            Transform::from_xyz(0.0, 1.6, 5.0).looking_at(Vec3::new(0.0, 0.9, 0.0), Vec3::Y)
        } else {
            Transform::from_xyz(0.0, 1.6, 0.0)
        };
    }
}


// function for mouse look means a player can look around fixed.
pub fn mouse_look(
    mut mouse_events: MessageReader<MouseMotion>,
    mut player: Query<(&mut Transform, &mut LookAngles), With<Player>>,
    mut pivot: Query<&mut Transform, (With<CameraPivot>, Without<Player>)>
) {


    let (mut player_transform, mut angles) = player.single_mut().unwrap();
    let mut pivot_transform = pivot.single_mut().unwrap();

    for event in mouse_events.read() {
        angles.yaw -= event.delta.x * 0.003;
        angles.pitch -= event.delta.y * 0.003;
        angles.pitch = angles.pitch.clamp(-1.54, 1.54);

        player_transform.rotation = Quat::from_rotation_y(angles.yaw);

        pivot_transform.rotation = Quat::from_rotation_x(angles.pitch);
    };
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
    blocks: Query<&Transform, (With<Block>, Without<Player>)>,
) {
    let (mut transform, mut velocity, mut on_ground) = player.single_mut().unwrap();
    let dt = time.delta_secs().min(0.03);
    let half = Vec3::new(PLAYER_WIDTH * 0.5, PLAYER_HEIGHT * 0.5, PLAYER_WIDTH * 0.5);
    on_ground.value = false;
    velocity.value.y = (velocity.value.y - 24.0 * dt).max(-30.0);

    if velocity.value.x != 0.0 {
        transform.translation.x += velocity.value.x * dt;
        for block in blocks.iter() {
            let d = transform.translation - block.translation;
            if d.x.abs() < half.x + 0.5 && d.y.abs() < half.y + 0.5 && d.z.abs() < half.z + 0.5 {
                transform.translation.x = block.translation.x - velocity.value.x.signum() * (half.x + 0.5);
                velocity.value.x = 0.0;
                break;
            }
        }
    }

    transform.translation.y += velocity.value.y * dt;
    for block in blocks.iter() {
        let d = transform.translation - block.translation;
        if d.x.abs() < half.x + 0.5 && d.y.abs() < half.y + 0.5 && d.z.abs() < half.z + 0.5 {
            on_ground.value = velocity.value.y < 0.0;
            transform.translation.y = block.translation.y - velocity.value.y.signum() * (half.y + 0.5);
            velocity.value.y = 0.0;
            break;
        }
    }

    if velocity.value.z != 0.0 {
        transform.translation.z += velocity.value.z * dt;
        for block in blocks.iter() {
            let d = transform.translation - block.translation;
            if d.x.abs() < half.x + 0.5 && d.y.abs() < half.y + 0.5 && d.z.abs() < half.z + 0.5 {
                transform.translation.z = block.translation.z - velocity.value.z.signum() * (half.z + 0.5);
                velocity.value.z = 0.0;
                break;
            }
        }
    }
}

pub fn select_block(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    block_assets: Res<BlockAssets>,
    camera: Query<&GlobalTransform, With<GameCamera>>,
    blocks: Query<(Entity, &Transform), With<Block>>,
    selected_blocks: Query<Entity, With<SellectBlock>>,
) {
    for entity in selected_blocks.iter() {
        commands.entity(entity).remove::<SellectBlock>();
    }

    let camera = camera.single().unwrap();
    let forward = camera.forward();
    let left = mouse.just_pressed(MouseButton::Left);
    let right = mouse.just_pressed(MouseButton::Right);
    let mut last = camera.translation();
    let mut distance = 0.0;

    while distance <= 6.0 {
        let point = camera.translation() + forward * distance;
        for (entity, block) in blocks.iter() {
            let d = point - block.translation;
            if d.x.abs() <= 0.5 && d.y.abs() <= 0.5 && d.z.abs() <= 0.5 {
                if left {
                    commands.entity(entity).despawn();
                } else {
                    commands.entity(entity).insert(SellectBlock);
                    if right {
                        spawn_block(&mut commands, last.round(), block_assets.mesh.clone(), block_assets.material.clone());
                    }
                }
                return;
            }
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
    let highlight_mesh = meshes.add(Cuboid::new(1.02, 1.02, 1.02));

    let highlight_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.25),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    commands.spawn((
        BlockHighlight,
        Mesh3d(highlight_mesh),
        MeshMaterial3d(highlight_material),
        Transform::default(),
        Visibility::Hidden,
    ));
}


// update highy light funct
pub fn update_block_highlight(
    selected_blocks: Query<&Transform, With<SellectBlock>>,
    mut highlight: Query<
        (&mut Transform, &mut Visibility),
        (With<BlockHighlight>, Without<SellectBlock>),
    >,
) {
    let (mut highlight_transform, mut visibility) =
        highlight.single_mut().unwrap();

    if let Ok(selected_transform) = selected_blocks.single() {
        highlight_transform.translation = selected_transform.translation;

        *visibility = Visibility::Visible;
    } else {
        *visibility = Visibility::Hidden;
    }
}


