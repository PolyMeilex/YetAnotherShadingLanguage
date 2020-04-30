use std::convert::{TryFrom, TryInto};

use syn::{Error, Result};

use syn::ExprCast;

use crate::glsl::Glsl;
use crate::yasl_type::YaslType;

use super::YaslExprLineScope;

#[derive(Debug)]
pub struct YaslExprCast {
    expr: Box<YaslExprLineScope>,
    ty: Box<YaslType>,
}

impl From<&YaslExprCast> for Glsl {
    fn from(expr: &YaslExprCast) -> Glsl {
        Glsl::Expr(format!(
            "{}({})",
            Glsl::from(&*expr.ty),
            Glsl::from(&*expr.expr)
        ))
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
