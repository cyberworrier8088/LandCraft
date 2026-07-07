// src/player.rs

use bevy::prelude::*;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::MessageReader;
use bevy::window::{CursorGrabMode, CursorOptions};

// add cube id
use bevy::math::primitives::Cuboid;


// ca;lling block to use it. 
use crate::world::{spawn_block, Block, BlockAssets};


#[derive(Component)]
pub struct Player;


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
        Camera3d::default(),
        Transform::from_xyz(8.0, 8.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),

        
    ));

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
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut().unwrap();

    let mut direction = Vec3::ZERO;

    let mut forward = *transform.forward();
    forward.y = 0.0;
    forward = forward.normalize_or_zero();
    
    let mut right = *transform.right();
    right.y = 0.0;
    right = right.normalize_or_zero();



    if keyboard.pressed(KeyCode::KeyW) {
        direction += forward;
        println!("W");
    }

    if keyboard.pressed(KeyCode::KeyS) {
        direction -= forward;
        println!("S");
    }

    if keyboard.pressed(KeyCode::KeyA) {
        direction -= right;
        println!("A");
    }

    if keyboard.pressed(KeyCode::KeyD) {
        direction += right;
        println!("D");
    }


    let speed = 5.0;

    transform.translation += direction.normalize_or_zero() * speed * time.delta_secs();
}



// function for mouse look means a player can look around fixed.
pub fn mouse_look(
    mut mouse_events: MessageReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut LookAngles), With<Player>>,
) {


    let (mut transform, mut angles) = query.single_mut().unwrap();

    for event in mouse_events.read() {
        angles.yaw -= event.delta.x * 0.003;
        angles.pitch -= event.delta.y * 0.003;
        angles.pitch = angles.pitch.clamp(-1.54, 1.54);

        transform.rotation = Quat::from_euler(
            EulerRot::YXZ,
            angles.yaw,
            angles.pitch,
            0.0,
        );

        println!("Yaw: {}, Pitch: {}", angles.yaw, angles.pitch);
    };
}



// function for lock cursor means mouse cursor lock not visible :) 
pub fn lock_cursor(
    mut cursor_options: Single<&mut CursorOptions>,
) {
    cursor_options.grab_mode = CursorGrabMode::Locked;
    cursor_options.visible = false; // cursor hidding
}


// function ffor detect block means block how much
pub fn detect_block(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    block_assets: Res<BlockAssets>,
    player: Query<&Transform, With<Player>>,
    blocks: Query<(Entity, &Transform), With<Block>>,

) {
    // mouse butter preess  geting fuunction
    let left_click = mouse.just_pressed(MouseButton::Left);
    let right_click = mouse.just_pressed(MouseButton::Right);

    if !left_click && !right_click {
        return;
    }

    let player_transform = player.single().unwrap();

    let forward = *player_transform.forward();

    let ray_distance = 5.0;
    let step_size = 0.1;
    let mut distance = 0.0;

    let mut previous_point = player_transform.translation;


    // this is for raycasting.
    'raycast: while distance <= ray_distance {
        
        let point = player_transform.translation + forward * distance;

        for (block_entity, block_transform) in blocks.iter() {
            let block_position = point.distance(block_transform.translation);


            if block_position < 0.5 {
                println!("HIT BLOCK!: {:?}", block_entity);

                if left_click {
                    commands.entity(block_entity).despawn();
                }

                if right_click {
                    let place_position = previous_point.round();

                    spawn_block(
                        &mut commands,
                        place_position,
                        block_assets.mesh.clone(),
                        block_assets.material.clone(),
                    );
                }

                break 'raycast;
            }
        
        }

        // for cheaking block at the perticuler point.
        println!("Ray Point: {:?}", point);
        previous_point = point;
        distance += step_size;
        
    }


}


pub fn apply_gravity(
    time: Res<Time>,
    mut player: Query<&mut Velocity, With<Player>>,

) {
    let mut velocity = player.single_mut().unwrap();

    let gravity = -9.81;

    velocity.value.y += gravity * time.delta_secs();
}

pub fn apply_velocity(
    time: Res<Time>,
    mut player: Query<(&mut Transform, &Velocity), With<Player>>,
) {
    let (mut transform, velocity) = player.single_mut().unwrap();

    transform.translation += velocity.value * time.delta_secs();
}



// adding grond collision
pub fn ground_collision(
    mut player: Query<(&mut Transform, &mut Velocity,  &mut OnGround), With<Player>>,
    blocks: Query<&Transform, (With<Block>, Without<Player>)>,
) {
    
    let (mut player_transform, mut velocity, mut on_ground) = player.single_mut().unwrap();

    on_ground.value = false;

    let player_half_height = 0.9;

    let feet_position = player_transform.translation - Vec3::Y * player_half_height;


    for block_transform in blocks.iter() {
        let horizontal_distance = Vec2::new(
            feet_position.x - block_transform.translation.x,
            feet_position.z - block_transform.translation.z,
        ).length();

        let block_top = block_transform.translation.y + 0.5;


        let vertical_distance = feet_position.y - block_top;


        if horizontal_distance < 0.5
        && vertical_distance <= 0.0
        && velocity.value.y <= 0.0
        {
            player_transform.translation.y = block_top + player_half_height;

            velocity.value.y = 0.0;

            on_ground.value = true;
        }

        println!("Horizontal: {}, Vertical: {}", horizontal_distance, vertical_distance);
    }

    println!("FEET: {:?}", feet_position);
}


// player block touch time highylight function
pub fn select_block(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    blocks: Query<(Entity, &Transform), With<Block>>,
    selected_blocks: Query<Entity, With<SellectBlock>>,
) {
    // remove selection from the previosly selected block.
    for entity in selected_blocks.iter() {
        commands.entity(entity).remove::<SellectBlock>();
    }

    let player_transform = player.single().unwrap();


    let forward = *player_transform.forward();

    let ray_distance = 5.0;
    let step_size = 0.1;
    let mut distance = 0.0;

    'raycast: while distance <= ray_distance {
        let point = player_transform.translation + forward * distance;

        for (block_entity, block_transform) in blocks.iter() {
            let distance_to_block = point.distance(block_transform.translation);

            if distance_to_block < 0.5 {
                commands.entity(block_entity).insert(SellectBlock);
                println!("Selected Block: {:?}", block_entity);
                break 'raycast;
            }
        }

        distance += step_size;
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


//
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



