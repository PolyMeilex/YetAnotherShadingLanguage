use crate::glsl::{Glsl, GlslFragment};
use std::convert::{TryFrom, TryInto};
use syn::{Block, Error, Result};

use crate::yasl_stmt::YaslStmt;

#[derive(Debug)]
pub struct YaslBlock {
    stmts: Vec<YaslStmt>,
}

impl From<&YaslBlock> for Glsl {
    fn from(block: &YaslBlock) -> Glsl {
        let mut elements = Vec::new();

        for s in block.stmts.iter() {
            elements.push(s.into());
        }

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
