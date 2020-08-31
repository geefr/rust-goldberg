
use serde_derive::*;

// Primitive serialisation
#[derive(Serialize,Deserialize,Debug)]
pub enum ColliderType {
    Cuboid,
    Ball,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct PrimitiveDefinition {
    pub name     : String,
    pub path_obj : String, // Path to the .obj file
    pub path_mtl : String, // Path to the directory containing the .mtl files
    pub scale    : [f32; 3],
    pub density : f32,
    pub restitution : f32,
    pub friction : f32,
    // TODO: Multiple types
    // TODO: How to represent this in json? For now just store all possible combinations of colliders here and sort it out later
    pub collider_def : Vec<ColliderDefinition>,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct ColliderDefinition {
    pub collider_type : ColliderType,
    pub origin : [f32; 3], // Center of the collider, relative to the primitive
    // Dimensions of the collider. Meaning varies based on collider type
    // Cuboid - half x,y,z
    // Sphere - half radius,_,_
    pub dimensions : [f32; 3],
}

// Scene/level serialisation
#[derive(Serialize,Deserialize,Debug)]
pub struct LevelDefinition {
    pub name : String,
    pub ground_dimensions : [f32; 2], // Width/Depth
    pub ground_colour : [f32; 3], // rgb, 0 -> 1
    pub background_colour : [f32; 3], // rgb, 0 -> 1
    pub primitives : Vec<LevelPrimitiveDefinition>,
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct LevelPrimitiveDefinition {
    pub name : String,
    pub position : [f32; 3],
    pub rotation : [f32; 3],
    pub is_static: bool, // If true the body is static, false dynamic
}
