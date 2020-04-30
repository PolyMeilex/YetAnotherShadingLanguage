use std::convert::{TryFrom, TryInto};

use proc_macro2::Span;
use syn::{spanned::Spanned, Error, ExprAssign, Result};

use crate::glsl::Glsl;
use crate::yasl_ident::YaslIdent;

use super::YaslExprLineScope;

#[derive(Debug)]
pub struct YaslExprAssign {
    left: YaslIdent,
    right: YaslExprLineScope,
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

impl TryFrom<ExprAssign> for YaslExprAssign {
    type Error = Error;
    fn try_from(c: ExprAssign) -> Result<Self> {
        let left = match *c.left {
            syn::Expr::Path(p) => p.try_into()?,
            _ => return Err(Error::new(c.left.span(), "Expeted Ident")),
        };
        let right: YaslExprLineScope = (*c.right).try_into()?;

        Ok(Self { left, right })
    }
}
