
use three_d::{Gl, VertexBuffer, ElementBuffer,vec2,Vec3,vec3,vec4,Mat4,SquareMatrix,CPUMesh};

use crate::datatypes::{Renderable,RenderableMesh,RenderMethod};

pub fn load_gltf( gl: &Gl, gltf_data: &[u8] ) -> Result<Renderable, String> {
    let (gltf, buffers, _images) = gltf::import_slice(gltf_data).unwrap(); // TODO

    let mut results = Renderable {
        children : Vec::new(),
        meshes   : Vec::new(),
        transform : Mat4::identity(),
    };

    for scene in gltf.scenes() {
        for node in scene.nodes() {
            parse_gltf_node( gl, &buffers, &node, &mut results.children );
        }
    }

    Ok(results)
}

fn parse_gltf_node( gl: &Gl, buffers : &Vec<gltf::buffer::Data>, node : &gltf::Node, results : &mut Vec<Renderable> ) {
    let mut meshes : Vec<RenderableMesh> = Vec::new();
    if let Some(gltf_mesh) = node.mesh() {
        for primitive in gltf_mesh.primitives() {
            let name = gltf_mesh.name().unwrap_or("");
            let mut verts : Vec<Vec3> = Vec::new();
            let mut norms : Vec<Vec3> = Vec::new();
            let mut indices : Vec<u32> = Vec::new();
            let mut draw_count = 0;
            let mut draw_method = RenderMethod::DrawArrays;

            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if let Some(iter) = reader.read_positions() {
                for vertex_position in iter {
                    let v = vec3(vertex_position[0],vertex_position[1],vertex_position[2]);
                    verts.push(v);
                    draw_count += 1;

                }
            }
            if let Some(iter) = reader.read_normals() {
                for normal in iter {
                    let n = vec3(normal[0], normal[1], normal[2]);
                    norms.push(n);
                }
            }

            let mut gltf_indices = reader.read_indices()
                .map(|read_indices| {
                    read_indices.into_u32().collect::<Vec<_>>()
                })
                .unwrap_or(Vec::new());
            
            if !gltf_indices.is_empty() {
                draw_count = indices.len() as u32;
                draw_method = RenderMethod::DrawElements;
                indices.append(&mut gltf_indices);
            }

            let gl_material = primitive.material();
            let bc = gl_material.pbr_metallic_roughness().base_color_factor();
            let em = gl_material.emissive_factor();
            let mr = vec2(
                gl_material.pbr_metallic_roughness().metallic_factor(),
                gl_material.pbr_metallic_roughness().roughness_factor(),
            );


            // TODO: This is nasty, must be a language way of doing this
            let mut verts_flat = Vec::new();
            for vert in &verts {
                verts_flat.push(vert.x);
                verts_flat.push(vert.y);
                verts_flat.push(vert.z);
            }
            let mut norms_flat = Vec::new();
            for n in &norms {
                norms_flat.push(n.x);
                norms_flat.push(n.y);
                norms_flat.push(n.z);
            }

            // let vertex_buffer = VertexBuffer::new_with_static_f32(&gl, &verts_flat).unwrap();
            // let normal_buffer = VertexBuffer::new_with_static_f32(&gl, &norms_flat).unwrap();
            // let element_buffer = ElementBuffer::new_with_u32(&gl, &indices).unwrap();
    
            let bmin = primitive.bounding_box().min;
            let bmax = primitive.bounding_box().max;
            println!("gltf min: {:?}", bmin);
            println!("gltf max: {:?}", bmax);

            let mut mesh = CPUMesh::new(&indices, &verts_flat, &norms_flat).unwrap().to_mesh(&gl).unwrap();

        // TODO: Total hacks here
        mesh.color = vec3(bc[0], bc[1], bc[2]);
        mesh.diffuse_intensity = 1.0;
        mesh.specular_intensity = 1.0 - mr.y;
        mesh.specular_power = 32.0;


            meshes.push( RenderableMesh{
                // vertices: verts,
                // vertex_buffer,
                // normals: norms,
                // normal_buffer,
                // elements: indices,
                // element_buffer,
                name : String::from(name),
                // draw_count,
                // draw_method,
                // material,
                bounds_min : vec3(bmin[0], bmin[1], bmin[2]),
                bounds_max : vec3(bmax[0], bmax[1], bmax[2]),
                mesh,
            });
        }
    }

    let mut children : Vec<Renderable> = Vec::new();
    for child in node.children() {
        parse_gltf_node(gl, buffers, &child, &mut children);
    }

    let gt = node.transform().matrix();
    // TODO: This is either perfect, or perfectly wrong, double check the order
    let matrix = Mat4::new(
        gt[0][0], gt[0][1], gt[0][2], gt[0][3],
        gt[1][0], gt[1][1], gt[1][2], gt[1][3],
        gt[2][0], gt[2][1], gt[2][2], gt[2][3],
        gt[3][0], gt[3][1], gt[3][2], gt[3][3]
    );

    results.push( Renderable {
        meshes,
        children,
        transform : matrix,
    } );
}