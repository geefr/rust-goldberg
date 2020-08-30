
extern crate kiss3d;
extern crate nalgebra as na;

use crate::engine::*;

use na::{Point2, Vector3};

use ncollide3d::shape::{Cuboid};
use ncollide3d::query::{Ray, RayCast};

use kiss3d::event::{Key, MouseButton, Modifiers};
use kiss3d::camera::{Camera};
use kiss3d::planar_camera::*;

pub trait Interaction {
    fn on_key_down( &mut self, state : &mut AppState, k : &Key, modif : &Modifiers );
    fn on_key_up( &mut self, state : &mut AppState, k : &Key, modif : &Modifiers );
    fn on_mouse_down( &mut self, state : &mut AppState, k : &MouseButton, modif : &Modifiers );
    fn on_mouse_up( &mut self, state : &mut AppState, k : &MouseButton, modif : &Modifiers );
    fn on_mouse_move( &mut self, state : &mut AppState, x : f32, y : f32, modif : &Modifiers );
    fn render( &mut self, state : &mut AppState );
}

#[derive(Debug)]
enum EditorPlacementMode {
    Singular,
    Instanced,
}
pub struct EditorModeInteraction {
    pub ground_collision_cuboid : Cuboid<f32>,
    pub primitive_name : String,
    cursor_ray : Ray<f32>,
    cursor_position : Point2<f32>,
    primitive_rotation : na::Vector3<f32>,
    primitive_rotation_delta : f32,
    primitive_spawn_height : f32,
    primitive_spawn_spacing : f32,
    primitve_last_spawn_pos : na::Vector3<f32>,
    primitive_auto_rotate : bool,
    primitive_placement_mode : EditorPlacementMode,
    mouse_button1_pressed : bool,
}
impl EditorModeInteraction {
    pub fn new(ground_collision_cuboid : Cuboid<f32>, primitive_name : &str) -> Self {
        EditorModeInteraction
        {
            ground_collision_cuboid,
            cursor_ray : Ray::new(na::Point3::new(0.0,0.0,0.0), na::Vector3::new(0.0,0.0,0.0)),
            cursor_position : na::Point2::<f32>::new(0.0,0.0),
            primitive_name : String::from(primitive_name),
            primitive_rotation : na::Vector3::new(0.0,0.0,0.0),
            primitive_rotation_delta : 15.0_f64.to_radians() as f32,
            primitive_spawn_height : 0.5,
            primitive_spawn_spacing : 1.2,
            primitve_last_spawn_pos : na::Vector3::new(0.0,0.0,0.0),
            primitive_auto_rotate : true,
            primitive_placement_mode : EditorPlacementMode::Singular,
            mouse_button1_pressed : false,
        }
    }
}

