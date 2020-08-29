
use three_d::*;
use rand::Rng;

extern crate nalgebra as na;
use nphysics3d::object::{DefaultBodySet, DefaultColliderSet, BodyStatus};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use nphysics3d::math::Isometry;

mod gltfloader;
use gltfloader::*;

mod physicsbuilders;
use physicsbuilders::*;

mod datatypes;
use datatypes::{Renderable,RenderMethod,EntityRigidBody};

fn render_entities( gl : &Gl, cam : &Camera, eye : &Vec3, time : f32, entities : &Vec<EntityRigidBody>, model_matrix : &Mat4 ) {
    //println!("render_entities:\n\n");
    for entity in entities {
        // TODO: Should store shader progs etc on the entities
        let entity_matrix = model_matrix * entity.active_transform;
        render_renderable( gl, cam, eye, time, &entity.renderable, &entity_matrix );
    }
}

fn render_renderables( gl : &Gl, cam : &Camera, eye : &Vec3, time : f32, renderables : &Vec<Renderable>, model_matrix : &Mat4 ) {
    for renderable in renderables {
        render_renderable(gl, cam, eye, time, renderable, model_matrix);
    }
}

fn render_renderable( gl : &Gl, cam : &Camera, eye : &Vec3, time : f32, renderable : &Renderable, model_matrix : &Mat4 ) {
    let node_matrix = model_matrix * renderable.transform;

    for mesh in &renderable.meshes {
        
        mesh.mesh.render(&node_matrix, cam);
    }

    render_renderables(&gl, &cam, eye, time, &renderable.children, &node_matrix);
}

fn update_entities(entities_rigid : &mut Vec<EntityRigidBody>, phys_colliders : &DefaultColliderSet<f32>) {
    for entity in entities_rigid {
        if !entity.phys_collider.is_some() {
            continue;
        }
        
        if let Some(collider) = phys_colliders.get(entity.phys_collider.unwrap()) {
            // https://github.com/dimforge/nphysics/blob/master/src_testbed/objects/node.rs
            let tx = na::convert::<Isometry<f64>, Isometry<f32>>( na::convert_unchecked(*collider.position()) );

            let collider_translation = tx.translation;
            let collider_rotation = tx.rotation;

            let rotation = Mat4::from(
            cgmath::Quaternion::new(
                collider_rotation[0],
                collider_rotation[1],
                collider_rotation[2],
                collider_rotation[3],
            ));
            
            let translation = vec3(
                collider_translation.x, collider_translation.y, collider_translation.z,
            );

            entity.active_transform = Mat4::from_translation(translation) * Mat4::from_nonuniform_scale(entity.scale.x, entity.scale.y, entity.scale.z) * rotation;
        }
    }
}

