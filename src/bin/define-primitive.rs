
use text_io::read;

extern crate goldberg;
use goldberg::types::*;

fn main() {
    let mut prim = PrimitiveDefinition {
        name : String::from(""),
        path_obj : String::from(""),
        path_mtl : String::from(""),
        scale : [1.0;3],
        collider_type : ColliderType::Cuboid,
        collider_def  : ColliderDefinitionCuboid {
            origin : [0.0;3],
            dimensions : [1.0;3],
        },
        collider_def_composite_cuboid : Vec::new(),
    };

    println!("Defining a primitive type: ");
    println!("Primitive (file) name: ");
    prim.name = read!("{}\n");
    println!("Path to .obj file, relative to assets dir: ");
    prim.path_obj = read!("{}\n");
    println!("Folder containing .mtl files, relative to assets dir: ");
    prim.path_mtl = read!("{}\n");
    println!("Primitive x scale: ");
    prim.scale[0] = read!("{}\n");
    println!("Primitive y scale: ");
    prim.scale[1] = read!("{}\n");
    println!("Primitive z scale: ");
    prim.scale[2] = read!("{}\n");


    // TODO: This sucks, improve it
    println!("Select index of collider type [0 = Cuboid, 1 = CompositeCuboid]");
    let collider_type_index : u32 = read!("{}\n");
    prim.collider_type = match collider_type_index {
        0 => ColliderType::Cuboid,
        1 => ColliderType::CompositeCuboid,
        _ => {
            println!("ERROR: You chose an invalid type index");
            return;
        }
    };

    loop {
        let mut collider_def = ColliderDefinitionCuboid {
            origin : [0.0;3],
            dimensions : [0.0;3],
        };

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

        match prim.collider_type {
            ColliderType::Cuboid => {
                prim.collider_def = collider_def;
                break;
            },
            ColliderType::CompositeCuboid => {
                prim.collider_def_composite_cuboid.push(collider_def);
                println!("Add another collider (y/n)?");
                let choice : String = read!("{}\n");
                if choice == "y" {
                    continue;
                } else {
                    break;
                }
            }
        }
    }

    println!("Nice, here's your data:");
    
    let json_str = serde_json::to_string_pretty(&prim).unwrap();
    println!("\n\n{}\n\n", json_str);
}
