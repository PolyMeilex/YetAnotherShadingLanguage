use std::convert::{TryFrom, TryInto};
use syn::{spanned::Spanned, Error, Item, ItemStatic, Result};

use crate::convert::{AsGlsl, Glsl, GlslLine};
use crate::yasl_expr::YaslExprLineScope;
use crate::yasl_type::YaslType;

mod func;
use func::YaslItemFn;

mod layout;
pub use layout::YaslItemLayout;

#[derive(Debug)]
pub enum YaslItem {
    Static(YaslItemStatic),
    Layout(YaslItemLayout),
    Fn(YaslItemFn),
}

impl YaslItem {}

impl AsGlsl for YaslItem {
    fn as_glsl(&self) -> Glsl {
        match self {
            Self::Static(s) => s.as_glsl(),
            Self::Layout(l) => l.as_glsl(),
            Self::Fn(f) => f.as_glsl(),
        }
    }
}

impl TryFrom<Item> for YaslItem {
    type Error = Error;
    fn try_from(item: Item) -> Result<Self> {
        Ok(match item {
            Item::Static(s) => Self::Static(s.try_into()?),
            Item::Fn(f) => Self::Fn(f.try_into()?),
            _ => return Err(Error::new(item.span(), "Unsuported Item")),
        })
    }
}

impl From<YaslItemLayout> for YaslItem {
    fn from(layout: YaslItemLayout) -> Self {
        YaslItem::Layout(layout)
    }
}

#[derive(Debug)]
pub struct YaslItemStatic {
    static_token: syn::token::Static,
    ident: syn::Ident,
    ty: YaslType,
    expr: YaslExprLineScope,
}

impl AsGlsl for YaslItemStatic {
    fn as_glsl(&self) -> Glsl {
        Glsl::Line(GlslLine {
            span: Some(self.ident.span()),
            ends_with_semi: true,
            glsl_string: format!(
                "{} {} = {}",
                self.ty.as_glsl(),
                self.ident.to_string(),
                self.expr.as_glsl()
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
            ident: item.ident,
            ty: (*item.ty).try_into()?,
            expr: (*item.expr).try_into()?,
        })
    }
}
