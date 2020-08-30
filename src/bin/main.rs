extern crate kiss3d;
extern crate nalgebra as na;
use na::{Point3, RealField, Vector3, UnitQuaternion,Isometry};

use ncollide3d::shape::{Cuboid, ShapeHandle};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, DefaultColliderHandle, Ground, RigidBodyDesc,
};
use nphysics3d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::{Window};
use kiss3d::camera::ArcBall;

use std::time::Instant;
use std::path::Path;
use std::fs::{self, File};
use std::io::BufReader;
use std::io;

extern crate goldberg;
use goldberg::types::*;

struct PhysicsEntity {
    collider : DefaultColliderHandle,
    node : SceneNode,
}
struct AppState {
    mechanical_world: DefaultMechanicalWorld<f32>,
    geometrical_world: DefaultGeometricalWorld::<f32>,
    bodies: DefaultBodySet::<f32>,
    colliders: DefaultColliderSet<f32>,
    joint_constrants: DefaultJointConstraintSet::<f32>,
    force_generators: DefaultForceGeneratorSet::<f32>,
    physics_entities: Vec<PhysicsEntity>,
    simulation_start_time : Instant,
    simulation_last_update_ms: f32,
}

fn load_primitives_definitions( assets_path : &String ) -> io::Result<Vec<PrimitiveDefinition>> {
    let mut results = Vec::new();
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
                let prim = serde_json::from_reader(reader)?;
                results.push(prim);
            }
        }
    }

    Ok(results)
}

fn main() {
    // Init graphics
    let mut window = Window::new("OLC Jam 2020");
    window.set_background_color(0.1, 0.1, 0.1);

    let primitives = load_primitives_definitions(&String::from("/home/gareth/source/rust/olc-jam-2020/assets/")).unwrap();
    for prim in primitives {
        println!("{} : {}", prim.name, serde_json::to_string(&prim).unwrap());
    }

    // Init physics
    let mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, -9.81, 0.0));
    let geometrical_world = DefaultGeometricalWorld::<f32>::new();
    let mut bodies = DefaultBodySet::<f32>::new();
    let mut colliders = DefaultColliderSet::new();
    let joint_constrants = DefaultJointConstraintSet::<f32>::new();
    let force_generators = DefaultForceGeneratorSet::<f32>::new();

    let mut state = AppState {
        mechanical_world,
        geometrical_world,
        bodies,
        colliders,
        joint_constrants,
        force_generators,
        physics_entities : Vec::new(),
        simulation_start_time : Instant::now(),
        simulation_last_update_ms : 0.0,
    };

    // Ground definition
    let ground_thickness = 0.2;
    let ground_width = 100.0;
    let ground_shape = ShapeHandle::new(Cuboid::new(
        Vector3::new(ground_width, ground_thickness, ground_width)
    ));
    let ground_handle = state.bodies.insert(Ground::new());
    let co = ColliderDesc::new(ground_shape)
        .translation(Vector3::y() * -ground_thickness)
        .build(BodyPartHandle(ground_handle, 0));
    state.colliders.insert(co);

    let mut ground_geometry = window.add_cube(ground_width, ground_thickness, ground_width);
    ground_geometry.set_color(0.9, 0.9, 0.9);

    // Create boxes
    let num = 6;
    let rad = 0.1;
    let cuboid = ShapeHandle::new(Cuboid::new(Vector3::repeat(rad)));

    let shift = (rad + ColliderDesc::<f32>::default_margin()) * 2.0;
    let centerx = shift * (num as f32) / 20.0;
    let centery = shift / 2.0;
    let centerz = shift * (num as f32) / 2.0;
    let height = 3.0;

    for i in 0usize..num {
        for j in 0usize..num {
            for k in 0usize..num {
                let x = i as f32 * shift - centerx;
                let y = j as f32 * shift + centery + height;
                let z = k as f32 * shift - centerz;

                // Build the rigid body.
                let rb = RigidBodyDesc::new()
                    .translation(Vector3::new(x, y, z))
                    .build();
                let rb_handle = state.bodies.insert(rb);

                // Build the collider.
                let co = ColliderDesc::new(cuboid.clone())
                    .density(1.0)
                    .build(BodyPartHandle(rb_handle, 0));

                let collision_handle = state.colliders.insert(co);
                
                let mut cubey = window.add_obj(
                    Path::new("/home/gareth/source/rust/olc-jam-2020/assets/models/primitives/domino.obj"),
                    Path::new("/home/gareth/source/rust/olc-jam-2020/assets/models/primitives/"),
                    Vector3::new(rad, rad, rad)
                );
                
                // let mut cubey = window.add_cube(rad, rad, rad);
                // cubey.set_color(1.0, 0.0, 0.0);

                state.physics_entities.push(PhysicsEntity{
                    collider : collision_handle,
                    node : cubey,
                });
            }
        }
    }

    state.simulation_start_time = Instant::now();

    // TODO: We would use this, as needed for wasm compatibility,
    // but unfortunately we can't then render with anything except
    // the default camera...
    // TODO: We can get the default camera within the step function
    // but there's 2 problems
    // - We can only get the Trait, not the camera itself
    // - We can't replace the camera on the window at all, there's no method to do it whatsoever
    // window.render_loop(state);
    let mut camera = ArcBall::new(Point3::new(5.0, 5.0, 5.0), Point3::new(0.0, 1.5, 0.0));
    
    while !window.should_close() {
        let simulation_elapsed_ms : f32 = state.simulation_start_time.elapsed().as_millis() as f32;
        let simulation_delta = simulation_elapsed_ms - state.simulation_last_update_ms;
        state.simulation_last_update_ms = simulation_elapsed_ms;

        state.mechanical_world.set_timestep( simulation_delta / 1000.0 );
        state.mechanical_world.step(
            &mut state.geometrical_world,
            &mut state.bodies,
            &mut state.colliders,
            &mut state.joint_constrants,
            &mut state.force_generators,
        );

        for ent in &mut state.physics_entities {
            if let Some(co) = &state.colliders.get(ent.collider) {
                let pos = na::convert_unchecked(*co.position());
                ent.node.set_local_transformation(pos);
            }
        }

        window.set_light(Light::Absolute(Point3::new(100.0, 100.0, 100.0)));
        window.render_with_camera(&mut camera); 
    }
}
