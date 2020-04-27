use std::convert::{TryFrom, TryInto};

use syn::{Error, Result};

use syn::ExprCast;

use crate::convert::{AsGlsl, Glsl};
use crate::yasl_type::YaslType;

use super::YaslExprLineScope;

#[derive(Debug)]
pub struct YaslExprCast {
    expr: Box<YaslExprLineScope>,
    ty: Box<YaslType>,
}

impl AsGlsl for YaslExprCast {
    fn as_glsl(&self) -> Glsl {
        Glsl::Expr(format!("{}({})", self.ty.as_glsl(), self.expr.as_glsl()))
    }
}

impl TryFrom<ExprCast> for YaslExprCast {
    type Error = Error;
    fn try_from(c: ExprCast) -> Result<Self> {
        let expr: YaslExprLineScope = (*c.expr).try_into()?;

        let expr = Box::new(expr);

        let ty = Box::new((*c.ty).try_into()?);
        Ok(Self { expr, ty })
    }
}
