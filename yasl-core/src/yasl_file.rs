use std::{collections::HashMap, convert::TryInto};

use syn::parse::ParseStream;
use syn::Result;

use crate::glsl::{Glsl, GlslFragment};
use crate::{
    yasl_ident::YaslIdent,
    yasl_item::YaslItem,
    yasl_type::{Typed, YaslType},
};

#[derive(Debug)]
pub struct YaslFile {
    items: Vec<YaslItem>,
}

impl YaslFile {}

impl From<YaslFile> for Glsl {
    fn from(mut file: YaslFile) -> Glsl {
        let mut elements = Vec::new();

        let mut global_idents: HashMap<String, YaslType> = HashMap::new();
        for i in file.items.iter_mut() {
            for ident in i.update_idents() {
                global_idents.insert(ident.to_string(), ident.get_type().unwrap().clone());
            }

            i.attempt_type_anotation(&global_idents);

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
