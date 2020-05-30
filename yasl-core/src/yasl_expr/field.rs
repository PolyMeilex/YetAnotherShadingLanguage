use std::convert::{TryFrom, TryInto};

use proc_macro2::Span;
use syn::{spanned::Spanned, Error, ExprField, Result};

use crate::glsl::Glsl;

use super::YaslExprLineScope;

#[derive(Debug)]
pub struct YaslExprField {
    base: Box<YaslExprLineScope>,
    member: syn::Ident,
}
impl YaslExprField {
    pub fn span(&self) -> Span {
        self.member.span()
    }
}

impl From<&YaslExprField> for Glsl {
    fn from(expr: &YaslExprField) -> Glsl {
        Glsl::Expr(format!(
            "{}.{}",
            Glsl::from(&*expr.base),
            expr.member.to_string()
        ))
    }
}

impl TryFrom<ExprField> for YaslExprField {
    type Error = Error;
    fn try_from(f: ExprField) -> Result<Self> {
        let base = Box::new((*f.base).try_into()?);

        let member = match f.member {
            syn::Member::Named(i) => i.into(),
            syn::Member::Unnamed(i) => Err(Error::new(i.span(), "Expected Ident"))?,
        };

        Ok(Self { base, member })
    }
}
