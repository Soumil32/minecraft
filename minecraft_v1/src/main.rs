use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_cube, spawn_camera))
        .run();

}


// spawn a 3d cube at the origin
fn spawn_cube(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut _images: ResMut<Assets<Image>>,
    mut _materials: ResMut<Assets<StandardMaterial>>
) {
    
    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    let bundle = PbrBundle {
        mesh: cube_mesh,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    };
    commands.spawn(bundle);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}