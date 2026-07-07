use bevy::prelude::*;
use bevy::math::primitives::Cuboid;


// importing player file some struct
use crate::player::Player;

// component markers for articulation queries
#[derive(Component)]
pub struct PlayerRoot;

#[derive(Component)]
pub struct HeadPivot;

#[derive(Component)]
pub struct LeftArmPivot;

#[derive(Component)]
pub struct RightArmPivot;

#[derive(Component)]
pub struct LeftLegPivot;

#[derive(Component)]
pub struct RightLegPivot;

pub fn setup_player_root(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // structural Primitive Layout (1.0 Unit = 1 Block / 16 Pixels :))
    let head_mesh = meshes.add(Cuboid::new(0.5, 0.5, 0.5));
    let torso_mesh = meshes.add(Cuboid::new(0.5, 0.75, 0.25));
    let limb_mesh = meshes.add(Cuboid::new(0.25, 0.75, 0.25)); // arms/Legs share bounds

    // steve color palette
    let skin_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.74, 0.55, 0.43),
        ..default()
    });
    let shirt_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.68, 0.68), 
        ..default()
    });
    let pants_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.26, 0.31, 0.65), 
        ..default()
    });

    // spawn entity hierarchy
    // root anchor tracks the bottom center of the feet at ground level (Y = 0.0)
    commands.spawn((
        PlayerRoot,
        Transform::from_xyz(8.0, 0.0, 8.0),
        Visibility::default(),
    ))
    .with_children(|parent| {
        
        // stationary center body element
        parent.spawn((
            Mesh3d(torso_mesh),
            MeshMaterial3d(shirt_mat.clone()),
            Transform::from_xyz(0.0, 1.125, 0.0),
        ));

        // head pivots at base of neck
        parent.spawn((
            HeadPivot,
            Transform::from_xyz(0.0, 1.5, 0.0),
            Visibility::default(),
        )).with_children(|head| {
            head.spawn((
                Mesh3d(head_mesh),
                MeshMaterial3d(skin_mat.clone()),
                Transform::from_xyz(0.0, 0.25, 0.0), // up by half height
            ));
        });

        // left arm pivots at top-left shoulder
        parent.spawn((
            LeftArmPivot,
            Transform::from_xyz(-0.375, 1.5, 0.0),
            Visibility::default(),
        )).with_children(|arm| {
            arm.spawn((
                Mesh3d(limb_mesh.clone()),
                MeshMaterial3d(shirt_mat.clone()),
                Transform::from_xyz(0.0, -0.375, 0.0), // shift down relative to pivot
            ));
        });

        // right arm pivots at top-right shoulder
        parent.spawn((
            RightArmPivot,
            Transform::from_xyz(0.375, 1.5, 0.0),
            Visibility::default(),
        )).with_children(|arm| {
            arm.spawn((
                Mesh3d(limb_mesh.clone()),
                MeshMaterial3d(shirt_mat.clone()),
                Transform::from_xyz(0.0, -0.375, 0.0),
            ));
        });

        // left leg pivots at left hip socket
        parent.spawn((
            LeftLegPivot,
            Transform::from_xyz(-0.125, 0.75, 0.0),
            Visibility::default(),
        )).with_children(|leg| {
            leg.spawn((
                Mesh3d(limb_mesh.clone()),
                MeshMaterial3d(pants_mat.clone()),
                Transform::from_xyz(0.0, -0.375, 0.0),
            ));
        });

        // right leg pivots at right hip socket
        parent.spawn((
            RightLegPivot,
            Transform::from_xyz(0.125, 0.75, 0.0),
            Visibility::default(),
        )).with_children(|leg| {
            leg.spawn((
                Mesh3d(limb_mesh),
                MeshMaterial3d(pants_mat),
                Transform::from_xyz(0.0, -0.375, 0.0),
            ));
        });
    });
}


pub fn sync_player_model(
    player: Query<&Transform, (With<Player>, Without<PlayerRoot>)>,
    mut model: Query<&mut Transform, (With<PlayerRoot>, Without<Player>)>
) {
    let player_transform = player.single().unwrap();
    let mut model_transform = model.single_mut().unwrap();

    model_transform.translation = player_transform.translation;
}