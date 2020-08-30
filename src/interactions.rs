
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

pub struct EditorModeInteraction {
    pub ground_collision_cuboid : Cuboid<f32>,
    pub cursor_ray : Ray<f32>,
    pub cursor_position : Point2<f32>,
    pub primitive_name : String,
}
impl Interaction for EditorModeInteraction {
    fn on_key_down( &mut self, state : &mut AppState, k : &Key, _modif : &Modifiers ) {
        if *k == Key::Tab {
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
        }
    }
    fn on_key_up( &mut self, _state : &mut AppState, _k : &Key, _modif : &Modifiers ) {
    }
    fn on_mouse_down( &mut self, state : &mut AppState, k : &MouseButton, modif : &Modifiers ) {
        if modif.contains(Modifiers::Shift) {
            if *k == MouseButton::Button2 {
                // Find the intersection between cursor ray and ground, then spawn something
                let toi = self.ground_collision_cuboid.toi_with_ray(&na::Isometry3::identity(), &self.cursor_ray, 10000.0, true).unwrap();
                if toi > 0.0 {
                    let intersection_point = self.cursor_ray.origin + self.cursor_ray.dir * toi;
                    state.add_primitive(&self.primitive_name, &Vector3::new(
                        intersection_point.x,
                        intersection_point.y + 0.01,
                        intersection_point.z,
                    ));
                }
            }
        }
    }
    fn on_mouse_up( &mut self, _state : &mut AppState, _k : &MouseButton, _modif : &Modifiers ) {

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
    }

    fn render( &mut self, state : &mut AppState ) {
        let control_text = format!(
        "Controls:
           Left Mouse: Rotate Camera
           Right Mouse: Translate Camera
           Shift + Right click: Spawn {}",
        self.primitive_name
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
