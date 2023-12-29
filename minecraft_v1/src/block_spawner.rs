use bevy::{prelude::*, render::{mesh::Indices, render_resource::{PrimitiveTopology, Face}}};
// import meshvertextattribute
use bevy_meshem::prelude::*;

/// Constants for us to use.
const FACTOR: usize = 100;
const MESHING_ALGORITHM: MeshingAlgorithm = MeshingAlgorithm::Culling;


#[derive(Component)]
struct Meshy {
    ma: MeshingAlgorithm,
    meta: MeshMD<u16>,
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
    fn is_covering(&self, voxel: &Self::Voxel, _side: prelude::Face) -> bool {
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

fn spawn_culled_grid(
    breg: Res<BlockRegistry>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let grid = [1; FACTOR * FACTOR * FACTOR];
    let dims: Dimensions = (FACTOR, FACTOR, FACTOR);

    let (culled_mesh, metadata) = mesh_grid(
        dims,
        // Automatically cull the bottom when generating the mesh
        &[Bottom],
        &grid,
        breg.into_inner(),
        MESHING_ALGORITHM,
        None,
    )
    .unwrap();
    let texture_handle = asset_server.load("textures/blocks/dirt.png");
    let culled_mesh_handle: Handle<Mesh> = meshes.add(culled_mesh.clone());
    commands.spawn((
        PbrBundle {
            mesh: culled_mesh_handle,
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle),
                ..default()
            }),
            ..default()
        },
        Meshy {
            ma: MESHING_ALGORITHM,
            meta: metadata,
        },
    ));
}

pub struct BlockSpawnerPlugin;

impl Plugin for BlockSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.
            add_systems(Startup, spawn_culled_grid)
            .insert_resource(BlockRegistry {
                block: vec![
                            Mesh::new(PrimitiveTopology::TriangleList),
                            generate_voxel_mesh(
                                [1.0, 1.0, 1.0],
                                [1, 1],
                                [
                                    (Top, [0, 0]),
                                    (Bottom, [0, 0]),
                                    (Forward, [0, 0]),
                                    (Back, [0, 0]),
                                    (Left, [0, 0]),
                                    (Right, [0, 0]),
                                ], // texture,
                                [0.5, 0.5, 0.5],
                                0.05,
                                Some(0.8),
                                1.0,
                            )
                            ],
            });
    }
}

/// spawn a 3d cube at the origin
fn _spawn_cubes(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut _materials: ResMut<Assets<StandardMaterial>>
) {
    
    let cube_mesh = meshes.add(_create_cube_mesh());
    // load in the texture for the cube from the assets folder
    let texture_handle = asset_server.load("textures/blocks/stone.png");
    let material = StandardMaterial {
        base_color_texture: Some(texture_handle),
        cull_mode: Some(Face::Back),
        ..Default::default()
    };

    let bundle = PbrBundle {
        material: _materials.add(material),
        mesh: cube_mesh,
        transform: Transform::from_scale(Vec3::splat(1.0)),
        ..Default::default()
    };
    
    for x in 0..100 {
        for y in 0..10 {
            for z in 0..100 {
                let position = Vec3::new(
                    (x*1) as f32,
                    (y*1) as f32,
                    (z*1) as f32
                );
                let mut bundle = bundle.clone();
                bundle.transform.translation = position;
                commands.spawn(bundle);
            }
        }
    }
}

#[rustfmt::skip]
fn _create_cube_mesh() -> Mesh {
    Mesh::new(PrimitiveTopology::TriangleList)
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
    ])))
}