use syn::parse::{Parse, ParseStream, Result};
use syn::parse_macro_input;
use syn::Error;

use yasl_core::Shader;

struct Compiler {
    sprv: Vec<u8>,
}
impl Parse for Compiler {
    fn parse(input: ParseStream) -> Result<Self> {
        let shader: Shader = input.parse()?;

        #[cfg(feature = "use-shaderc")]
        let sprv = {
            let mut compiler = shaderc::Compiler::new().unwrap();

            compiler.compile_into_spirv(
                &shader.glsl,
                shaderc::ShaderKind::Vertex,
                "shader.glsl",
                "main",
                None,
            )
        };

        #[cfg(feature = "use-glsl-to-spirv")]
        let mut sprv = glsl_to_spirv::compile(&shader.glsl, glsl_to_spirv::ShaderType::Vertex);

        match sprv {
            Err(e) => {
                #[cfg(feature = "use-shaderc")]
                {
                    if let shaderc::Error::CompilationError(_n, e) = e {
                        let lines: Vec<&str> = e.split("\n").filter(|l| l.len() > 0).collect();
                        for l in lines {
                            let split: Vec<&str> = l.split(":").collect();
                            if let Some(n) = split.get(1) {
                                if let Ok(n) = n.parse::<usize>() {
                                    if let Some(s) = shader.sourcemap[n - 2].span {
                                        return Err(Error::new(s.into(), l));
                                    }
                                }
                            }
                        }

                        panic!("{:?}", e);
                    } else {
                        panic!("{:?}", e);
                    }
                }

                #[cfg(feature = "use-glsl-to-spirv")]
                panic!("{:?}", e);
                // let split: Vec<&str> = r.split("\n").collect();

                // let mut lines = split.iter();

                // let _file = lines.next();

                // println!("==GLSL VALIDATOR==");
                // for l in lines.filter(|l| l.len() > 0) {
                //     if l.starts_with("ERROR:") {
                //         let split: Vec<&str> = l.split(":").collect();
                //         if split.len() >= 3 {
                //             if split[1].starts_with(" /tmp/") {
                //                 let err = l;
                //                 // panic!("{:?}", err);
                //                 // println!("{:?}", split[2]);
                //             }
                //         }
                //     }
                // }
            }
            #[cfg(feature = "use-shaderc")]
            Ok(sprv) => {
                    Ok(Self {
                        sprv: sprv.as_binary_u8().to_vec(),
                    })
            }
            #[cfg(feature = "use-glsl-to-spirv")]
            Ok(mut sprv) => {
                    use std::io::prelude::*;

                    let mut buffer = Vec::new();
                    sprv.read_to_end(&mut buffer).unwrap();

                    Ok(Self { sprv: buffer })
            }
        }

    }
}

use proc_macro::TokenStream;
#[proc_macro]
pub fn yasl_vert(input: TokenStream) -> TokenStream {
    let shader = parse_macro_input!(input as Compiler);

    format!("const a: [u8;{}] = {:?};", shader.sprv.len(), shader.sprv)
        .parse()
        .unwrap()
}
