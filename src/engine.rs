extern crate kiss3d;
extern crate nalgebra as na;
use na::{Point3, Vector3};

use ncollide3d::shape::{Cuboid, ShapeHandle};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, DefaultColliderHandle, RigidBodyDesc,
};
use nphysics3d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

use kiss3d::scene::SceneNode;
use kiss3d::window::{Window};
use kiss3d::camera::ArcBall;
use kiss3d::planar_camera::*;
use kiss3d::text::Font;

use std::collections::HashMap;
use std::time::Instant;
use std::path::Path;
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::io::BufReader;


use crate::types::*;

// Global state
pub struct PhysicsEntity {
    pub collider : DefaultColliderHandle,
    pub node : SceneNode,
}
pub struct AppState {
    pub window : Window,
    pub assets_path : String,
    pub mechanical_world: DefaultMechanicalWorld<f32>,
    pub geometrical_world: DefaultGeometricalWorld::<f32>,
    pub bodies: DefaultBodySet::<f32>,
    pub colliders: DefaultColliderSet<f32>,
    pub joint_constrants: DefaultJointConstraintSet::<f32>,
    pub force_generators: DefaultForceGeneratorSet::<f32>,
    pub physics_entities: Vec<PhysicsEntity>,
    pub simulation_start_time : Instant,
    pub simulation_last_update_ms: f32,
    pub primitives_library : HashMap<String, PrimitiveDefinition>,
    pub camera : ArcBall,
    pub planar_camera : FixedView,
    // TODO: Having this here duplicates a load of stuff, but makes it easy to save the level at the end
    pub level_definition : LevelDefinition,
    pub level_file : String,
}
impl AppState {
    pub fn add_primitive( &mut self, name : &str, position : &Vector3<f32>, rotation : &Vector3<f32> ) -> bool {
        // Log the primitive in the level definition
        self.level_definition.primitives.push(
            LevelPrimitiveDefinition{
                name : String::from(name),
                position : [position.x, position.y, position.z],
                rotation : [rotation.x, rotation.y, rotation.z],
            }
        );

        if let Some(prim) = self.primitives_library.get_mut(name) {
            // TODO: we only handle cuboids for now
            match &prim.collider_type {
                ColliderType::Cuboid => {
                    // Build the rigid body.
                    let collider_pos = Vector3::from(prim.collider_def.origin);
                    let collider_dim = Vector3::from(prim.collider_def.dimensions);
                    let prim_scale = Vector3::from(prim.scale);
                
                    let rb = RigidBodyDesc::new()
                        .translation(*position)
                        .rotation(*rotation)
                        .build();
                    let rb_handle = self.bodies.insert(rb);
                
                    let cuboid = ShapeHandle::new(Cuboid::new(collider_dim));
                
                    // Build the collider.
                    let co = ColliderDesc::new(cuboid)
                        .density(1.0)
                        .set_translation(collider_pos)
                        .build(BodyPartHandle(rb_handle, 0));
                
                    let collision_handle = self.colliders.insert(co);
                
                    let gfx = self.window.add_obj(
                        Path::new(&format!("{}/{}", self.assets_path, prim.path_obj)),
                        Path::new(&format!("{}/{}", self.assets_path, prim.path_mtl)),
                        prim_scale,
                    );
                
                    self.physics_entities.push(PhysicsEntity{
                        collider : collision_handle,
                        node : gfx,
                    });
                    //self.add_primitive_cuboid( &prim, position )
                }
            }
            return true;
        }
        false
    }


    pub fn draw_hud_text( &mut self, text : &str, position : &na::Point2<f32>, size: f32 ) {
        let font = Font::default();
        // Text coordinates are in pixels, from top-left
        self.window.draw_text(
            &text,
            &position,
            size,
            &font,
            &Point3::new(1.0, 1.0, 1.0),
        );
    }

    // TODO: Should just do this in a constructor or such
    pub fn add_primitives_from_level_definition(&mut self) {
        // TODO: Hack around borrowing issues, should learn the correct pattern for this
        let prims = self.level_definition.primitives.clone();
        for prim in prims {
            self.add_primitive(&prim.name,&Vector3::from(prim.position), &Vector3::from(prim.rotation));
        }
    }
}

// Engine + functions
pub fn load_primitives_definitions( assets_path : &String ) -> io::Result<HashMap<String, PrimitiveDefinition>> {
    let mut results = HashMap::new();
    let prim_path = format!("{}{}", assets_path, "/primitives");
    let primitives_path = Path::new(&prim_path);
    for entry in fs::read_dir(primitives_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let ext = match path.extension() {
                Some(x) => x,
                None => continue
            };
            if ext == "json" {
                let json_file = File::open(path)?;
                let reader = BufReader::new(json_file);
                let prim : PrimitiveDefinition = serde_json::from_reader(reader)?;
                results.insert(prim.name.clone(), prim);
            }
        }
    }

    Ok(results)
}

pub fn load_level_definition( level_file : &String ) -> Option<LevelDefinition> {
    let path = Path::new(level_file);
    if !path.is_file() { return None; }
    
    let ext = match path.extension() {
        Some(x) => x,
        None => return None
    };
    if ext != "json" { return None; }
    
    let json_file = match File::open(path) {
        Ok(x) => x,
        _ => return None
    };
    let reader = BufReader::new(json_file);
    let level : LevelDefinition = match serde_json::from_reader(reader) {
        Ok(x) => x,
        _ => return None
    };
    Some(level)
}

pub fn load_level_empty( name : &str ) -> Option<LevelDefinition> {
    Some(LevelDefinition {
        name : String::from(name),
        ground_dimensions : [100.0, 100.0],
        ground_colour : [0.9, 0.9, 0.9],
        background_colour : [0.1,0.1,0.1],
        primitives : Vec::new(),
    })
}

pub fn save_level_definition( level : &LevelDefinition, level_file : &String ) {
    let level_str = serde_json::to_string_pretty(level).unwrap();
    let mut file = match File::create(level_file) {
        Ok(x) => x,
        Err(e) => {
            println!("ERROR: Failed to save level: {:?}", e);
            return;
        }
    };
    file.write_all(level_str.as_bytes()).unwrap();
}
