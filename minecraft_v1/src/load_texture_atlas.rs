use bevy::prelude::*;

#[derive(Resource)]
pub struct TextureAtlas {
    pub handle: Option<Handle<Image>>,
}

pub struct LoadTextureAtlasPlugin;

impl Plugin for LoadTextureAtlasPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(TextureAtlas { handle: None })
            .add_systems(Startup, load_texture_atlas);
    }
}

pub fn load_texture_atlas(
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<TextureAtlas>,
) {
    let texture_handle = asset_server.load("textures/blocks/texture_atlas.png");
    texture_atlas.handle = Some(texture_handle);
}