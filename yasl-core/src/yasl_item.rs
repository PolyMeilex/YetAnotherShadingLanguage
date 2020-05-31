use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
};
use syn::{spanned::Spanned, Error, Item, Result};

use crate::{
    glsl::Glsl,
    yasl_ident::YaslIdent,
    yasl_type::{Typed, YaslType},
};

mod static_it;
use static_it::YaslItemStatic;

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

impl YaslItem {
    pub fn update_idents(&mut self) -> Vec<YaslIdent> {
        match self {
            YaslItem::Static(i) => vec![i.get_ident()],
            YaslItem::Fn(f) => vec![f.get_ident()],
            _ => Vec::new(),
        }
    }
    pub fn attempt_type_anotation(&mut self, idents: &HashMap<String, YaslType>) {
        match self {
            YaslItem::Fn(f) => f.attempt_type_anotation(idents),
            _ => {}
        }
    }
}

impl From<&YaslItem> for Glsl {
    fn from(item: &YaslItem) -> Glsl {
        match item {
            YaslItem::Static(s) => s.into(),
            YaslItem::Layout(l) => l.into(),
            YaslItem::Fn(f) => f.into(),
        }
    }
}

impl From<&mut YaslItem> for Glsl {
    fn from(item: &mut YaslItem) -> Glsl {
        match item {
            YaslItem::Static(ref s) => s.into(),
            YaslItem::Layout(ref l) => l.into(),
            YaslItem::Fn(ref f) => f.into(),
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
