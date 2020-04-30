use std::error::Error;
use std::fs::File;
use std::io::Read;

use yasl_core::Shader;

use spirv_cross::{hlsl, spirv, ErrorCode};

pub fn words_from_bytes(buf: &[u8]) -> &[u32] {
    unsafe {
        std::slice::from_raw_parts(
            buf.as_ptr() as *const u32,
            buf.len() / std::mem::size_of::<u32>(),
        )
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./test/main_vert.yasl")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let shader = syn::parse_str::<Shader>(&content)?;

    println!("{:#?}", shader.glsl);

    let sprv = {
        let mut compiler = shaderc::Compiler::new().unwrap();

        compiler
            .compile_into_spirv(
                &shader.glsl,
                shaderc::ShaderKind::Vertex,
                "shader.glsl",
                "main",
                None,
            )
            .unwrap()
            .as_binary_u8()
            .to_vec()
    };

    let sprv = words_from_bytes(&sprv);
    let module: spirv::Module = spirv::Module::from_words(sprv);
    let mut ast = spirv::Ast::<hlsl::Target>::parse(&module).unwrap();
    println!("{}", ast.compile().unwrap());

    Ok(())
}

fn main() {
    run().unwrap()
}
