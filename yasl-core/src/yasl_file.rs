use std::convert::TryInto;

use syn::parse::ParseStream;
use syn::Result;

use crate::convert::{AsGlsl, Glsl, GlslFragment};
use crate::yasl_item::YaslItem;

#[derive(Debug)]
pub struct YaslFile {
    items: Vec<YaslItem>,
}

impl YaslFile {}

impl AsGlsl for YaslFile {
    fn as_glsl(&self) -> Glsl {
        let mut elements = Vec::new();

        for i in self.items.iter() {
            elements.push(i.as_glsl());
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
