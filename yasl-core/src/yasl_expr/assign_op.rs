use std::convert::{TryFrom, TryInto};

use proc_macro2::Span;
use syn::{spanned::Spanned, BinOp, Error, ExprAssignOp, Result};

use quote::quote;

use crate::convert::{AsGlsl, Glsl};
use crate::yasl_ident::YaslIdent;

use super::YaslExprLineScope;

#[derive(Debug)]
pub struct YaslExprAssignOp {
    left: YaslIdent,
    op: BinOp,
    right: YaslExprLineScope,
}

impl YaslExprAssignOp {
    pub fn span(&self) -> Span {
        self.left.span()
    }
}

impl AsGlsl for YaslExprAssignOp {
    fn as_glsl(&self) -> Glsl {
        let op = self.op;
        let op = quote!(#op).to_string();
        Glsl::Expr(format!(
            "{} {} {}",
            self.left.as_glsl(),
            op,
            self.right.as_glsl()
        ))
    }
}

impl TryFrom<ExprAssignOp> for YaslExprAssignOp {
    type Error = Error;
    fn try_from(c: ExprAssignOp) -> Result<Self> {
        let left = match *c.left {
            syn::Expr::Path(p) => p.try_into()?,
            _ => return Err(Error::new(c.left.span(), "Expeted Ident")),
        };
        let op = c.op;
        let right: YaslExprLineScope = (*c.right).try_into()?;

        Ok(Self { left, op, right })
    }
}
