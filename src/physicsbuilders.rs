use crate::datatypes::{EntityRigidBody,Renderable};

extern crate nalgebra as na;
use nphysics3d::object::*;
use ncollide3d::shape::{ShapeHandle, Compound,Cuboid};
use nphysics3d::material::*;
use three_d::{vec3,vec4};

pub fn phys_build_rigid_body( body : &mut EntityRigidBody, body_set : &mut DefaultBodySet<f32>, collider_set : &mut DefaultColliderSet<f32> ) {
    // TODO: Parameterisationness
    // Collider shape - can be plane, ball, cuboid, all sorts, and compound shapes. Comes from ncollide3d
    // TODO: For now translate the geometry directly into the collider (Performance warning)
    let mut shapes = Vec::new();
    get_collider_shapes(&body, &body.renderable, &mut shapes);

    if !shapes.is_empty() {
        let body_pos = vec4(0.0,0.0,0.0,1.0);
        let body_pos = body.active_transform * body_pos;
        let rigid_body_translation = na::Vector3::new(body_pos.x, body_pos.y, body_pos.z);

        // https://www.nphysics.org/rigid_body_simulations_with_contacts/
        let rigid_body = RigidBodyDesc::new()
            .translation(rigid_body_translation)
            .linear_damping(0.01)
            .angular_damping(0.01)
            .mass(1.0)
            .status(body.phys_body_status)
            .build();

        body.phys_body = Some(body_set.insert(rigid_body));

        let compound = ShapeHandle::new(Compound::new(shapes));

        let collider = ColliderDesc::new(compound)
            // Physics material (restitution, friction)
            .material(MaterialHandle::new(BasicMaterial::new(0.3,0.8)))
            .density(1.0) // Required for angular calculations on boxes to work
            .build(BodyPartHandle(body.phys_body.unwrap(), 0));
        body.phys_collider = Some(collider_set.insert(collider));
    }
}

fn get_collider_shapes(body : &EntityRigidBody, renderable : &Renderable, shapes : &mut Vec<(na::Isometry3<f32>, ShapeHandle<f32>)> ) {
    for mesh in &renderable.meshes {
        // Convert min/max bounds of mesh to center + size

        // let center = (mesh.bounds_min + mesh.bounds_max) / 2.0;
        // let dim = mesh.bounds_max - center;
        
        // let center = vec4(center.x, center.y, center.z, 1.0);
        // let dim = vec4(dim.x, dim.y, dim.z, 1.0);

        // let center = renderable.transform * center;
        // let dim = renderable.transform * dim;

        // println!("Collider transform: {:?}", renderable.transform);
        let center = body.collider_origin;
        let dim = body.collider_dimensions;
        println!("Collider center: {:?}", center);
        println!("Collider dim: {:?}", dim);

        let delta = na::Isometry3::new(na::Vector3::new(center.x, center.y, center.z), na::zero());
        let collision_shape = ShapeHandle::new(Cuboid::new(na::Vector3::new(dim.x, dim.y, dim.z)));
        shapes.push((delta, collision_shape));
    }
    
    // recurse
    for child in &renderable.children {
        get_collider_shapes(body, &child, shapes);
    }
}