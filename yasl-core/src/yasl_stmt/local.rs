use crate::glsl::{Glsl, GlslLine};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
};

use syn::spanned::Spanned;
use syn::{Error, Local, Pat, Result};

use crate::yasl_expr::YaslExprLineScope;
use crate::yasl_ident::YaslIdent;
use crate::yasl_type::{Typed, YaslType};

#[derive(Debug)]
pub struct YaslLocal {
    ident: YaslIdent,
    ty: YaslType,
    init: Option<YaslExprLineScope>,
}

impl YaslLocal {
    pub fn attempt_type_anotation(&mut self, idents: &HashMap<String, YaslType>) {
        if let Some(init) = &mut self.init {
            init.attempt_type_anotation(idents);
            if let Some(ty) = init.get_type() {
                self.ty = ty;
            }
        }
    }
}

impl Typed for YaslLocal {
    fn get_type(&self) -> Option<YaslType> {
        Some(self.ty.clone())
    }
}

impl From<&YaslLocal> for Glsl {
    fn from(local: &YaslLocal) -> Glsl {
        let init_glsl = if let Some(init) = &local.init {
            format!("= {}", Glsl::from(init))
        } else {
            String::new()
        };

        Glsl::Line(GlslLine {
            span: Some(local.ident.span()),
            ends_with_semi: true,
            glsl_string: format!(
                "{} {} {}",
                Glsl::from(&local.ty).to_string(),
                Glsl::from(&local.ident),
                init_glsl
            ),
        })
    }
}

impl TryFrom<Local> for YaslLocal {
    type Error = Error;
    fn try_from(l: Local) -> Result<Self> {
        let init = if let Some((_eq, expr)) = l.init {
            let expr: YaslExprLineScope = (*expr).try_into()?;
            Some(expr)
        } else {
            None
        };

        let pat_span = l.pat.span();

        let p: Option<(YaslIdent, YaslType)> = match l.pat {
            Pat::Type(t) => {
                let ident = t.clone().try_into()?;
                let ty = (*t.ty).try_into()?;

                Some((ident, ty))
            }
            Pat::Ident(i) => {
                if let Some(init) = &init {
                    if let Some(t) = init.get_type() {
                        Some((i.ident.into(), t))
                    } else {
                        Some((i.ident.into(), YaslType::Void))
                    }
                } else {
                    None
                }
            }
            _ => None,
        };

        let (ident, ty) = if let Some(p) = p {
            p
        } else {
            return Err(Error::new(pat_span, "Expected Type"));
        };

        Ok(Self { ty, ident, init })
    }
}
