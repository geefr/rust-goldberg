
use three_d::{VertexBuffer, ElementBuffer,Vec2,Vec3,Vec4,Mat4,Mesh};

extern crate nalgebra as na;
use nphysics3d::object::{DefaultBodyHandle, DefaultColliderHandle,BodyStatus};

// pub struct RenderableMaterial {
//     pub emissive_factor : Vec3,
//     pub base_colour : Vec4,
//     pub metallic_roughness_factor : Vec2,
// }

pub enum RenderMethod {
  DrawArrays,
  DrawElements,
}

pub struct RenderableMesh {
    pub name     : String,
    //pub material  : RenderableMaterial,
    pub bounds_min: Vec3,
    pub bounds_max: Vec3,
    pub mesh : Mesh,
}

pub struct Renderable {
    pub meshes   : Vec<RenderableMesh>,
    pub children : Vec<Renderable>,
    pub transform: Mat4,
}

pub struct EntityRigidBody {
    pub renderable : Renderable,
    pub scale : Vec3,
    pub active_transform : Mat4,
    pub phys_body_status : BodyStatus,
    pub phys_body : Option<DefaultBodyHandle>,
    pub phys_collider : Option<DefaultColliderHandle>,
    pub collider_origin : Vec3,
    pub collider_dimensions : Vec3,
}
