use std::collections::HashMap;

use bevy::{
    prelude::*,
    render::{render_resource::PrimitiveTopology, mesh::Indices}
};
// import meshvertextattribute
use bevy_meshem::prelude::*;
use crate::{load_texture_atlas::TextureAtlas, block_types::BlockType};
use crate::chunk_manager::*;

/// Constants for us to use.
const _FACTOR: usize = 100;
const _MESHING_ALGORITHM: MeshingAlgorithm = MeshingAlgorithm::Culling;

#[derive(Component)]
struct Meshy {
    _ma: MeshingAlgorithm,
    _meta: MeshMD<u16>,
}

#[derive(Event, Default)]
struct RegenerateMesh;

#[derive(Resource)]
struct BlockRegistry {
    block: Vec<Mesh>,
}

/// The important part! Without implementing a [`VoxelRegistry`], you can't use the function.
impl VoxelRegistry for BlockRegistry {
    /// The type of our Voxel, the example uses u16 for Simplicity but you may have a struct
    /// Block { Name: ..., etc ...}, and you'll define that as the type, but encoding the block
    /// data onto simple type like u16 or u64 is probably prefferable.
    type Voxel = u16;
    /// The get_mesh function, probably the most important function in the
    /// [`VoxelRegistry`], it is what allows us to  quickly access the Mesh of each Voxel.
    fn get_mesh(&self, voxel: &Self::Voxel) -> VoxelMesh<&Mesh> {
        if *voxel == 0 {
            return VoxelMesh::Null;
        }
        VoxelMesh::NormalCube(&self.block[*voxel as usize])
    }
    /// Important function that tells our Algorithm if the Voxel is "full", for example, the Air
    /// in minecraft is not "full", but it is still on the chunk data, to singal there is nothing.
    fn is_covering(&self, voxel: &Self::Voxel, _side: Face) -> bool {
        return *voxel != 0;
    }
    /// The center of the Mesh, out mesh is defined in src/default_block.rs, just a constant.
    fn get_center(&self) -> [f32; 3] {
        return [0.5, 0.5, 0.5];
    }
    /// The dimensions of the Mesh, out mesh is defined in src/default_block.rs, just a constant.
    fn get_voxel_dimensions(&self) -> [f32; 3] {
        return [1.0, 1.0, 1.0];
    }
    /// The attributes we want to take from out voxels, note that using a lot of different
    /// attributes will likely lead to performance problems and unpredictible behaviour.
    /// We chose these 3 because they are very common, the algorithm does preserve UV data.
    fn all_attributes(&self) -> Vec<bevy::render::mesh::MeshVertexAttribute> {
        return vec![
            Mesh::ATTRIBUTE_POSITION,
            Mesh::ATTRIBUTE_UV_0,
            // generate a custom MeshVertexAttribute which maps a 16x16 texture to each face
            Mesh::ATTRIBUTE_NORMAL,
        ];
    }
}

fn _spawn_culled_grid(
    breg: Res<BlockRegistry>,
    texture_atlas: Res<TextureAtlas>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let grid = [1; _FACTOR * _FACTOR * _FACTOR];
    let dims: Dimensions = (_FACTOR, _FACTOR, _FACTOR);

    let (culled_mesh, metadata) = mesh_grid(
        dims,
        // Automatically cull the bottom when generating the mesh
        &[Bottom],
        &grid,
        breg.into_inner(),
        _MESHING_ALGORITHM,
        None,
    )
    .unwrap();
    let texture_handle = texture_atlas.handle.clone().unwrap();
    let culled_mesh_handle: Handle<Mesh> = meshes.add(culled_mesh.clone());
    commands.spawn((
        PbrBundle {
            mesh: culled_mesh_handle,
            material: materials.add(StandardMaterial {
                reflectance: 0.0,
                base_color_texture: Some(texture_handle),
                ..default()
            }),
            ..default()
        },
        Meshy {
            _ma: _MESHING_ALGORITHM,
            _meta: metadata,
        },
    ));
}

enum Side {
    Top,
    Bottom,
    Forward,
    Back,
    Left,
    Right,
}

pub struct BlockSpawnerPlugin;

impl Plugin for BlockSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_chunk)
            .insert_resource(BlockRegistry {
                block: vec![
                    Mesh::new(PrimitiveTopology::TriangleList),
                    generate_voxel_mesh(
                        [1.0, 1.0, 1.0],
                        [64, 32],
                        [
                            (Top, [1, 28]),
                            (Bottom, [1, 28]),
                            (Forward, [1, 28]),
                            (Back, [1, 28]),
                            (Left, [1, 28]),
                            (Right, [1, 28]),
                        ], // texture,
                        [0.5, 0.5, 0.5],
                        0.05,
                        Some(0.8),
                        1.0,
                    ),
                ],
            });
    }
}

