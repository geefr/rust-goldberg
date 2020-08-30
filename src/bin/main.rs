extern crate kiss3d;
extern crate nalgebra as na;
use na::{Point3, Vector3};

use ncollide3d::shape::{Cuboid, ShapeHandle};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, Ground,
};
use nphysics3d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

use kiss3d::light::Light;
use kiss3d::window::{Window};
use kiss3d::camera::{ArcBall};
use kiss3d::planar_camera::*;
use kiss3d::event::{Action, WindowEvent};


use std::collections::HashMap;
use std::time::Instant;

extern crate goldberg;
use goldberg::interactions::*;
use goldberg::engine::*;

fn main() {
    // Init graphics
    let mut window = Window::new("OLC Jam 2020");
    window.set_background_color(0.1, 0.1, 0.1);

    // Init physics
    let mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, -9.81, 0.0));
    let geometrical_world = DefaultGeometricalWorld::<f32>::new();
    let bodies = DefaultBodySet::<f32>::new();
    let colliders = DefaultColliderSet::new();
    let joint_constrants = DefaultJointConstraintSet::<f32>::new();
    let force_generators = DefaultForceGeneratorSet::<f32>::new();

    // TODO: We would use this, as needed for wasm compatibility,
    // but unfortunately we can't then render with anything except
    // the default camera...
    // TODO: We can get the default camera within the step function
    // but there's 2 problems
    // - We can only get the Trait, not the camera itself
    // - We can't replace the camera on the window at all, there's no method to do it whatsoever
    // window.render_loop(state);
    let camera = ArcBall::new(Point3::new(5.0, 5.0, 5.0), Point3::new(0.0, 1.5, 0.0));
    let planar_camera = FixedView::new();

    let assets_path = String::from("/home/gareth/source/rust/olc-jam-2020/assets/");
    let mut state = AppState {
        window,
        mechanical_world,
        geometrical_world,
        bodies,
        colliders,
        joint_constrants,
        force_generators,
        physics_entities : Vec::new(),
        simulation_start_time : Instant::now(),
        simulation_last_update_ms : 0.0,
        assets_path : assets_path.clone(),
        primitives_library : load_primitives_definitions(&assets_path).unwrap(),
        camera,
        planar_camera,
    };

    // Ground definition
    let ground_thickness = 0.2;
    let ground_width = 100.0;
    let ground_collision_cuboid = Cuboid::new(
        Vector3::new(ground_width, ground_thickness, ground_width)
    );
    let ground_shape = ShapeHandle::new(ground_collision_cuboid);
    let ground_handle = state.bodies.insert(Ground::new());
    let ground_collider = ColliderDesc::new(ground_shape)
        .translation(Vector3::y() * -ground_thickness)
        .build(BodyPartHandle(ground_handle, 0));
    state.colliders.insert(ground_collider);

    let mut ground_geometry = state.window.add_cube(ground_width, ground_thickness, ground_width);
    ground_geometry.set_color(0.9, 0.9, 0.9);

    // // TODO: Scene loading (dummy objects)
    // for x in -20..20 {
    //     for z in -20..20 {
    //         state.add_primitive("domino", &Vector3::new(x as f32, 0.01, z as f32));
    //     }
    // }

    // Interations
    let mut interactions : HashMap<String, Box<dyn Interaction>> = HashMap::new();
    interactions.insert(String::from("EditorMode"), Box::new(
        EditorModeInteraction::new(ground_collision_cuboid, "domino")
    ));

    let interaction = interactions.get_mut("EditorMode").unwrap();

    state.simulation_start_time = Instant::now();
    while !state.window.should_close() {
        for event in state.window.events().iter() {
            match event.value {
                WindowEvent::Key(k, Action::Press, modif) => {
                    interaction.on_key_down(&mut state, &k, &modif);
                },
                WindowEvent::Key(k, Action::Release, modif) => {
                    interaction.on_key_up(&mut state, &k, &modif);
                },
                WindowEvent::MouseButton(k, Action::Press, modif) => {
                    interaction.on_mouse_down(&mut state, &k, &modif);
                },
                WindowEvent::MouseButton(k, Action::Release, modif) => {
                    interaction.on_mouse_up(&mut state, &k, &modif);
                },
                WindowEvent::CursorPos(x, y, modif) => {
                    interaction.on_mouse_move(&mut state, x as f32, y as f32, &modif);
                }
                _ => {}
            }
        }

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

        state.window.set_light(Light::Absolute(Point3::new(100.0, 100.0, 100.0)));

        // Let the interaction render what it needs (cursors etc)
        interaction.render(&mut state);


        state.window.render_with_camera(&mut state.camera); 
    }
}
