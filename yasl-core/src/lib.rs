use syn::parse::{Parse, ParseStream, Result};

mod glsl;
use glsl::{Glsl, GlslLine};

mod yasl_block;
mod yasl_expr;
mod yasl_file;
mod yasl_ident;
mod yasl_item;
mod yasl_stmt;
mod yasl_type;

use yasl_file::YaslFile;

pub struct Shader {
    pub glsl: String,
    pub sourcemap: Vec<GlslLine>,
}

impl Parse for Shader {
    fn parse(input: ParseStream) -> Result<Self> {
        let version = "#version 450\n";
        let mut out = String::new();
        out += version;

        let file = YaslFile::parse(input)?;

        let glsl: Glsl = (&file).into();
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

        Ok(Self {
            glsl: out,
            sourcemap,
        })
    }
}
