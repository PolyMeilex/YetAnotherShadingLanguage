use std::convert::{TryFrom, TryInto};

use proc_macro::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Error, Ident, Result};
use syn::{ExprPath, PatType, Path};

#[derive(Debug)]
pub struct YaslIdent(Ident);
impl YaslIdent {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
    pub fn as_glsl(&self) -> String {
        self.to_string()
    }
    pub fn span(&self) -> Span {
        self.0.span().unstable()
    }
}

impl From<Ident> for YaslIdent {
    fn from(i: Ident) -> Self {
        Self(i)
    }
}

impl TryFrom<Path> for YaslIdent {
    type Error = Error;
    fn try_from(p: Path) -> Result<Self> {
        let i = if let Some(i) = p.get_ident() {
            i.clone()
        } else {
            if !p.segments.is_empty() {
                let span = p.segments[0].ident.span();
                let mut i = Vec::new();
                for s in p.segments.into_iter() {
                    i.push(s.ident.to_string())
                }

                if i[0] != "glsl" {
                    return Err(Error::new(span, "Only 'glsl' namespace is posible"));
                }

                let s = i.join("_");

                Ident::new(&s, span)
            } else {
                return Err(Error::new(p.span(), "Expected Ident"));
            }
        };

        Ok(i.into())
    }
}

impl TryFrom<ExprPath> for YaslIdent {
    type Error = Error;
    fn try_from(p: ExprPath) -> Result<Self> {
        let i = p.path.try_into()?;
        Ok(i)
    }
}

impl TryFrom<PatType> for YaslIdent {
    type Error = Error;
    fn try_from(t: PatType) -> Result<Self> {
        if let syn::Pat::Ident(i) = *t.pat {
            Ok(i.ident.into())
        } else {
            return Err(Error::new(t.pat.span(), "Expected Ident"));
        }
    }
}
