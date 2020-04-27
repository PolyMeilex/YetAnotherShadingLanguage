use std::convert::{TryFrom, TryInto};

use proc_macro::Span;
use syn::{punctuated::Punctuated, spanned::Spanned, Error, Result};

use syn::{Expr, ExprCall};

use crate::convert::{AsGlsl, Glsl};
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

impl AsGlsl for YaslExprCall {
    fn as_glsl(&self) -> Glsl {
        let ident = self.ident.to_string();

        let mut args = Vec::new();

        for a in self.args.iter() {
            args.push(a.as_glsl().to_string());
        }

        if ident.starts_with("glsl_") {
            let ident = ident.split("glsl_").collect::<Vec<&str>>()[1];
            Glsl::Expr(format!("{}({})", ident, args.join(",")))
        } else {
            Glsl::Expr(format!("yasl_{}({})", ident, args.join(",")))
        }
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
