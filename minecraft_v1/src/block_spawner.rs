use bevy::{
    prelude::*,
    render::render_resource::PrimitiveTopology
};
// import meshvertextattribute
use bevy_meshem::prelude::*;
use crate::load_texture_atlas::TextureAtlas;

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

fn spawn_culled_grid(
    breg: Res<BlockRegistry>,
    texture_atlas: Res<TextureAtlas>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
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
            ma: MESHING_ALGORITHM,
            meta: metadata,
        },
    ));
}

pub struct BlockSpawnerPlugin;

impl Plugin for BlockSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_culled_grid)
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