/// spawn a 3d cube at the origin
fn spawn_chunk(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    _asset_server: Res<AssetServer>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
    mut chunks: ResMut<ChunkManager>,
) {
    const CHUNK_X: u16 = 16;
    const CHUNK_Y: u16 = 256;
    const CHUNK_Z: u16 = 16;

    const START_X: f32 = 0.0;
    const START_Y: f32 = 0.0;
    const START_Z: f32 = 0.0;
    
    let mut chunk = Chunk {
        position: Vec3::new(START_X, START_Y, START_Z),
        blocks: HashMap::default(),
    };

    for x in 0..CHUNK_X {
        for y in 0..CHUNK_Y {
            for z in 0..CHUNK_Z {
                let block = Block {
                    local_position: Vec3::new(x as f32, y as f32, z as f32),
                    absolute_position: Vec3::new(x as f32 + START_X, y as f32 + START_Y, z as f32 + START_Z),
                    is_visible: true,
                    block_type: BlockType::Dirt,
                    is_transparent: false,
                };
                chunk.blocks.insert(
                    Position::new(x as isize, y as isize, z as isize),
                    block,
                );
            }
        }
    }
    for block in chunk.blocks.values() {
        let block_above = chunk.blocks.get(&(block.local_position - Vec3::new(0.0, -1.0, 0.0)).into());
        let block_below = chunk.blocks.get(&(block.local_position - Vec3::new(0.0, 1.0, 0.0)).into());
        let block_left = chunk.blocks.get(&(block.local_position - Vec3::new(1.0, 0.0, 0.0)).into());
        let block_right = chunk.blocks.get(&(block.local_position + Vec3::new(1.0, 0.0, 0.0)).into());
        let block_front = chunk.blocks.get(&(block.local_position + Vec3::new(0.0, 0.0, 1.0)).into());
        let block_back = chunk.blocks.get(&(block.local_position - Vec3::new(0.0, 0.0, 1.0)).into());

        let mut sides = Vec::new();

        if block_above.is_none() || (block_above.is_some() && block_above.unwrap().is_transparent && block_above.unwrap().is_visible) {
            sides.push(Side::Top);
        }
        if block_below.is_none() || (block_below.is_some() && block_below.unwrap().is_transparent && block_below.unwrap().is_visible) {
            sides.push(Side::Bottom);
        }
        if block_left.is_none() || (block_left.is_some() && block_left.unwrap().is_transparent && block_left.unwrap().is_visible) {
            sides.push(Side::Left);
        }
        if block_right.is_none() || (block_right.is_some() && block_right.unwrap().is_transparent && block_right.unwrap().is_visible) {
            sides.push(Side::Right);
        }
        if block_front.is_none() || (block_front.is_some() && block_front.unwrap().is_transparent && block_front.unwrap().is_visible) {
            sides.push(Side::Forward);
        }
        if block_back.is_none() || (block_back.is_some() && block_back.unwrap().is_transparent && block_back.unwrap().is_visible) {
            sides.push(Side::Back);
        }
        
        let cube_mesh = create_cube_mesh(sides);
        let cube_handle = meshes.add(cube_mesh);
        commands.spawn(PbrBundle {
            mesh: cube_handle,
            material: _materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.7, 0.6),
                ..Default::default()
            }),
            transform: Transform::from_translation(block.absolute_position),
            ..Default::default()
        });
    }

    chunks.chunks.insert(Position::new(START_X as isize, START_Y as isize, START_Z as isize), chunk);
}

