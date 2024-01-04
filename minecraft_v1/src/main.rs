mod block_spawner;
mod player_movement;
mod load_texture_atlas;
mod chunk_manager;
mod block_types;

use bevy::{prelude::*, pbr::wireframe::{WireframePlugin, WireframeConfig}};
use bevy_flycam::prelude::*;
use block_spawner::BlockSpawnerPlugin;
use player_movement::PlayerMovementPlugin;
use load_texture_atlas::LoadTextureAtlasPlugin;
use chunk_manager::ChunkManagerPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.7, 1.0)))
        .insert_resource(AmbientLight {
            color: Color::rgb(0.8, 0.8, 0.8),
            brightness: 1.0,
        })
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            BlockSpawnerPlugin,
            PlayerMovementPlugin,
            LoadTextureAtlasPlugin,
            ChunkManagerPlugin,
            WireframePlugin,
            PlayerPlugin,
        ))
        .add_systems(Startup, (
            spawn_sun, 
            use_wireframe
        ))
        .run();

}

fn spawn_sun(mut commands: Commands) {
    let light = DirectionalLightBundle {
        transform: Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };

    commands.spawn(light);
}

fn use_wireframe(mut wireframe_config: ResMut<WireframeConfig>) {
    wireframe_config.global = true;
}

fn _spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 50.0, 50.0),
        ..Default::default()
    });
}