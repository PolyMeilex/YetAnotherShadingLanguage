use std::convert::{TryFrom, TryInto};

use proc_macro2::Span;
use syn::{spanned::Spanned, Error, ExprAssign, Result};

use crate::convert::{AsGlsl, Glsl};
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

impl AsGlsl for YaslExprAssign {
    fn as_glsl(&self) -> Glsl {
        Glsl::Expr(format!(
            "{} = {}",
            self.left.as_glsl(),
            self.right.as_glsl()
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