fn main() {
    let mut window = Window::new("OLC Code Jam 2020", 1920, 1080).unwrap();
    let (mut screen_width, mut screen_height) = window.framebuffer_size();
    
    let gl = window.gl();

    // Camera
    let fov = 75.0;
    let near = 1.0;
    let far = 1000.0;
    let eye = vec3(5.0, 5.0, 5.0);
    let target = vec3(0.0,0.0,0.0);
    let up = vec3(0.0,1.0,0.0);
    let mut cam = Camera::new_perspective(&gl, eye, target, up,
        degrees(fov), screen_width as f32 / screen_height as f32, near, far);
    let mut renderer = DeferredPipeline::new(&gl).unwrap();

    let mut entities_rigid : Vec<EntityRigidBody> = Vec::new();

    // Create entities in the world
    // TODO: Some level format/definition loading would go here, maybe just serde it + lookup meshes/etc in a table?
    entities_rigid.push( EntityRigidBody{
        renderable : load_gltf(&gl, include_bytes!("../assets/models/world_base/world_base.glb")).unwrap(),
        scale : vec3(1.0, 1.0, 1.0),
        active_transform  : Mat4::identity(),
        phys_body_status: BodyStatus::Static,
        collider_origin : vec3(0.0,0.0,0.0),
        collider_dimensions : vec3(100.0, 1.0, 100.0),
        phys_body: None,
        phys_collider: None,
    });
    let mut rng = rand::thread_rng();

    // Load the dominos
    let domino_gltf = include_bytes!("../assets/models/primitives/Domino.glb");

    // for z in -20..100 {
        let z = 0;
        let z_offset = z as f32 * 2.0;
        entities_rigid.push( EntityRigidBody{
            renderable : load_gltf(&gl, domino_gltf).unwrap(),
            scale : vec3(1.0, 1.0, 1.0),
            active_transform  : Mat4::from_translation(vec3(0.0, 2.0, z_offset)),
            phys_body_status: BodyStatus::Dynamic,
            collider_origin : vec3(0.0,-1.0,0.0),
            collider_dimensions : vec3(0.1, 0.5, 0.2),
            phys_body: None,
            phys_collider: None,
        });
    // }
    
    // for _i in 0..200 {
    //     let domino = load_gltf(&gl, domino_gltf).unwrap();
    //     let x_offset: f32 = rng.gen_range(-10.0, 10.0);
    //     let y_offset: f32 = rng.gen_range(100.0, 1000.0);
    //     let z_offset: f32 = rng.gen_range(-10.0, 10.0);

    //     // let x_scale: f32 = rng.gen_range(0.1, 2.5);
    //     // let y_scale: f32 = rng.gen_range(0.1, 2.5);
    //     // let z_scale: f32 = rng.gen_range(0.1, 2.5);
    //     let x_scale = 1.0;
    //     let y_scale = 1.0;
    //     let z_scale = 1.0;

    //     entities_rigid.push( EntityRigidBody{ 
    //         renderable: domino,
    //         scale : vec3(x_scale, y_scale, z_scale),
    //         active_transform : Mat4::from_translation(vec3(x_offset, y_offset, z_offset)),
    //         phys_body_status: BodyStatus::Dynamic,
    //         phys_body: None,
    //         phys_collider: None,
    //     } );
    // }
    
    // Light definitions
    let mut directional_light0 = DirectionalLight::new(&gl, 0.3, &vec3(0.5, 0.5, 0.5), &vec3(0.0, -1.0, 0.0)).unwrap();
    let mut directional_light1 = DirectionalLight::new(&gl, 0.3, &vec3(0.8, 0.8, 0.8), &vec3(0.0, -1.0, 0.0)).unwrap();
    let mut point_light0 = PointLight::new(&gl, 0.5, &vec3(0.8, 0.8, 0.8), &vec3(50.0, 50.0, 0.0), 0.5, 0.05, 0.005).unwrap();
    let mut point_light1 = PointLight::new(&gl, 0.5, &vec3(0.8, 0.8, 0.8), &vec3(-50.0, 50.0, 0.0), 0.5, 0.05, 0.005).unwrap();
    let mut spot_light = SpotLight::new(&gl, 0.8, &vec3(1.0, 1.0, 1.0), &vec3(80.0, 80.0, 20.0), &vec3(-8.0, -8.0, -2.0), 25.0, 0.1, 0.001, 0.0001).unwrap();

    // Physics setup
    // The world
    let mut phys_mech_world = DefaultMechanicalWorld::new(na::Vector3::new(0.0, -9.81, 0.0));
    let mut phys_geom_world = DefaultGeometricalWorld::new();
    // The bodies/parts in the world
    let mut phys_bodies = DefaultBodySet::new();
    let mut phys_colliders = DefaultColliderSet::new();
    let mut phys_joints = DefaultJointConstraintSet::new();
    let mut phys_forces = DefaultForceGeneratorSet::new();

    // Populate the physics model
    for mut entity in &mut entities_rigid {
        phys_build_rigid_body(&mut entity, &mut phys_bodies, &mut phys_colliders);
    }

    // Render loop
    let mut time = 0.0;
    let clear_colour = vec4(0.1,0.1,0.1,1.0);
    let enable_shadows = false;
    window.render_loop(move |frame_input| {
        screen_width = frame_input.screen_width;
        screen_height = frame_input.screen_height;
        let elapsed_time_ms = frame_input.elapsed_time as f32;
        time += elapsed_time_ms / 1000.0;

        // Physics update
        // TODO: Run physics update on separate thread (How does that affect wasm target?)
        phys_mech_world.set_timestep(elapsed_time_ms / 1000.0);
        phys_mech_world.step(
            &mut phys_geom_world,
            &mut phys_bodies,
            &mut phys_colliders,
            &mut phys_joints,
            &mut phys_forces,
        );

        update_entities(&mut entities_rigid, &phys_colliders);

        cam.set_size(screen_width as f32, screen_height as f32);
        cam.set_perspective_projection(degrees(fov), screen_width as f32 / screen_height as f32, near, far);

        let render_scene = |camera: &Camera| {
            render_entities(&gl, &cam, &eye, time, &entities_rigid, &Mat4::identity());
        };
        if enable_shadows {
            spot_light.clear_shadow_map();
            directional_light0.clear_shadow_map();
            directional_light1.clear_shadow_map();

            directional_light0.generate_shadow_map(&vec3(0.0, 0.0, 0.0), 4.0, 4.0, 20.0, 1024, 1024, &render_scene);
            directional_light1.generate_shadow_map(&vec3(0.0, 0.0, 0.0), 4.0, 4.0, 20.0, 1024, 1024, &render_scene);
            spot_light.generate_shadow_map(20.0, 1024, &render_scene);
        }

        renderer.geometry_pass(screen_width, screen_height, &|| {
            render_scene(&cam);
        }).unwrap();

        Screen::write(&gl, 0, 0, screen_width, screen_height, Some(&clear_colour), Some(1.0), &|| {
            renderer.light_pass(&cam, None, &[&directional_light0, &directional_light1],
                &[&spot_light], &[&point_light0, &point_light1]).unwrap();
        }).unwrap();

    }).unwrap();
}
