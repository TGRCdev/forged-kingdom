use std::{fs, path::Path};

/// Generates a mod.rs that publicly exports every
/// *.rs file in the given directory
fn generate_mod_rs(dir: &str) {
    let path = Path::new("src").join(dir);
    let mut contents = "// GENERATED CODE - DO NOT EDIT!
// This file is automatically generated by a build script (build.rs).
// Any changes made to this file will be overwritten on the next build.

"
    .to_string();

    for child in fs::read_dir(&path).unwrap().filter_map(|child| {
        let child = child.unwrap();
        let path = child.path();

        if path.extension().unwrap() == "rs"
            && !&["mod", "_generated_mod"].contains(&path.file_stem().unwrap().to_str().unwrap())
        {
            return Some(child);
        }
        None
    }) {
        let file_type = child.file_type().unwrap();
        let path = child.path();

        let file_name = path.file_stem().unwrap().to_str().unwrap();
        if file_type.is_file() {
            contents.push_str(&format!("pub mod {file_name};\n"));
        } else if file_type.is_dir() && fs::exists(path.join("mod.rs")).unwrap() {
            contents.push_str(&format!("pub mod {file_name};\n"));
        }
    }

    let mod_path = path.join("_generated_mod.rs");
    println!("cargo:rerun-if-changed={}", mod_path.display());

    fs::write(&mod_path, contents).unwrap();
}

fn main() {
    let auto_export_paths = ["components", "plugins", "resources", "systems", "utils"];

    auto_export_paths.iter().for_each(|dir| {
        generate_mod_rs(dir);
    });

    println!("cargo::rerun-if-changed=build.rs");
}
