use crate::glsl::{Glsl, GlslLine};
use crate::yasl_expr::YaslExprLineScope;
use crate::yasl_type::YaslType;
use std::convert::{TryFrom, TryInto};
use syn::{spanned::Spanned, Error, ItemStatic, Result};

use crate::yasl_ident::YaslIdent;

#[derive(Debug)]
pub struct YaslItemStatic {
    static_token: syn::token::Static,
    ident: YaslIdent,
    ty: YaslType,
    expr: YaslExprLineScope,
}

impl From<&YaslItemStatic> for Glsl {
    fn from(item: &YaslItemStatic) -> Glsl {
        Glsl::Line(GlslLine {
            span: Some(item.ident.span()),
            ends_with_semi: true,
            glsl_string: format!(
                "{} {} = {}",
                Glsl::from(&item.ty),
                Glsl::from(&item.ident),
                Glsl::from(&item.expr),
            ),
        })
    }
}

impl TryFrom<ItemStatic> for YaslItemStatic {
    type Error = Error;
    fn try_from(item: ItemStatic) -> Result<Self> {
        if item.vis != syn::Visibility::Inherited {
            return Err(Error::new(
                item.span(),
                "Visibility Keywords are not supported",
            ));
        }
        if item.mutability.is_some() {
            return Err(Error::new(item.span(), "Mut Keyword is not supported"));
        }
        Ok(Self {
            static_token: item.static_token,
            ident: item.ident.into(),
            ty: (*item.ty).try_into()?,
            expr: (*item.expr).try_into()?,
        })
    }
}
