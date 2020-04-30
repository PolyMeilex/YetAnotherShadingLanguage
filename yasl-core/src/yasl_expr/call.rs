use std::convert::{TryFrom, TryInto};

use proc_macro2::Span;
use syn::{punctuated::Punctuated, spanned::Spanned, Error, Result};

use syn::{Expr, ExprCall};

use crate::glsl::Glsl;
use crate::yasl_ident::YaslIdent;

use super::YaslExprLineScope;

#[derive(Debug)]
pub struct YaslExprCall {
    ident: YaslIdent,
    args: Punctuated<YaslExprLineScope, syn::token::Comma>,
}
impl YaslExprCall {
    pub fn span(&self) -> Span {
        self.ident.span()
    }
}

impl From<&YaslExprCall> for Glsl {
    fn from(expr: &YaslExprCall) -> Glsl {
        let ident = Glsl::from(&expr.ident);

        let mut args: Vec<String> = Vec::new();

        for a in expr.args.iter() {
            args.push(Glsl::from(a).into());
        }

        Glsl::Expr(format!("{}({})", ident, args.join(",")))
    }
}

impl TryFrom<ExprCall> for YaslExprCall {
    type Error = Error;
    fn try_from(c: ExprCall) -> Result<Self> {
        let ident = if let Expr::Path(p) = *c.func {
            p.try_into()?
        } else {
            return Err(Error::new(c.func.span(), "Expected Ident"));
        };

        let mut args = Punctuated::new();

        for e in c.args.into_iter() {
            args.push(e.try_into()?);
        }

        Ok(Self { ident, args })
    }
}
