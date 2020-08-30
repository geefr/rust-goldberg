extern crate kiss3d;
extern crate nalgebra as na;
use na::{Point2, Point3, Vector3};

use ncollide3d::shape::{Cuboid, ShapeHandle};
use ncollide3d::query::{Ray, RayCast};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, DefaultColliderHandle, Ground, RigidBodyDesc,
};
use nphysics3d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::{Window};
use kiss3d::camera::{Camera,ArcBall};
use kiss3d::planar_camera::*;
use kiss3d::event::{Action, Key, MouseButton, WindowEvent, Modifiers};
use kiss3d::text::Font;

use std::collections::HashMap;
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
    window : Window,
    assets_path : String,
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

fn load_primitives_definitions( assets_path : &String ) -> io::Result<HashMap<String, PrimitiveDefinition>> {
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

fn add_primitive( state : &mut AppState, map : &HashMap<String, PrimitiveDefinition>, name : &str, position : &Vector3<f32> ) -> bool {
    if let Some(prim) = map.get(name) {
        // TODO: we only handle cuboids for now
        match &prim.collider_type {
            ColliderType::Cuboid => add_primitive_cuboid( state, prim, position )
        }
        return true;
    }
    false
}

fn add_primitive_cuboid( state : &mut AppState, prim : &PrimitiveDefinition, position : &Vector3<f32> ) {
    // Build the rigid body.
    let collider_pos = Vector3::from(prim.collider_def.origin);
    let collider_dim = Vector3::from(prim.collider_def.dimensions);
    let prim_scale = Vector3::from(prim.scale);

    let rb = RigidBodyDesc::new()
        .translation(*position)
        .build();
    let rb_handle = state.bodies.insert(rb);

    let cuboid = ShapeHandle::new(Cuboid::new(collider_dim));

    // Build the collider.
    let mut co = ColliderDesc::new(cuboid)
        .density(1.0)
        .set_translation(collider_pos)
        .build(BodyPartHandle(rb_handle, 0));

    let collision_handle = state.colliders.insert(co);

    let mut gfx = state.window.add_obj(
        Path::new(&format!("{}/{}", state.assets_path, prim.path_obj)),
        Path::new(&format!("{}/{}", state.assets_path, prim.path_mtl)),
        prim_scale,
    );

    state.physics_entities.push(PhysicsEntity{
        collider : collision_handle,
        node : gfx,
    });
}

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

    let mut state = AppState {
        window,
        assets_path : String::from("/home/gareth/source/rust/olc-jam-2020/assets/"),
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

    let primitives = load_primitives_definitions(&state.assets_path).unwrap();

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

    for x in -20..20 {
        for z in -20..20 {
            add_primitive(&mut state, &primitives, "domino", &Vector3::new(x as f32, 0.01, z as f32));
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
    let mut planar_camera = FixedView::new();
    
    let mut cursor_ray = Ray::new(na::Point3::new(0.0,0.0,0.0), na::Vector3::new(0.0,0.0,0.0));
    let mut cursor_position = na::Point2::<f32>::new(0.0,0.0);
    while !state.window.should_close() {
        for event in state.window.events().iter() {
            match event.value {
                WindowEvent::Key(k, Action::Press, _) => {
                    if k == Key::Space {
                        add_primitive(&mut state, &primitives, "cubey", &Vector3::new(0.0, 50.0, 0.0));
                    }
                },
                WindowEvent::MouseButton(k, Action::Press, modif) => {
                    if modif.contains(Modifiers::Control) {
                        if k == MouseButton::Button2 {
                            // Find the intersection between cursor ray and ground, then spawn something
                            let toi = ground_collision_cuboid.toi_with_ray(&na::Isometry3::identity(), &cursor_ray, 10000.0, true).unwrap();
                            if toi > 0.0 {
                                let intersection_point = cursor_ray.origin + cursor_ray.dir * toi;
                                add_primitive(&mut state, &primitives, "cubey", &Vector3::new(
                                    intersection_point.x,
                                    intersection_point.y + 20.0,
                                    intersection_point.z,
                                ));
                            }
                        }
                    }
                },
                WindowEvent::CursorPos(x, y, _modif) => {
                    let window_size = na::Vector2::new(state.window.size()[0] as f32, state.window.size()[1] as f32);
                    let cursor_position_projected = na::Point2::new(x as f32, y as f32);

                    // (position, direction)
                    let ray = camera.unproject(&cursor_position_projected, &window_size);
                    cursor_ray = Ray::new(ray.0, ray.1);
                    
                    // TODO: HAX! - abusing the planar camera to work out the cursor coords
                    let unprojected = planar_camera.unproject(&cursor_position_projected, &window_size);
                    cursor_position = unprojected;
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

        // Draw HUD elements
        draw_planar_cursor(&mut state, &cursor_position);
        // draw_hud_text(&mut state, 
        //     &format!("Cursor(World): {},{},{}", cursor_position_world.x, cursor_position_world.y, cursor_position_world.z),
        //     &Point2::new(0.0,0.0), 50.0);

        state.window.render_with_camera(&mut camera); 
    }
}

fn draw_planar_cursor( state: &mut AppState, cursor_position : &na::Point2<f32> ) {
    const CROSS_SIZE: f32 = 10.0;
    let cursor_colour = na::Point3::new(1.0, 0.5, 1.0);
    let up = na::Vector2::new(CROSS_SIZE, 0.0);

    // TODO: HAX!
    // draw_planar line seems to take pixel offsets from screen center
    // cursor_position is hacked from the planar_camera (despite not using it), so
    // here we transform from -1.0-1.0 -> what draw_planar_line needs
    let cursor_planar_pos = na::Point2::new(
        cursor_position.x * (state.window.size()[0] / 2) as f32,
        cursor_position.y * (state.window.size()[1] / 2) as f32,
    );

    // let cursor_position = na::Point2::new(100.0, 100.0);
    // println!("Draw cursor: {:?}", cursor_position);
    state.window.draw_planar_line(&(cursor_planar_pos - up), &(cursor_planar_pos + up), &cursor_colour);

    let right = na::Vector2::new(0.0, CROSS_SIZE);
    state.window.draw_planar_line(&(cursor_planar_pos - right), &(cursor_planar_pos + right), &cursor_colour);
}

fn draw_hud_text( state: &mut AppState, text : &str, position : &na::Point2<f32>, size: f32 ) {
    let font = Font::default();
    // Text coordinates are in pixels, from top-left
    state.window.draw_text(
        &text,
        &position,
        size,
        &font,
        &Point3::new(1.0, 1.0, 1.0),
    );
}