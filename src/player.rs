// src/player.rs

use bevy::prelude::*;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::MessageReader;
use bevy::window::{CursorGrabMode, CursorOptions};


// ca;lling block to use it. 
use crate::world::Block;


#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct LookAngles {
    pub yaw: f32, // yaw means left and right
    pub pitch: f32, // pitch means up and down
}



// setup player function 
pub fn setup_player(mut commands: Commands) {
    commands.spawn((
        Player,
        LookAngles {
            yaw: 0.0,
            pitch: 0.0,
        },
        Camera3d::default(),
        Transform::from_xyz(8.0, 8.0, 16.0).looking_at(Vec3::ZERO, Vec3::Y),
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

    let forward = *transform.forward();

    let right = *transform.right();



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
    blocks: Query<&Transform, With<Block>>,
) {
    println!("Blocks: {}", blocks.iter().count());
}