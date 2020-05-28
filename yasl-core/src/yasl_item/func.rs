use std::convert::{TryFrom, TryInto};
use syn::{spanned::Spanned, Error, FnArg, ItemFn, Result};

use crate::glsl::{Glsl, GlslFragment, GlslLine};
use crate::yasl_block::YaslBlock;
use crate::yasl_ident::YaslIdent;
use crate::yasl_type::YaslType;

#[derive(Debug)]
pub struct YaslItemFn {
    ident: YaslIdent,
    args: Vec<(YaslIdent, YaslType)>,
    output: YaslType,
    block: Box<YaslBlock>,
}

impl From<&YaslItemFn> for Glsl {
    fn from(item: &YaslItemFn) -> Glsl {
        let mut args = Vec::new();

        for a in item.args.iter() {
            args.push(format!("{} {}", Glsl::from(&a.1), Glsl::from(&a.0)));
        }

        let args_glsl = args.join(",");

        let mut elements = Vec::new();

        elements.push(Glsl::Line(GlslLine {
            span: Some(item.ident.span()),
            ends_with_semi: false,
            glsl_string: format!(
                "{} {}({})",
                Glsl::from(&item.output),
                Glsl::from(&item.ident),
                args_glsl
            ),
        }));

        elements.push((&*item.block).into());

        // elements.push(Glsl::Line(GlslLine {
        //     span: Some(item.ident.span()),
        //     ends_with_semi: false,
        //     glsl_string: "}".into(),
        // }));

        Glsl::Fragment(GlslFragment { elements })
    }
}

impl TryFrom<ItemFn> for YaslItemFn {
    type Error = Error;
    fn try_from(f: ItemFn) -> Result<Self> {
        if f.vis != syn::Visibility::Inherited {
            return Err(Error::new(
                f.span(),
                "Visibility Keywords are not supported",
            ));
        }

        if f.sig.constness.is_some()
            || f.sig.asyncness.is_some()
            || f.sig.unsafety.is_some()
            || f.sig.abi.is_some()
        {
            return Err(Error::new(f.span(), "This Keywords is not supported"));
        }

        //TODO: Error Out On Generics

        let ident = f.sig.ident;

        let inputs = f.sig.inputs;

        let mut args = Vec::new();

        for i in inputs.into_iter() {
            if let FnArg::Typed(t) = i {
                let ident = if let syn::Pat::Ident(i) = *t.pat {
                    i.ident
                } else {
                    return Err(Error::new(t.pat.span(), "Expected Ident"));
                };

                let ty = (*t.ty).try_into()?;

                args.push((ident.into(), ty));
            } else {
                return Err(Error::new(i.span(), "Expected Type"));
            };
        }

        let output = f.sig.output;

        let output = match output {
            syn::ReturnType::Default => YaslType::Void,
            syn::ReturnType::Type(_, t) => (*t).try_into()?,
        };

        let block = (*f.block).try_into()?;

        Ok(Self {
            ident: ident.into(),
            args,
            output,
            block: Box::new(block),
        })
    }
}
