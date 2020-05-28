use std::convert::{TryFrom, TryInto};

use proc_macro2::Span;
use syn::{spanned::Spanned, Error, ExprAssign, Result};

use crate::glsl::Glsl;
use crate::yasl_ident::YaslIdent;

use super::YaslExprLineScope;

#[derive(Debug)]
pub struct YaslExprUnary {
    op: syn::UnOp,
    expr: Box<YaslExprLineScope>,
}
impl YaslExprAssign {
    pub fn span(&self) -> Span {
        self.left.span()
    }
}

impl From<&YaslExprAssign> for Glsl {
    fn from(expr: &YaslExprAssign) -> Glsl {
        Glsl::Expr(format!(
            "{} = {}",
            Glsl::from(&expr.left),
            Glsl::from(&expr.right),
        ))
    }
}

impl TryFrom<ExprUnary> for YaslExprUnary {
    type Error = Error;
    fn try_from(u: ExprUnary) -> Result<Self> {
        let op = 

        Ok(Self { left, right })
    }
}
