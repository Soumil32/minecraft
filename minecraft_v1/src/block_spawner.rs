use bevy::prelude::*;

pub struct BlockSpawnerPlugin;

impl Plugin for BlockSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cubes);
    }
}

/// spawn a 3d cube at the origin
fn spawn_cubes(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut _materials: ResMut<Assets<StandardMaterial>>
) {
    
    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    // load in the texture for the cube from the assets folder
    let texture_handle = asset_server.load("textures/blocks/stone.png");
    let material = StandardMaterial {
        base_color_texture: Some(texture_handle),
        ..Default::default()
    };

    let bundle = PbrBundle {
        material: _materials.add(material),
        mesh: cube_mesh,
        transform: Transform::from_scale(Vec3::splat(1.0)),
        ..Default::default()
    };
    
    for x in 0..10 {
        for y in 0..10 {
            for z in 0..10 {
                let position = Vec3::new(
                    (x*1) as f32 + 0.5,
                    (y*1) as f32 + 0.5,
                    (z*1) as f32 + 0.5
                );
                let mut bundle = bundle.clone();
                bundle.transform.translation = position;
                commands.spawn(bundle);
            }
        }
    }
}