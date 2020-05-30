use crate::{
    glsl::Glsl,
    yasl_type::{Typed, YaslType},
};
use std::convert::{TryFrom, TryInto};

use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{Error, Ident, Result};
use syn::{ExprPath, PatType, Path};

#[derive(Debug, Clone)]
pub struct YaslIdent {
    prefix: String,
    ident: Ident,
    ty: Option<YaslType>,
}
impl YaslIdent {
    pub fn to_string(&self) -> String {
        format!("{}{}", self.prefix, self.ident.to_string())
    }
    pub fn span(&self) -> Span {
        self.ident.span()
    }
    pub fn set_type(&mut self, ty: YaslType) {
        self.ty = Some(ty);
    }
}

impl Typed for YaslIdent {
    fn get_type(&self) -> Option<YaslType> {
        self.ty.clone()
    }
}

impl From<&YaslIdent> for Glsl {
    fn from(ident: &YaslIdent) -> Glsl {
        Glsl::Expr(ident.to_string())
    }
}

impl From<Ident> for YaslIdent {
    fn from(ident: Ident) -> Self {
        // TODO MOVE TIHS TO KEYWORDS MODULE
        let prefix = match ident.to_string().as_str() {
            "vec2" | "vec3" | "vec4" => "",
            _ => "yasl_",
        }
        .into();
        Self {
            prefix,
            ident,
            ty: None,
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

                let prefix = iter.next().unwrap().ident.to_string();

                let prefix = match prefix.as_str() {
                    "glsl" => "",
                    "f32" => "",
                    "f64" => "d",
                    "bool" => "b",
                    "i32" => "i",
                    "u32" => "u",
                    _ => {
                        return Err(Error::new(
                            prefix.span(),
                            "Only 'glsl,f32,f64,bool,i32,u32' prefix is allowed",
                        ))
                    }
                }
                .into();

                let ident = iter.next().unwrap().ident;

                Self {
                    prefix,
                    ident,
                    ty: None,
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