impl Interaction for EditorModeInteraction {
    fn on_key_down( &mut self, state : &mut AppState, k : &Key, modif : &Modifiers ) {
        match *k {
            Key::Tab => {
                // Advance to the next primitive
                let mut it = state.primitives_library.iter();
                while let Some((key, _value)) = it.next() {
                    if *key == self.primitive_name {
                        if let Some((next_key, _next_val)) = it.next() {
                            self.primitive_name = next_key.clone();
                            break;
                        } else {
                            // Loop to beginning of primitive library
                            if let Some((first_key, _)) = state.primitives_library.iter().next() {
                                self.primitive_name = first_key.clone();
                            }
                        }
                    }
                }
            },
            Key::Q => self.primitive_rotation = Vector3::new(0.0,0.0,0.0),
            Key::E => {
                if self.primitive_spawn_height > 1.0 {
                    self.primitive_spawn_height = 0.01;
                } else {
                    self.primitive_spawn_height = 30.0;
                }
            },
            Key::A => self.primitive_rotation.y += self.primitive_rotation_delta,
            Key::D => self.primitive_rotation.y -= self.primitive_rotation_delta,
            Key::W => self.primitive_rotation.x -= self.primitive_rotation_delta,
            Key::S => {
                if modif.contains(Modifiers::Control) {
                    save_level_definition(&state.level_definition, &state.level_file);
                } else {
                    self.primitive_rotation.x += self.primitive_rotation_delta;
                }
            },
            Key::R => self.primitive_spawn_spacing *= 1.1,
            Key::F => self.primitive_spawn_spacing *= 0.9,
            Key::C => self.primitive_auto_rotate = !self.primitive_auto_rotate,
            Key::X => {
                match self.primitive_placement_mode {
                    EditorPlacementMode::Instanced => self.primitive_placement_mode = EditorPlacementMode::Singular,
                    EditorPlacementMode::Singular => self.primitive_placement_mode = EditorPlacementMode::Instanced,
                };
            },
            _ => {}
        }
    }
    fn on_key_up( &mut self, _state : &mut AppState, _k : &Key, _modif : &Modifiers ) {

    }
    fn on_mouse_down( &mut self, state : &mut AppState, k : &MouseButton, _modif : &Modifiers ) {
        if *k == MouseButton::Button1 {
            self.mouse_button1_pressed = true;
        }

        // TODO: This is a copy of code below, de-dupe it
        // Find the intersection between cursor ray and ground, then spawn something
        if let Some(toi) = self.ground_collision_cuboid.toi_with_ray(&na::Isometry3::identity(), &self.cursor_ray, 10000.0, true) {
            if toi > 0.0 {
                let intersection_point = self.cursor_ray.origin + self.cursor_ray.dir * toi;
                let intersection_point = na::Vector3::new(
                    intersection_point.x,
                    intersection_point.y + self.primitive_spawn_height,
                    intersection_point.z,
                );

                if self.primitive_auto_rotate {
                    // Calculate the angle from the last spawn point, auto-update the Y rotation to match
                    let last_to_current = (intersection_point - self.primitve_last_spawn_pos).normalize();
                    // TODO: This is tied to the ingest format now, should change at some point
                    // .objs imported must have -Ve Z axis as forward
                    let zero_reference = Vector3::new(0.0,0.0,-1.0);

                    let mut angle = zero_reference.dot(&last_to_current).acos();
                    let cross = zero_reference.cross(&last_to_current);
                    if Vector3::new(0.0, 1.0, 0.0).dot(&cross) < 0.0  {
                        angle *= -1.0;
                    }
                    self.primitive_rotation.y = angle;
                }

                match self.primitive_placement_mode {
                    EditorPlacementMode::Singular => {
                        if self.mouse_button1_pressed{
                            self.primitve_last_spawn_pos = intersection_point;
                            state.add_primitive(&self.primitive_name, &self.primitve_last_spawn_pos, &self.primitive_rotation);
                            self.mouse_button1_pressed = false;
                        }
                    },
                    _ => {}
                }
            }
        }
    }
    fn on_mouse_up( &mut self, _state : &mut AppState, k : &MouseButton, _modif : &Modifiers ) {
        if *k == MouseButton::Button1 {
            self.mouse_button1_pressed = false;
        }
    }
    fn on_mouse_move( &mut self, state : &mut AppState, x : f32, y : f32, _modif : &Modifiers ) {
        let window_size = na::Vector2::new(state.window.size()[0] as f32, state.window.size()[1] as f32);
        let cursor_position_projected = na::Point2::new(x as f32, y as f32);
    
        // (position, direction)
        let ray = state.camera.unproject(&cursor_position_projected, &window_size);
        self.cursor_ray = Ray::new(ray.0, ray.1);
        
        // TODO: HAX! - abusing the planar camera to work out the cursor coords
        let unprojected = state.planar_camera.unproject(&cursor_position_projected, &window_size);
        self.cursor_position = unprojected;
        
        // Find the intersection between cursor ray and ground, then spawn something
        if let Some(toi) = self.ground_collision_cuboid.toi_with_ray(&na::Isometry3::identity(), &self.cursor_ray, 10000.0, true) {
            if toi > 0.0 {
                let intersection_point = self.cursor_ray.origin + self.cursor_ray.dir * toi;
                let intersection_point = na::Vector3::new(
                    intersection_point.x,
                    intersection_point.y + self.primitive_spawn_height,
                    intersection_point.z,
                );

                if self.primitive_auto_rotate {
                    // Calculate the angle from the last spawn point, auto-update the Y rotation to match
                    let last_to_current = (intersection_point - self.primitve_last_spawn_pos).normalize();
                    // TODO: This is tied to the ingest format now, should change at some point
                    // .objs imported must have -Ve Z axis as forward
                    let zero_reference = Vector3::new(0.0,0.0,-1.0);

                    let mut angle = zero_reference.dot(&last_to_current).acos();
                    let cross = zero_reference.cross(&last_to_current);
                    if Vector3::new(0.0, 1.0, 0.0).dot(&cross) < 0.0  {
                        angle *= -1.0;
                    }
                    self.primitive_rotation.y = angle;
                }

                match self.primitive_placement_mode {
                    EditorPlacementMode::Instanced => {
                        if (intersection_point - self.primitve_last_spawn_pos).magnitude() > self.primitive_spawn_spacing {
                            self.primitve_last_spawn_pos = intersection_point;
                            if self.mouse_button1_pressed{
                                state.add_primitive(&self.primitive_name, &self.primitve_last_spawn_pos, &self.primitive_rotation);
                            }
                        }
                    },
                    _ => {}
                }
                
            }
        }
    }

