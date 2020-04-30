use std::convert::{TryFrom, TryInto};

use syn::{BinOp, Error, Result};

use syn::ExprBinary;

use quote::quote;

use crate::glsl::Glsl;

use super::YaslExprLineScope;

#[derive(Debug)]
pub struct YaslExprBinary {
    left: Box<YaslExprLineScope>,
    op: BinOp,
    right: Box<YaslExprLineScope>,
}

impl From<&YaslExprBinary> for Glsl {
    fn from(expr: &YaslExprBinary) -> Glsl {
        let left: Glsl = (&*expr.left).into();
        let op = &expr.op;
        let op = quote!(#op).to_string();
        let right: Glsl = (&*expr.right).into();

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
