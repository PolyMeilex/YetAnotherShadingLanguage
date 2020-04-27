use proc_macro::TokenStream;

use syn::parse::{Parse, ParseStream, Result};
use syn::parse_macro_input;
use syn::{spanned::Spanned, Error};

mod convert;
use convert::{AsGlsl, Glsl};

mod yasl_block;
mod yasl_expr;
mod yasl_file;
mod yasl_ident;
mod yasl_item;
mod yasl_local;
mod yasl_stmt;
mod yasl_type;

use yasl_file::YaslFile;

struct Shader {
    sprv: Vec<u8>,
}

impl Parse for Shader {
    fn parse(input: ParseStream) -> Result<Self> {
        let version = "#version 450\n";
        let mut out = String::new();
        out += version;

        let file = YaslFile::parse(input)?;

        let glsl = file.as_glsl();
        let glsl = if let Glsl::Fragment(f) = glsl {
            f
        } else {
            panic!("Expected Fragment On TopLevel");
        };

        out += &glsl.to_string();

        out += "\n";
        out += "void main(){ yasl_main(); }";

        let sourcemap = glsl.squash();

        println!("{}", out);

        #[cfg(feature = "use-shaderc")]
        let sprv = {
            let mut compiler = shaderc::Compiler::new().unwrap();

            compiler.compile_into_spirv(
                &out,
                shaderc::ShaderKind::Vertex,
                "shader.glsl",
                "main",
                None,
            )
        };

        #[cfg(feature = "use-glsl-to-spirv")]
        let sprv = glsl_to_spirv::compile(&out, glsl_to_spirv::ShaderType::Vertex);

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
                                    if let Some(s) = sourcemap[n - 2].span {
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
            Ok(mut sprv) => {
                #[cfg(feature = "use-shaderc")]
                {
                    Ok(Self {
                        sprv: sprv.as_binary_u8().to_vec(),
                    })
                }

                #[cfg(feature = "use-glsl-to-spirv")]
                {
                    use std::io::prelude::*;

                    let mut buffer = Vec::new();
                    sprv.read_to_end(&mut buffer).unwrap();

                    Ok(Self { sprv: buffer })
                }
            }
        }
    }
}
#[proc_macro]
pub fn yasl_vert(input: TokenStream) -> TokenStream {
    let shader = parse_macro_input!(input as Shader);

    format!("const a: [u8;{}] = {:?};", shader.sprv.len(), shader.sprv)
        .parse()
        .unwrap()
}
