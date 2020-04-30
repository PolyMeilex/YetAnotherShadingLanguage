use std::convert::TryInto;
use syn::parse::{Parse, ParseStream};
use syn::{Error, Result, Type};
use syn::{LitInt, Token};

use crate::glsl::{Glsl, GlslLine};

use crate::yasl_ident::YaslIdent;
use crate::yasl_type::YaslType;

#[derive(Debug)]
enum LayoutKind {
    Input,
    Output,
}

impl From<&LayoutKind> for Glsl {
    fn from(kind: &LayoutKind) -> Glsl {
        Glsl::Expr(
            match kind {
                LayoutKind::Input => "in",
                LayoutKind::Output => "out",
            }
            .to_string(),
        )
    }
}

#[derive(Debug)]
pub struct YaslItemLayout {
    kind: LayoutKind,
    pos: usize,
    ident: YaslIdent,
    ty: YaslType,
}

impl YaslItemLayout {}

impl From<&YaslItemLayout> for Glsl {
    fn from(item: &YaslItemLayout) -> Glsl {
        Glsl::Line(GlslLine {
            span: Some(item.ident.span()),
            ends_with_semi: true,
            glsl_string: format!(
                "layout(location={}) {} {} {}",
                item.pos.to_string(),
                Glsl::from(&item.kind),
                Glsl::from(&item.ty),
                Glsl::from(&item.ident),
            ),
        })
    }
}

syn::custom_keyword!(layout);
syn::custom_keyword!(input);
syn::custom_keyword!(output);

impl Parse for YaslItemLayout {
    fn parse(ps: ParseStream) -> Result<Self> {
        let let_token = ps.parse::<layout>()?;
        let _ = ps.parse::<Token![<]>()?;

        let kind = if ps.peek(input) {
            ps.parse::<input>()?;
            LayoutKind::Input
        } else if ps.peek(output) {
            ps.parse::<output>()?;
            LayoutKind::Output
        } else {
            return Err(Error::new(let_token.span, "Expected input or output kind"));
        };

        let _ = ps.parse::<Token![,]>()?;
        let pos = ps.parse::<LitInt>()?;
        let pos = pos.base10_parse()?;
        let _ = ps.parse::<Token![>]>()?;

        let ident: syn::Ident = ps.parse()?;

        let _ = ps.parse::<Token![:]>()?;

        let ty: Type = ps.parse()?;

        let _ = ps.parse::<Token![;]>()?;

        Ok(Self {
            kind,
            pos,
            ident: ident.into(),
            ty: ty.try_into()?,
        })
    }
}