#[rustfmt::skip]
fn create_cube_mesh(sides: Vec<Side>) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut attribute_positions = Vec::new();
    let mut attribute_uvs = Vec::new();
    let mut attribute_normals = Vec::new();
    let mut indices = Vec::new();

    for side in sides {
        match side {
            Side::Top => {
                attribute_positions.extend_from_slice(&[
                    [-0.5, 0.5, -0.5], // vertex with index 0
                    [0.5, 0.5, -0.5], // vertex with index 1
                    [0.5, 0.5, 0.5], // etc. until 23
                    [-0.5, 0.5, 0.5],
                ]);
                attribute_uvs.extend_from_slice(&[
                    [0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0],
                ]);
                attribute_normals.extend_from_slice(&[
                    // Normals for the top side (towards +y)
                    [0.0, 1.0, 0.0],
                    [0.0, 1.0, 0.0],
                    [0.0, 1.0, 0.0],
                    [0.0, 1.0, 0.0],
                ]);
                indices.extend_from_slice(&[
                    0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
                ]);
            },
            Side::Bottom => {
                attribute_positions.extend_from_slice(&[
                    // bottom   (-y)
                    [-0.5, -0.5, -0.5],
                    [0.5, -0.5, -0.5],
                    [0.5, -0.5, 0.5],
                    [-0.5, -0.5, 0.5],
                ]);
                attribute_uvs.extend_from_slice(&[
                    [0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0],
                ]);
                attribute_normals.extend_from_slice(&[
                    [0.0, -1.0, 0.0],
                    [0.0, -1.0, 0.0],
                    [0.0, -1.0, 0.0],
                    [0.0, -1.0, 0.0],
                ]);
                indices.extend_from_slice(&[
                    4,5,7 , 5,6,7, // bottom (-y)
                ]);
            },
            Side::Right => {
                attribute_positions.extend_from_slice(&[
                    // right    (+x)
                    [0.5, -0.5, -0.5],
                    [0.5, -0.5, 0.5],
                    [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
                    [0.5, 0.5, -0.5],
                ]);
                attribute_uvs.extend_from_slice(&[
                    [0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0],
                ]);
                attribute_normals.extend_from_slice(&[
                    // Normals for the right side (towards +x)
                    [1.0, 0.0, 0.0],
                    [1.0, 0.0, 0.0],
                    [1.0, 0.0, 0.0],
                    [1.0, 0.0, 0.0],
                ]);

                indices.extend_from_slice(&[
                    8,11,9 , 9,11,10, // right (+x)
                ]);
            },
            Side::Left => {
                attribute_positions.extend_from_slice(&[
                    // left     (-x)
                    [-0.5, -0.5, -0.5],
                    [-0.5, -0.5, 0.5],
                    [-0.5, 0.5, 0.5],
                    [-0.5, 0.5, -0.5],
                ]);
                attribute_uvs.extend_from_slice(&[
                    [0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0],
                ]);
                attribute_normals.extend_from_slice(&[
                    [-1.0, 0.0, 0.0],
                    [-1.0, 0.0, 0.0],
                    [-1.0, 0.0, 0.0],
                    [-1.0, 0.0, 0.0],
                ]);
                indices.extend_from_slice(&[
                    12,13,15 , 13,14,15, // left (-x)
                ]);
            },
            Side::Back => {
                attribute_positions.extend_from_slice(&[
                    // back     (+z)
                    [-0.5, -0.5, 0.5],
                    [-0.5, 0.5, 0.5],
                    [0.5, 0.5, 0.5],
                    [0.5, -0.5, 0.5],
                ]);
                attribute_uvs.extend_from_slice(&[
                    [0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0],
                ]);
                attribute_normals.extend_from_slice(&[
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0, 1.0],
                ]);
                indices.extend_from_slice(&[
                    16,19,17 , 17,19,18, // back (+z)
                ]);
            },
            Side::Forward => {
                attribute_positions.extend_from_slice(&[
                    // forward  (-z)
                    [-0.5, -0.5, -0.5],
                    [-0.5, 0.5, -0.5],
                    [0.5, 0.5, -0.5],
                    [0.5, -0.5, -0.5],
                ]);
                attribute_uvs.extend_from_slice(&[
                    [0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0],
                ]);
                attribute_normals.extend_from_slice(&[
                    // Normals for the forward side (towards -z)
                    [0.0, 0.0, -1.0],
                    [0.0, 0.0, -1.0],
                    [0.0, 0.0, -1.0],
                    [0.0, 0.0, -1.0],
                ]);
                indices.extend_from_slice(&[
                    20,21,23 , 21,22,23, // forward (-z)
                ]);
            }
        }
    }
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, attribute_positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, attribute_uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, attribute_normals);
    mesh.set_indices(Some(Indices::U32(indices)));
    return mesh;
    /*Mesh::new(PrimitiveTopology::TriangleList)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // Each array is an [x, y, z] coordinate in local space.
        // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
        // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
        vec![
            // top (facing towards +y)
            [-0.5, 0.5, -0.5], // vertex with index 0
            [0.5, 0.5, -0.5], // vertex with index 1
            [0.5, 0.5, 0.5], // etc. until 23
            [-0.5, 0.5, 0.5],
            // bottom   (-y)
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [-0.5, -0.5, 0.5],
            // right    (+x)
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
            [0.5, 0.5, -0.5],
            // left     (-x)
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
            // back     (+z)
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
            // forward  (-z)
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
        ],
    )
    // Set-up UV coordinated to point to the upper (V < 0.5), "dirt+grass" part of the texture.
    // Take a look at the custom image (assets/textures/array_texture.png)
    // so the UV coords will make more sense
    // Note: (0.0, 0.0) = Top-Left in UV mapping, (1.0, 1.0) = Bottom-Right in UV mapping
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            // Assigning the UV coords for the top side.
            [0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0],
            // Assigning the UV coords for the bottom side.
            [0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0],
            // Assigning the UV coords for the right side.
            [0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0],
            // Assigning the UV coords for the left side.
            [0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0],
            // Assigning the UV coords for the back side.
            [0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0],
            // Assigning the UV coords for the forward side.
            [0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0],
        ],
    )
    // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
    // the surface.
    // Normals are required for correct lighting calculations.
    // Each array represents a normalized vector, which length should be equal to 1.0.
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // Normals for the back side (towards +z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // Normals for the forward side (towards -z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ],
    )
    // Create the triangles out of the 24 vertices we created.
    // To construct a square, we need 2 triangles, therefore 12 triangles in total.
    // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
    // by one, in a counter-clockwise order (relative to the position of the viewer, the order
    // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
    // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
    // further examples and the implementation of the built-in shapes.
    .with_indices(Some(Indices::U32(vec![
        0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
        4,5,7 , 5,6,7, // bottom (-y)
        8,11,9 , 9,11,10, // right (+x)
        12,13,15 , 13,14,15, // left (-x)
        16,19,17 , 17,19,18, // back (+z)
        20,21,23 , 21,22,23, // forward (-z)
    ])));*/
}