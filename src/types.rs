
use serde_derive::*;

// Primitive serialisation
#[derive(Serialize,Deserialize,Debug)]
pub enum ColliderType {
    Cuboid,
    CompositeCuboid,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct PrimitiveDefinition {
    pub name     : String,
    pub path_obj : String, // Path to the .obj file
    pub path_mtl : String, // Path to the directory containing the .mtl files
    pub scale    : [f32; 3],
    pub collider_type : ColliderType,
    // TODO: Multiple types
    // TODO: How to represent this in json? For now just store all possible combinations of colliders here and sort it out later
    pub collider_def  : ColliderDefinitionCuboid,
    pub collider_def_composite_cuboid : Vec<ColliderDefinitionCuboid>,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct ColliderDefinitionCuboid {
    pub origin : [f32; 3], // Center of the collider, relative to the primitive
    pub dimensions : [f32; 3], // Dimensions of the collider (Half w/h/d)
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
}
