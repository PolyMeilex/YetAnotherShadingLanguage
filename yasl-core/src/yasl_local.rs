use crate::convert::{AsGlsl, Glsl, GlslLine};
use std::convert::{TryFrom, TryInto};

use syn::spanned::Spanned;
use syn::{Error, Local, Pat, Result};

use crate::yasl_expr::YaslExprLineScope;
use crate::yasl_ident::YaslIdent;
use crate::yasl_type::YaslType;

#[derive(Debug)]
pub struct YaslLocal {
    ident: YaslIdent,
    ty: YaslType,
    init: Option<YaslExprLineScope>,
}

impl AsGlsl for YaslLocal {
    fn as_glsl(&self) -> Glsl {
        let init_glsl = if let Some(i) = &self.init {
            let mut init_glsl = String::new();
            init_glsl += "= ";
            init_glsl += &i.as_glsl().to_string();
            init_glsl
        } else {
            String::new()
        };

        Glsl::Line(GlslLine {
            span: Some(self.ident.span()),
            ends_with_semi: true,
            glsl_string: format!(
                "{} {} {}",
                self.ty.as_glsl(),
                self.ident.to_string(),
                init_glsl
            ),
        })
    }
}

impl TryFrom<Local> for YaslLocal {
    type Error = Error;
    fn try_from(l: Local) -> Result<Self> {
        let (ident, ty) = if let Pat::Type(t) = l.pat {
            let ident = t.clone().try_into()?;
            let ty = (*t.ty).try_into()?;

            (ident, ty)
        } else {
            return Err(Error::new(l.pat.span(), "Expected Type"));
        };

        let init = if let Some((_eq, expr)) = l.init {
            let expr: YaslExprLineScope = (*expr).try_into()?;
            Some(expr)
        } else {
            None
        };

        Ok(Self { ty, ident, init })
    }
}
