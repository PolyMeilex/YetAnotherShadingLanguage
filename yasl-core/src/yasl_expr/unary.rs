use std::convert::{TryFrom, TryInto};

use proc_macro2::Span;
use syn::{spanned::Spanned, Error, ExprUnary, Result};

use quote::quote;

use crate::glsl::Glsl;

use super::YaslExprLineScope;

#[derive(Debug)]
pub struct YaslExprUnary {
    op: syn::UnOp,
    expr: Box<YaslExprLineScope>,
}
impl YaslExprUnary {
    pub fn span(&self) -> Span {
        self.op.span()
    }
}

impl From<&YaslExprUnary> for Glsl {
    fn from(expr: &YaslExprUnary) -> Glsl {
        let op = expr.op;
        let op = quote!(#op).to_string();

        Glsl::Expr(format!("{}{}", op, Glsl::from(&*expr.expr)))
    }
}

impl TryFrom<ExprUnary> for YaslExprUnary {
    type Error = Error;
    fn try_from(u: ExprUnary) -> Result<Self> {
        let op = u.op;
        let expr = Box::new((*u.expr).try_into()?);

        Ok(Self { op, expr })
    }
}
