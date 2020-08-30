extern crate kiss3d;
extern crate nalgebra as na;
use na::{Point3, Vector3,Isometry3,Translation3};

use ncollide3d::shape::{Cuboid, ShapeHandle};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, Ground,
};
use nphysics3d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

use kiss3d::light::Light;
use kiss3d::window::{Window,CanvasSetup,NumSamples};
use kiss3d::camera::{ArcBall};
use kiss3d::planar_camera::*;
use kiss3d::event::{Action, WindowEvent,MouseButton};

use std::collections::HashMap;
use std::time::Instant;
use std::env;

extern crate goldberg;
use goldberg::interactions::*;
use goldberg::engine::*;
use goldberg::types::*;

fn main() {
    let assets_path = String::from("/home/gareth/source/rust/olc-jam-2020/assets/");
    let mut level_file = assets_path.clone() + "/levels/default.json";

    let args: Vec<String> = env::args().collect();
    // TODO: This level loading is a mess, sort it out
    let mut level_definition : Option<LevelDefinition> = None;
    if args.len() > 1 {
        let passed_level_name = &args[1].clone();
        let passed_level_file = assets_path.clone() + "/levels/" + passed_level_name + ".json";

        level_file = passed_level_file;
        if let Some(ldef) = load_level_definition(&level_file) {
            level_definition = Some(ldef);
        } else {
            println!("WARNING: Failed to load level: {}", args[1]);
            level_definition = load_level_empty(passed_level_name);
        }
    }

    if !level_definition.is_some() {
        level_definition = load_level_empty("default");
    }

    // Init graphics
    let window = Window::new_with_setup("Goldberg: Geefr's Physics Playground", 1280, 1024, CanvasSetup {
        vsync : false,
        samples : NumSamples::Four,
    });

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
    let mut camera = ArcBall::new(Point3::new(5.0, 5.0, 5.0), Point3::new(0.0, 1.5, 0.0));
    camera.rebind_rotate_button(Some(MouseButton::Button2));
    camera.rebind_drag_button(Some(MouseButton::Button3));
    let planar_camera = FixedView::new();

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
        level_definition : level_definition.unwrap(),
        level_file,
        render_debug_extents : false,
    };

    // Setup the scene based on the level definition
    let ground_thickness = 1.0;
    let ground_collision_cuboid = Cuboid::new(
        Vector3::new(state.level_definition.ground_dimensions[0] / 2.0, ground_thickness / 2.0, state.level_definition.ground_dimensions[1] / 2.0)
    );
    let ground_shape = ShapeHandle::new(ground_collision_cuboid);
    let ground_handle = state.bodies.insert(Ground::new());
    let ground_collider = ColliderDesc::new(ground_shape)
        // .translation(Vector3::y() * - ground_thickness)
        .build(BodyPartHandle(ground_handle, 0));
    state.colliders.insert(ground_collider);

    let mut ground_geometry = state.window.add_cube(state.level_definition.ground_dimensions[0], ground_thickness, state.level_definition.ground_dimensions[1]);
    ground_geometry.append_translation(&Translation3::new(0.0, - ground_thickness, 0.0));
    ground_geometry.set_color(
        state.level_definition.ground_colour[0],
        state.level_definition.ground_colour[1],
        state.level_definition.ground_colour[2],
    );

    if state.render_debug_extents {
        ground_geometry.set_points_size(4.0);
        ground_geometry.set_lines_width(2.0);
        ground_geometry.set_surface_rendering_activation(false);
    }

    state.window.set_background_color(
        state.level_definition.background_colour[0],
        state.level_definition.background_colour[1],
        state.level_definition.background_colour[2],
    );
    
    state.add_primitives_from_level_definition();

    // Interactions
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
                let mut pos : Isometry3<f32> = na::convert_unchecked(*co.position());
                let collider_translation = Translation3::new(
                    - ent.collider_origin.x,
                    - ent.collider_origin.y,
                    - ent.collider_origin.z,
                );
                pos.append_translation_mut(&collider_translation);
                ent.node.set_local_transformation(pos);
            }
        }

        state.window.set_light(Light::Absolute(Point3::new(100.0, 100.0, 100.0)));

        // Let the interaction render what it needs (cursors etc)
        interaction.render(&mut state);

        state.window.render_with_camera(&mut state.camera); 
    }
}
