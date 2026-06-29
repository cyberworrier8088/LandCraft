use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn setup_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Camera3d::default(),
        Transform::from_xyz(8.0, 8.0, 16.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
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


pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut().unwrap();

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
        println!("W");
    }

    if keyboard.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
        println!("S");
    }

    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
        println!("A");
    }

    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
        println!("D");
    }


    let speed = 5.0;

    transform.translation += direction.normalize_or_zero() * speed * time.delta_secs();
}