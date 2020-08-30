
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
        }
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

    // TODO: Collider type (We only have one, so no choice)

    println!("Collider origin x");
    prim.collider_def.origin[0] = read!("{}\n");
    println!("Collider origin y");
    prim.collider_def.origin[1] = read!("{}\n");
    println!("Collider origin z");
    prim.collider_def.origin[2] = read!("{}\n");

    println!("Collider scale x");
    prim.collider_def.dimensions[0] = read!("{}\n");
    println!("Collider scale y");
    prim.collider_def.dimensions[1] = read!("{}\n");
    println!("Collider scale z");
    prim.collider_def.dimensions[2] = read!("{}\n");


    println!("Nice, here's your data:");
    
    let json_str = serde_json::to_string_pretty(&prim).unwrap();
    println!("\n\n{}\n\n", json_str);
}
