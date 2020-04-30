use crate::glsl::Glsl;
use std::convert::{TryFrom, TryInto};

use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{Error, Ident, Result};
use syn::{ExprPath, PatType, Path};

#[derive(Debug)]
pub struct YaslIdent {
    prefix: String,
    ident: Ident,
}
impl YaslIdent {
    fn to_string(&self) -> String {
        format!("{}{}", self.prefix, self.ident.to_string())
    }
    pub fn span(&self) -> Span {
        self.ident.span()
    }
}
impl From<&YaslIdent> for Glsl {
    fn from(ident: &YaslIdent) -> Glsl {
        Glsl::Expr(ident.to_string())
    }
}

impl From<Ident> for YaslIdent {
    fn from(ident: Ident) -> Self {
        Self {
            prefix: "yasl_".into(),
            ident,
        }
    }
}

impl TryFrom<Path> for YaslIdent {
    type Error = Error;
    fn try_from(p: Path) -> Result<Self> {
        let i = if let Some(i) = p.get_ident() {
            i.clone().into()
        } else {
            if p.segments.len() == 2 {
                let mut iter = p.segments.into_iter();

                let prefix = iter.next().unwrap().ident;

                if prefix.to_string() != "glsl" {
                    return Err(Error::new(
                        prefix.span(),
                        "Only 'glsl' namespace is posible",
                    ));
                }

                let ident = iter.next().unwrap().ident;

                Self {
                    // GLSL Prefix means that we need to left prefix empty to call glsl buildins
                    prefix: "".into(),
                    ident,
                }
            } else {
                return Err(Error::new(p.span(), "Expected Ident"));
            }
        };

        Ok(i)
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
