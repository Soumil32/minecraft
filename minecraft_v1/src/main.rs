use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.7, 1.0)))
        .insert_resource(AmbientLight {
            color: Color::rgb(0.8, 0.8, 0.8),
            brightness: 1.0,
        })
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (spawn_cube, spawn_camera, spawn_sun))
        .run();

}


// spawn a 3d cube at the origin
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

fn spawn_sun(mut commands: Commands) {
    let light = DirectionalLightBundle {
        transform: Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };

    commands.spawn(light);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}