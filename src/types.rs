

use serde_derive::*;

#[derive(Serialize,Deserialize,Debug)]
pub enum ColliderType {
    Cuboid,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct PrimitiveDefinition {
    pub name     : String,
    pub path_obj : String, // Path to the .obj file
    pub path_mtl : String, // Path to the directory containing the .mtl files
    pub scale    : [f32; 3],
    pub collider_type : ColliderType,
    pub collider_def  : ColliderDefinitionCuboid, // TODO: Multiple types
}

#[derive(Serialize,Deserialize,Debug)]
pub struct ColliderDefinitionCuboid {
    pub origin : [f32; 3], // Center of the collider, relative to the primitive
    pub dimensions : [f32; 3], // Dimensions of the collider (Half w/h/d)
}
