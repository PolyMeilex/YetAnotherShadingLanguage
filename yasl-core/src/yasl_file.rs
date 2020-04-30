use std::convert::TryInto;

use syn::parse::ParseStream;
use syn::Result;

use crate::glsl::{Glsl, GlslFragment};
use crate::yasl_item::YaslItem;

#[derive(Debug)]
pub struct YaslFile {
    items: Vec<YaslItem>,
}

impl YaslFile {}

impl From<&YaslFile> for Glsl {
    fn from(file: &YaslFile) -> Glsl {
        let mut elements = Vec::new();

        for i in file.items.iter() {
            elements.push(i.into());
        }

        Glsl::Fragment(GlslFragment { elements })
    }
}

syn::custom_keyword!(layout);

impl YaslFile {
    pub fn parse(ps: ParseStream) -> Result<Self> {
        let mut items = Vec::new();
        while !ps.is_empty() {
            if ps.peek(layout) {
                use crate::yasl_item::YaslItemLayout;
                let layout = ps.parse::<YaslItemLayout>()?;
                items.push(layout.into());
            } else {
                let item: syn::Item = ps.parse()?;
                items.push(item.try_into()?);
            }
        }
        Ok(Self { items })
    }
}
