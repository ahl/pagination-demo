use std::{
    env,
    fs::{self, File},
    path::Path,
};

use progenitor::Generator;

fn main() {
    let src = "../api.json";
    println!("cargo:rerun-if-changed={}", src);
    let file = File::open(src).unwrap();
    let spec = serde_json::from_reader(file).unwrap();
    let mut generator = Generator::new();

    let content = generator.generate_text(&spec).unwrap();

    let mut out_file = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push("codegen.rs");

    fs::write(out_file, content).unwrap();
}
