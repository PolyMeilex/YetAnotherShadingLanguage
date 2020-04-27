use std::convert::TryInto;
use syn::parse::{Parse, ParseStream};
use syn::{Error, Ident, Result, Type};
use syn::{LitInt, Token};

use crate::convert::{AsGlsl, Glsl, GlslLine};

use crate::yasl_type::YaslType;

#[derive(Debug)]
enum LayoutKind {
    Input,
    Output,
}
impl AsGlsl for LayoutKind {
    fn as_glsl(&self) -> Glsl {
        Glsl::Expr(
            match self {
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
    ident: Ident,
    ty: YaslType,
}

impl YaslItemLayout {}

impl AsGlsl for YaslItemLayout {
    fn as_glsl(&self) -> Glsl {
        Glsl::Line(GlslLine {
            span: Some(self.ident.span().unstable()),
            ends_with_semi: true,
            glsl_string: format!(
                "layout(location={}) {} {} {}",
                self.pos.to_string(),
                self.kind.as_glsl(),
                self.ty.as_glsl(),
                self.ident.to_string()
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

        let ident: Ident = ps.parse()?;

        let _ = ps.parse::<Token![:]>()?;

        let ty: Type = ps.parse()?;

        let _ = ps.parse::<Token![;]>()?;

        Ok(Self {
            kind,
            pos,
            ident,
            ty: ty.try_into()?,
        })
    }
}
