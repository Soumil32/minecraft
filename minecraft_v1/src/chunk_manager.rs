use bevy::prelude::*;
use std::collections::HashMap;

pub struct ChunkManagerPlugin;

impl Plugin for ChunkManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ChunkManager {
                chunks: HashMap::default(),
            });
    }
}

#[derive(Resource)]
pub struct ChunkManager {
    pub chunks: HashMap<Position, Chunk>,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Position {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}

impl Into<Vec3> for Position {
    fn into(self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

impl From<Vec3> for Position {
    fn from(vec: Vec3) -> Self {
        Self {
            x: vec.x as isize,
            y: vec.y as isize,
            z: vec.z as isize,
        }
    }
}

pub struct Chunk {
    /// position of the chunk based on the top left corner block
    pub position: Vec3,
    pub blocks: HashMap<Position, Block>,
}

#[derive(Component)]
pub struct Block {
    pub local_position: Vec3, 
    pub absolute_position: Vec3,
    pub visible: bool,
}