use std::convert::TryFrom;

use syn::{Error, Result};

use syn::ExprLit;

use quote::quote;

use crate::{
    glsl::Glsl,
    yasl_type::{Typed, YaslScalarType, YaslType},
};

#[derive(Debug)]
pub struct YaslExprLit {
    lit: syn::Lit,
}

impl Typed for YaslExprLit {
    fn get_type(&self) -> Option<YaslType> {
        use syn::Lit;
        match self.lit {
            Lit::Float(_) => Some(YaslType::ScalarType(YaslScalarType::Float32)),
            Lit::Int(_) => Some(YaslType::ScalarType(YaslScalarType::Int)),
            Lit::Bool(_) => Some(YaslType::ScalarType(YaslScalarType::Bool)),
            _ => None,
        }
    }
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
