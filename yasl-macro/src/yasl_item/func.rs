use std::convert::{TryFrom, TryInto};
use syn::{spanned::Spanned, Error, FnArg, Ident, ItemFn, Result};

use crate::convert::{AsGlsl, Glsl, GlslFragment, GlslLine};
use crate::yasl_block::YaslBlock;
use crate::yasl_type::YaslType;

#[derive(Debug)]
pub struct YaslItemFn {
    ident: Ident,
    args: Vec<(Ident, YaslType)>,
    output: YaslType,
    block: Box<YaslBlock>,
}

impl AsGlsl for YaslItemFn {
    fn as_glsl(&self) -> Glsl {
        let mut args = Vec::new();

        for a in self.args.iter() {
            args.push(format!("{} {}", a.1.as_glsl(), a.0.to_string()));
        }

        let args_glsl = args.join(",");

        let mut elements = Vec::new();

        elements.push(Glsl::Line(GlslLine {
            span: Some(self.ident.span().unstable()),
            ends_with_semi: false,
            glsl_string: format!(
                "{} yasl_{}({}) {{",
                self.output.as_glsl().to_string(),
                self.ident.to_string(),
                args_glsl
            ),
        }));

        let block = self.block.as_glsl();

        elements.push(block);

        elements.push(Glsl::Line(GlslLine {
            span: Some(self.ident.span().unstable()),
            ends_with_semi: false,
            glsl_string: "}".into(),
        }));
        // if Glsl::Fragment(f) = block{

        // }

        // format!(
        //     "{} {}({}) {}",
        //     self.output.as_glsl(),
        //     self.ident.to_string(),
        //     args_glsl,
        //     self.block.as_glsl()
        // )
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

                args.push((ident, ty));
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
            ident,
            args,
            output,
            block: Box::new(block),
        })
    }
}
