
use text_io::read;

extern crate goldberg;
use goldberg::types::*;

fn main() {
    let mut prim = PrimitiveDefinition {
        name : String::from(""),
        path_obj : String::from(""),
        path_mtl : String::from(""),
        scale : [1.0;3],
        density : 1000.0,
        restitution : 0.0,
        friction : 0.2,
        collider_def : Vec::new(),
    };

    println!("Defining a primitive type: ");
    println!("Primitive (file) name: ");
    prim.name = read!("{}\n");
    println!("Path to .obj file, relative to assets dir: ");
    prim.path_obj = read!("{}\n");
    println!("Folder containing .mtl files, relative to assets dir: ");
    prim.path_mtl = read!("{}\n");

    println!("\n");
    println!("Density (g/m^3): ");
    prim.density = read!("{}\n");
    println!("Friction Coefficient: ");
    prim.friction = read!("{}\n");

    println!("\n");
    println!("Primitive x scale: ");
    prim.scale[0] = read!("{}\n");
    println!("Primitive y scale: ");
    prim.scale[1] = read!("{}\n");
    println!("Primitive z scale: ");
    prim.scale[2] = read!("{}\n");

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

        println!("Add another collider (y/n)?");
        let choice : String = read!("{}\n");
        if choice == "y" {
            continue;
        } else {
            break;
        }
    }

    println!("Nice, here's your data:");
    
    let json_str = serde_json::to_string_pretty(&prim).unwrap();
    println!("\n\n{}\n\n", json_str);
}
