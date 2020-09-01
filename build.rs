use std::path::{Path,PathBuf};
use std::{env, fs};

// Okay so Cargo is a source-code only build system, and pays no regards to packaging
// build.rs will be run during (before?) the target build as part of cargo build

// Thanks https://github.com/rust-lang/cargo/issues/5305
const ASSETS_DIR: &str = "assets";
fn main() {
    let target_dir_path = format!("{}/../../../", env::var("OUT_DIR").unwrap());
	println!("cargo:warning= Copying assets to {}", target_dir_path);
	// println!("cargo:rerun-if-changed={}", format!("{}/{}",ASSETS_DIR, target_dir_path));
	
	// TODO: Should delete any existing dir first?
    copy(ASSETS_DIR, format!("{}/{}", target_dir_path, ASSETS_DIR )).unwrap();
}

// Thanks https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
pub fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

	// println!("Copying {} => {}", input_root, output_root.to_string_lossy());

    while let Some(working_path) = stack.pop() {
        // println!("cargo:warning=process: {:?}", &working_path);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            // println!("cargo:warning= mkdir: {:?}", dest);
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        // println!("cargo:warning=  copy: {:?} -> {:?}", &path, &dest_path);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        // println!("cargo:warning=failed: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(())
}