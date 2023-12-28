use bevy::prelude::*;

pub struct BlockSpawnerPlugin;

impl Plugin for BlockSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cube);
    }
}

/// spawn a 3d cube at the origin
fn spawn_cube(mut commands: Commands,
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
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    };
    commands.spawn(bundle);
}