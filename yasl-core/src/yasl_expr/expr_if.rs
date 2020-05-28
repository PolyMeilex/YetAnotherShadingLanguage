use std::convert::{TryFrom, TryInto};

use proc_macro2::Span;
use syn::{spanned::Spanned, Error, ExprIf, Result};

use crate::glsl::Glsl;
use crate::glsl::GlslFragment;
use crate::glsl::GlslLine;

use super::YaslExprFunctionScope;
use super::YaslExprLineScope;
use crate::yasl_block::YaslBlock;

#[derive(Debug)]
pub struct YaslExprIf {
    if_token: syn::token::If,
    cond: Box<YaslExprLineScope>,
    then_branch: Box<YaslBlock>,
    else_branch: Option<(syn::token::Else, Box<YaslExprFunctionScope>)>,
}
impl YaslExprIf {
    pub fn span(&self) -> Span {
        self.if_token.span()
    }
}

impl From<&YaslExprIf> for Glsl {
    fn from(expr: &YaslExprIf) -> Glsl {
        let mut elements = Vec::new();

        elements.push(Glsl::Line(GlslLine {
            span: Some(expr.if_token.span()),
            ends_with_semi: false,
            glsl_string: format!("if({})", Glsl::from(&*expr.cond),),
        }));

        elements.push(Glsl::from(&*expr.then_branch));

        if let Some(expr) = &expr.else_branch {
            let el = Glsl::Line(GlslLine {
                span: Some(expr.0.span()),
                ends_with_semi: false,
                glsl_string: "else".into(),
            });
            let block = (&*expr.1).into();
            let fragment = GlslFragment {
                elements: vec![el, block],
            };
            elements.push(Glsl::Fragment(fragment));
        }

        Glsl::Fragment(GlslFragment { elements })
    }
}

impl TryFrom<ExprIf> for YaslExprIf {
    type Error = Error;
    fn try_from(c: ExprIf) -> Result<Self> {
        let if_token = c.if_token;
        let cond = Box::new((*c.cond).try_into()?);
        let then_branch = Box::new(c.then_branch.try_into()?);

        let else_branch = if let Some((e, expr)) = c.else_branch {
            Some((e, Box::new((*expr).try_into()?)))
        } else {
            None
        };

        Ok(Self {
            if_token,
            cond,
            then_branch,
            else_branch,
        })
    }
}
