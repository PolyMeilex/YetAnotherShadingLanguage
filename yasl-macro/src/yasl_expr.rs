use std::convert::{TryFrom, TryInto};

use syn::{spanned::Spanned, Error, Result};

use syn::Expr;

use crate::convert::{AsGlsl, Glsl, GlslLine};
use crate::yasl_ident::YaslIdent;

mod binary;
use binary::YaslExprBinary;

mod lit;
use lit::YaslExprLit;

mod call;
use call::YaslExprCall;

mod cast;
use cast::YaslExprCast;

mod assign;
use assign::YaslExprAssign;

mod assign_op;
use assign_op::YaslExprAssignOp;

/// Scope used in var init
/// For example `let a = 5 + call();`
#[derive(Debug)]
pub enum YaslExprLineScope {
    Lit(YaslExprLit),
    Binary(YaslExprBinary),
    Call(YaslExprCall),
    Cast(YaslExprCast),
    Ident(YaslIdent),
}
impl AsGlsl for YaslExprLineScope {
    fn as_glsl(&self) -> Glsl {
        use YaslExprLineScope::*;

        Glsl::Expr(match self {
            Lit(l) => l.as_glsl().to_string(),
            Binary(b) => b.as_glsl().to_string(),
            Call(c) => c.as_glsl().to_string(),
            Cast(c) => c.as_glsl().to_string(),
            Ident(i) => i.as_glsl().to_string(),
        })
    }
}

impl TryFrom<Expr> for YaslExprLineScope {
    type Error = Error;
    fn try_from(expr: Expr) -> Result<Self> {
        use YaslExprLineScope::*;
        match expr {
            Expr::Lit(l) => Ok(Lit(l.try_into()?)),
            Expr::Binary(b) => Ok(Binary(b.try_into()?)),
            Expr::Call(c) => Ok(Call(c.try_into()?)),
            Expr::Cast(c) => Ok(Cast(c.try_into()?)),
            Expr::Path(p) => Ok(Ident(p.try_into()?)),
            _ => {
                println!("{:#?}", expr);
                Err(Error::new(expr.span(), "Unsuported Action"))
            }
        }
    }
}

#[derive(Debug)]
pub struct YaslExprReturnScope(YaslExprLineScope);

impl AsGlsl for YaslExprReturnScope {
    fn as_glsl(&self) -> Glsl {
        let line = self.0.as_glsl().to_string();
        let line = "return ".to_string() + &line;
        Glsl::Line(GlslLine {
            span: None,
            ends_with_semi: true,
            glsl_string: line,
        })
    }
}
impl TryFrom<Expr> for YaslExprReturnScope {
    type Error = Error;
    fn try_from(expr: Expr) -> Result<Self> {
        Ok(Self(YaslExprLineScope::try_from(expr)?))
    }
}

#[derive(Debug)]
pub enum YaslExprFunctionScope {
    Call(YaslExprCall),
    Assign(YaslExprAssign),
    AssignOp(YaslExprAssignOp),
}

impl AsGlsl for YaslExprFunctionScope {
    fn as_glsl(&self) -> Glsl {
        use YaslExprFunctionScope::*;

        let (span, glsl) = match self {
            Call(c) => (c.span(), c.as_glsl()),
            Assign(a) => (a.span(), a.as_glsl()),
            AssignOp(a) => (a.span(), a.as_glsl()),
        };

        Glsl::Line(GlslLine {
            span: Some(span),
            ends_with_semi: true,
            glsl_string: glsl.to_string(),
        })
        // match self {
        //     Call(c) => c.as_glsl() + ";\n",
        //     Assign(a) => a.as_glsl(),
        //     AssignOp(a) => a.as_glsl(),
        // }
    }
}

impl TryFrom<Expr> for YaslExprFunctionScope {
    type Error = Error;
    fn try_from(expr: Expr) -> Result<Self> {
        use YaslExprFunctionScope::*;
        match expr {
            Expr::Call(c) => Ok(Call(c.try_into()?)),
            Expr::Assign(a) => Ok(Assign(a.try_into()?)),
            Expr::AssignOp(a) => Ok(AssignOp(a.try_into()?)),
            _ => Err(Error::new(expr.span(), "Unsuported Action")),
        }
    }
}
