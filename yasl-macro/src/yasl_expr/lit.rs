use std::convert::TryFrom;

use syn::{Error, Lit, Result};

use syn::ExprLit;

use quote::quote;

use crate::convert::{AsGlsl, Glsl};

#[derive(Debug)]
pub struct YaslExprLit {
    lit: Lit,
}

impl AsGlsl for YaslExprLit {
    fn as_glsl(&self) -> Glsl {
        let lit = &self.lit;

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
