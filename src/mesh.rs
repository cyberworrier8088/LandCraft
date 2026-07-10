use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

/// Represents the type of block to generate a mesh for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Cobblestone,
    Grass,
}

/// Creates a voxel block mesh with custom vertices, normals, and UV coordinates
/// mapped to a 2-tile horizontal texture atlas (Left = Cobblestone, Right = Grass).
pub fn create_block_mesh(block_type: BlockType) -> Mesh {
    // Initialize the mesh with TriangleList topology and default render asset usages
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
    // The atlas contains:
    // - Left half (U: 0.0 to 0.5): Cobblestone texture
    // - Right half (U: 0.5 to 1.0): Grass texture
    let cobble_uvs = [
        [0.0, 1.0], [0.5, 1.0], [0.5, 0.0], [0.0, 0.0]
    ];
    let grass_uvs = [
        [0.5, 1.0], [1.0, 1.0], [1.0, 0.0], [0.5, 0.0]
    ];

    let uv_vec = match block_type {
        BlockType::Cobblestone => {
            // Cobblestone block has Cobblestone texture on all 6 faces
            let mut uvs = Vec::with_capacity(24);
            for _ in 0..6 {
                uvs.extend_from_slice(&cobble_uvs);
            }
            uvs
        }
        BlockType::Grass => {
            // Grass block has Grass texture on all 6 faces
            let mut uvs = Vec::with_capacity(24);
            for _ in 0..6 {
                uvs.extend_from_slice(&grass_uvs);
            }
            uvs
        }
    };

    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv_vec);

    mesh
}