    fn render( &mut self, state : &mut AppState ) {
        let control_text = format!(
"Controls:
    Right Mouse: Rotate Camera
    Middle Mouse: Translate Camera
    Left drag: Spawn object: {}
    A/D: Rotate Primitive Y
    W/S: Rotate Primitive X
    Q  : Reset Primitive Rotation
    E  : Toggle spawn height
    R  : Increase spacing
    F  : Decrease spacing
    C  : Toggle auto-rotate
    X  : Cycle placement mode
    Ctrl+S : Save level ({})

Number of Primitives   : {},
Auto-Rotate Active     : {},
Placement Mode         : {:?},
Primitive Rotation     : {}°, {}°, {}°
Primitive Spawn Height : {}
Primitive Spacing      : {}
Primitive auto-rotate  : {}",
        self.primitive_name,
        state.level_definition.name,
        state.physics_entities.len(),
        self.primitive_auto_rotate,
        self.primitive_placement_mode,
        self.primitive_rotation.x.to_degrees(), self.primitive_rotation.y.to_degrees(), self.primitive_rotation.z.to_degrees(),
        self.primitive_spawn_height,
        self.primitive_spawn_spacing,
        self.primitive_auto_rotate
        );
        state.draw_hud_text(
            &control_text,
            &Point2::new(0.0,0.0), 30.0);

        const CROSS_SIZE: f32 = 10.0;
        let cursor_colour = na::Point3::new(1.0, 0.5, 1.0);
        let up = na::Vector2::new(CROSS_SIZE, 0.0);

        // TODO: HAX!
        // draw_planar line seems to take pixel offsets from screen center
        // cursor_position is hacked from the planar_camera (despite not using it), so
        // here we transform from -1.0-1.0 -> what draw_planar_line needs
        let cursor_planar_pos = na::Point2::new(
            self.cursor_position.x * (state.window.size()[0] / 2) as f32,
            self.cursor_position.y * (state.window.size()[1] / 2) as f32,
        );

        state.window.draw_planar_line(&(cursor_planar_pos - up), &(cursor_planar_pos + up), &cursor_colour);

        let right = na::Vector2::new(0.0, CROSS_SIZE);
        state.window.draw_planar_line(&(cursor_planar_pos - right), &(cursor_planar_pos + right), &cursor_colour);
    }
}
