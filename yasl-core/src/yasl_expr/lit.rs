use std::convert::TryFrom;

use syn::{Error, Lit, Result};

use syn::ExprLit;

use quote::quote;

use crate::glsl::Glsl;

#[derive(Debug)]
pub struct YaslExprLit {
    lit: Lit,
}

impl From<&YaslExprLit> for Glsl {
    fn from(expr: &YaslExprLit) -> Glsl {
        let lit = &expr.lit;
        Glsl::Expr(quote!(#lit).to_string())
    }
}

impl TryFrom<ExprLit> for YaslExprLit {
    type Error = Error;
    fn try_from(l: ExprLit) -> Result<Self> {
        let lit = l.lit;
        Ok(Self { lit })
    }
}
