use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

/// Creates a voxel block mesh with custom vertices, normals, and UV coordinates
/// mapped to a 2-tile horizontal texture atlas (Left = Cobblestone, Right = Grass).
pub fn create_block_mesh() -> Mesh {
    // Correctly initialize the mesh with TriangleList topology and default render asset usages
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    // 24 vertices for the 6 faces of the cube (4 vertices per face)
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            // Front (Z = +0.5)
            [-0.5, -0.5,  0.5],
            [ 0.5, -0.5,  0.5],
            [ 0.5,  0.5,  0.5],
            [-0.5,  0.5,  0.5],

            // Back (Z = -0.5)
            [ 0.5, -0.5, -0.5],
            [-0.5, -0.5, -0.5],
            [-0.5,  0.5, -0.5],
            [ 0.5,  0.5, -0.5],

            // Left (X = -0.5)
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5,  0.5],
            [-0.5,  0.5,  0.5],
            [-0.5,  0.5, -0.5],

            // Right (X = +0.5)
            [ 0.5, -0.5,  0.5],
            [ 0.5, -0.5, -0.5],
            [ 0.5,  0.5, -0.5],
            [ 0.5,  0.5,  0.5],

            // Top (Y = +0.5)
            [-0.5,  0.5,  0.5],
            [ 0.5,  0.5,  0.5],
            [ 0.5,  0.5, -0.5],
            [-0.5,  0.5, -0.5],

            // Bottom (Y = -0.5)
            [-0.5, -0.5, -0.5],
            [ 0.5, -0.5, -0.5],
            [ 0.5, -0.5,  0.5],
            [-0.5, -0.5,  0.5],
        ],
    );

    // 36 indices defining the 12 triangles of the cube
    mesh.insert_indices(Indices::U32(vec![
        // Front
        0, 1, 2,
        0, 2, 3,
        
        // Back
        4, 5, 6,
        4, 6, 7,

        // Left
        8, 9, 10,
        8, 10, 11,

        // Right
        12, 13, 14,
        12, 14, 15,

        // Top
        16, 17, 18,
        16, 18, 19,

        // Bottom
        20, 21, 22,
        20, 22, 23,
    ]));

    // 24 face-normal vectors pointing outward from the cube's center
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            // Front
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],

            // Back
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],

            // Left
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],

            // Right
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],

            // Top
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],

            // Bottom
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
        ],
    );

    // 24 texture coordinates. 
    // Left tile (Cobblestone) is [0.0, 0.5] on the U axis.
    // Right tile (Grass) is [0.5, 1.0] on the U axis.
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            // Front (Cobblestone)
            [0.0, 1.0],
            [0.5, 1.0],
            [0.5, 0.0],
            [0.0, 0.0],

            // Back (Cobblestone)
            [0.0, 1.0],
            [0.5, 1.0],
            [0.5, 0.0],
            [0.0, 0.0],

            // Left (Cobblestone)
            [0.0, 1.0],
            [0.5, 1.0],
            [0.5, 0.0],
            [0.0, 0.0],

            // Right (Cobblestone)
            [0.0, 1.0],
            [0.5, 1.0],
            [0.5, 0.0],
            [0.0, 0.0],

            // Top (Grass)
            [0.5, 1.0],
            [1.0, 1.0],
            [1.0, 0.0],
            [0.5, 0.0],

            // Bottom (Cobblestone)
            [0.0, 1.0],
            [0.5, 1.0],
            [0.5, 0.0],
            [0.0, 0.0],
        ],
    );

    mesh
}