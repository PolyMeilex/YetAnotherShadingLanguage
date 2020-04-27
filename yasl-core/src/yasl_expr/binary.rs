use std::convert::{TryFrom, TryInto};

use syn::{BinOp, Error, Result};

use syn::ExprBinary;

use quote::quote;

use crate::convert::{AsGlsl, Glsl};

use super::YaslExprLineScope;

#[derive(Debug)]
pub struct YaslExprBinary {
    left: Box<YaslExprLineScope>,
    op: BinOp,
    right: Box<YaslExprLineScope>,
}

impl AsGlsl for YaslExprBinary {
    fn as_glsl(&self) -> Glsl {
        let left = self.left.as_glsl();
        let op = &self.op;
        let op = quote!(#op).to_string();
        let right = self.right.as_glsl();

        Glsl::Expr(format!("{} {} {}", left, op, right))
    }
}

impl TryFrom<ExprBinary> for YaslExprBinary {
    type Error = Error;
    fn try_from(bin: ExprBinary) -> Result<Self> {
        Ok(Self {
            left: Box::new((*bin.left).try_into()?),
            op: bin.op,
            right: Box::new((*bin.right).try_into()?),
        })
    }
}
