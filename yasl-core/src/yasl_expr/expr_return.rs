use std::convert::{TryFrom, TryInto};

use proc_macro2::Span;
use syn::{spanned::Spanned, Error, ExprReturn, Result};

use crate::glsl::Glsl;

use super::YaslExprLineScope;

#[derive(Debug)]
pub struct YaslExprReturn {
    return_token: syn::token::Return,
    expr: Option<Box<YaslExprLineScope>>,
}
impl YaslExprReturn {
    pub fn span(&self) -> Span {
        self.return_token.span()
    }
}

impl From<&YaslExprReturn> for Glsl {
    fn from(expr: &YaslExprReturn) -> Glsl {
        let glsl_expr = if let Some(expr) = &expr.expr {
            format!("{}", Glsl::from(&**expr))
        } else {
            String::new()
        };

        Glsl::Expr(format!("return {}", glsl_expr))
    }
}

impl TryFrom<ExprReturn> for YaslExprReturn {
    type Error = Error;
    fn try_from(r: ExprReturn) -> Result<Self> {
        let return_token = r.return_token;
        let expr = if let Some(expr) = r.expr {
            Some(Box::new(YaslExprLineScope::try_from(*expr)?))
        } else {
            None
        };

        Ok(Self { return_token, expr })
    }
}
