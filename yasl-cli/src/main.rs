use std::error::Error;
use std::fs::File;
use std::io::Read;

use yasl_core::Shader;

fn run() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./test/vert.yasl")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let shader = syn::parse_str::<Shader>(&content)?;

    println!("{:#?}", shader.glsl);

    Ok(())
}

fn main() {
    run().unwrap()
}
