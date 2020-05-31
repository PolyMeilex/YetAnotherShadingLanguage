use crate::glsl::{Glsl, GlslFragment, GlslLine};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
};
use syn::{spanned::Spanned, Block, Error, ExprBlock, Result};

use crate::{
    yasl_ident::YaslIdent,
    yasl_stmt::YaslStmt,
    yasl_type::{Typed, YaslType},
};

#[derive(Debug)]
pub struct YaslBlock {
    brace_token: syn::token::Brace,
    stmts: Vec<YaslStmt>,
}
impl YaslBlock {
    pub fn attempt_type_anotation(&mut self, global_idents: &HashMap<String, YaslType>) {
        let mut idents = global_idents.clone();

        for stmt in self.stmts.iter_mut() {
            for ident in stmt.update_idents() {
                idents.insert(ident.to_string(), ident.get_type().unwrap().clone());
            }
            stmt.attempt_type_anotation(&idents);
        }
    }
}
impl From<&YaslBlock> for Glsl {
    fn from(block: &YaslBlock) -> Glsl {
        let mut elements = Vec::new();

        elements.push(Glsl::Line(GlslLine {
            span: Some(block.brace_token.span),
            ends_with_semi: false,
            glsl_string: "{".into(),
        }));

        for s in block.stmts.iter() {
            elements.push(s.into());
        }

        elements.push(Glsl::Line(GlslLine {
            span: Some(block.brace_token.span),
            ends_with_semi: false,
            glsl_string: "}".into(),
        }));

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
        let brace_token = block.brace_token;

        Ok(Self { brace_token, stmts })
    }
}

impl TryFrom<ExprBlock> for YaslBlock {
    type Error = Error;
    fn try_from(block: ExprBlock) -> Result<Self> {
        block.block.try_into()
    }
}
