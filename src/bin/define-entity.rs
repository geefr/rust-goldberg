
use text_io::read;

extern crate goldberg;
use goldberg::types::*;

fn main() {
    let mut ent = EntityDefinition {
        name : String::from(""),
        entity_type : EntityType::Cannon,
        primitive : PrimitiveDefinition {
            name : String::from(""),
            path_obj : String::from(""),
            path_mtl : String::from(""),
            scale : [1.0;3],
            density : 1000.0,
            restitution : 0.0,
            friction : 0.2,
            collider_def : Vec::new(),
        },
        active_default : false,
        cannon_spawn_point : [0.0;3],
        cannon_spawn_force : [0.0;3],
        cannon_projectile_name : String::from(""),
        cannon_projectile_scale : [1.0;3],
        cannon_ammo : 0,
        cannon_fire_delay : 0.0,
    };

    println!("Defining an entity. First define the primitive for it:");
    println!("Primitive (file) name: ");
    ent.primitive.name = read!("{}\n");
    println!("Path to .obj file, relative to assets dir: ");
    ent.primitive.path_obj = read!("{}\n");
    println!("Folder containing .mtl files, relative to assets dir: ");
    ent.primitive.path_mtl = read!("{}\n");

    println!("\n");
    println!("Density (g/m^3): ");
    ent.primitive.density = read!("{}\n");
    println!("Friction Coefficient: ");
    ent.primitive.friction = read!("{}\n");

    println!("\n");
    println!("Primitive x scale: ");
    ent.primitive.scale[0] = read!("{}\n");
    println!("Primitive y scale: ");
    ent.primitive.scale[1] = read!("{}\n");
    println!("Primitive z scale: ");
    ent.primitive.scale[2] = read!("{}\n");

    loop {
        // TODO: This sucks, improve it
        println!("Select index of collider type [0 = Cuboid, 1 = Ball]");
        let collider_type_index : u32 = read!("{}\n");
        let t = match collider_type_index {
            0 => ColliderType::Cuboid,
            1 => ColliderType::Ball,
            _ => {
                println!("ERROR: You chose an invalid type index");
                return;
            }
        };

        let mut collider_def = ColliderDefinition {
            collider_type : t,
            origin : [0.0;3],
            dimensions : [0.0;3],
        };

        println!("\n");
        println!("Collider origin x");
        collider_def.origin[0] = read!("{}\n");
        println!("Collider origin y");
        collider_def.origin[1] = read!("{}\n");
        println!("Collider origin z");
        collider_def.origin[2] = read!("{}\n");

        println!("Collider scale x");
        collider_def.dimensions[0] = read!("{}\n");
        println!("Collider scale y");
        collider_def.dimensions[1] = read!("{}\n");
        println!("Collider scale z");
        collider_def.dimensions[2] = read!("{}\n");

        println!("Collider scale x");
        collider_def.dimensions[0] = read!("{}\n");
        println!("Collider scale y");
        collider_def.dimensions[1] = read!("{}\n");
        println!("Collider scale z");
        collider_def.dimensions[2] = read!("{}\n");

        ent.primitive.collider_def.push(collider_def);

        println!("Add another collider (y/n)?");
        let choice : String = read!("{}\n");
        if choice == "y" {
            continue;
        } else {
            break;
        }
    }

    println!("\n\n Now define the entity");
    // TODO: This sucks, improve it
    println!("Select index of entity type [0 = Cannon]");
    let entity_type_index : u32 = read!("{}\n");
    let t = match entity_type_index {
        0 => EntityType::Cannon,
        _ => {
            println!("ERROR: You chose an invalid type index");
            return;
        }
    };
    ent.entity_type = t;

    println!("Name: ");
    ent.name = read!("{}\n");

    println!("Active by default? ");
    ent.active_default = read!("{}\n");

    println!("Cannon: Spawn point x: ");
    ent.cannon_spawn_point[0] = read!("{}\n");
    println!("Cannon: Spawn point y: ");
    ent.cannon_spawn_point[1] = read!("{}\n");
    println!("Cannon: Spawn point z: ");
    ent.cannon_spawn_point[2] = read!("{}\n");
    
    println!("Cannon: Projectile name: ");
    ent.cannon_projectile_name = read!("{}\n");

    println!("Cannon: projectile scale x: ");
    ent.cannon_projectile_scale[0] = read!("{}\n");
    println!("Cannon: projectile scale y: ");
    ent.cannon_projectile_scale[1] = read!("{}\n");
    println!("Cannon: projectile scale z: ");
    ent.cannon_projectile_scale[2] = read!("{}\n");
    
    println!("Cannon: Ammo Amount: ");
    ent.cannon_ammo = read!("{}\n");

    println!("Cannon: Fire Delay: ");
    ent.cannon_fire_delay = read!("{}\n");

    println!("Nice, here's your data:");
    
    let json_str = serde_json::to_string_pretty(&ent).unwrap();
    println!("\n\n{}\n\n", json_str);
}
