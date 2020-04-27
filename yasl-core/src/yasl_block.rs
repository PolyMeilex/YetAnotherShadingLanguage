use crate::convert::{AsGlsl, Glsl, GlslFragment};
use std::convert::{TryFrom, TryInto};
use syn::{Block, Error, Result};

use crate::yasl_stmt::YaslStmt;

#[derive(Debug)]
pub struct YaslBlock {
    stmts: Vec<YaslStmt>,
}

impl AsGlsl for YaslBlock {
    fn as_glsl(&self) -> Glsl {
        let mut elements = Vec::new();

        // elements.push(Glsl::Line(GlslLine {
        //     span: None,
        //     ends_with_semi: false,
        //     glsl_string: "{".into(),
        // }));

        for s in self.stmts.iter() {
            elements.push(s.as_glsl());
        }

        // elements.push(Glsl::Line(GlslLine {
        //     span: None,
        //     ends_with_semi: false,
        //     glsl_string: "}".into(),
        // }));

        Glsl::Fragment(GlslFragment { elements })
    }
}
impl TryFrom<Block> for YaslBlock {
    type Error = Error;
    fn try_from(block: Block) -> Result<Self> {
        let mut stmts = Vec::new();

        for s in block.stmts.into_iter() {
            stmts.push(s.try_into()?);
        }

        Ok(Self { stmts })
    }
}
