use std::convert::{TryFrom, TryInto};

use syn::{spanned::Spanned, Error, Result};

use syn::Expr;

use crate::glsl::{Glsl, GlslLine};
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

mod expr_if;
use expr_if::YaslExprIf;

mod expr_return;
use expr_return::YaslExprReturn;

use crate::yasl_block::YaslBlock;

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
impl From<&YaslExprLineScope> for Glsl {
    fn from(expr: &YaslExprLineScope) -> Glsl {
        use YaslExprLineScope::*;

        Glsl::Expr(match expr {
            Lit(l) => Glsl::from(l).to_string(),
            Binary(b) => Glsl::from(b).to_string(),
            Call(c) => Glsl::from(c).to_string(),
            Cast(c) => Glsl::from(c).to_string(),
            Ident(i) => Glsl::from(i).to_string(),
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
            _ => Err(Error::new(
                expr.span(),
                format!("Unsuported Action (Function Scope);\n {:#?}", expr),
            )),
        }
    }
}

// #[derive(Debug)]
// pub enum YaslExprIfCondScope {
//     Lit(YaslExprLit),
//     Binary(YaslExprBinary),
//     Call(YaslExprCall),
//     Cast(YaslExprCast),
//     Ident(YaslIdent),
// }
// impl From<&YaslExprIfCondScope> for Glsl {
//     fn from(expr: &YaslExprIfCondScope) -> Glsl {
//         use YaslExprIfCondScope::*;

//         Glsl::Expr(match expr {
//             Lit(l) => Glsl::from(l).to_string(),
//             Binary(b) => Glsl::from(b).to_string(),
//             Call(c) => Glsl::from(c).to_string(),
//             Cast(c) => Glsl::from(c).to_string(),
//             Ident(i) => Glsl::from(i).to_string(),
//         })
//     }
// }
// impl TryFrom<Expr> for YaslExprIfCondScope {
//     type Error = Error;
//     fn try_from(expr: Expr) -> Result<Self> {
//         use YaslExprIfCondScope::*;
//         match expr {
//             Expr::Lit(l) => Ok(Lit(l.try_into()?)),
//             Expr::Binary(b) => Ok(Binary(b.try_into()?)),
//             Expr::Call(c) => Ok(Call(c.try_into()?)),
//             Expr::Cast(c) => Ok(Cast(c.try_into()?)),
//             Expr::Path(p) => Ok(Ident(p.try_into()?)),
//             _ => Err(Error::new(expr.span(), "Unsuported Action (Line Scope)")),
//         }
//     }
// }

/// Scope Used when returning in function
// #[derive(Debug)]
// pub struct YaslExprReturnScope(YaslExprLineScope);

// impl From<&YaslExprReturnScope> for Glsl {
//     fn from(expr: &YaslExprReturnScope) -> Glsl {
//         let line = Glsl::from(&expr.0).to_string();
//         let line = "return ".to_string() + &line;
//         Glsl::Line(GlslLine {
//             span: None,
//             ends_with_semi: true,
//             glsl_string: line,
//         })
//     }
// }

// impl TryFrom<Expr> for YaslExprReturnScope {
//     type Error = Error;
//     fn try_from(expr: Expr) -> Result<Self> {
//         Ok(Self(YaslExprLineScope::try_from(expr)?))
//     }
// }

/// Scope Used Inside Of Functions
#[derive(Debug)]
pub enum YaslExprFunctionScope {
    Call(YaslExprCall),
    Assign(YaslExprAssign),
    AssignOp(YaslExprAssignOp),
    Return(YaslExprReturn),
    If(YaslExprIf),
    Block(YaslBlock),
}

impl From<&YaslExprFunctionScope> for Glsl {
    fn from(expr: &YaslExprFunctionScope) -> Glsl {
        use YaslExprFunctionScope::*;

        let glsl: Glsl = match expr {
            Call(c) => Glsl::Line(GlslLine {
                span: Some(c.span()),
                ends_with_semi: true,
                glsl_string: Glsl::from(c).to_string(),
            }),
            Assign(a) => Glsl::Line(GlslLine {
                span: Some(a.span()),
                ends_with_semi: true,
                glsl_string: Glsl::from(a).to_string(),
            }),
            AssignOp(a) => Glsl::Line(GlslLine {
                span: Some(a.span()),
                ends_with_semi: true,
                glsl_string: Glsl::from(a).to_string(),
            }),
            Return(r) => Glsl::Line(GlslLine {
                span: Some(r.span()),
                ends_with_semi: true,
                glsl_string: Glsl::from(r).to_string(),
            }),
            If(i) => i.into(),
            Block(b) => b.into(),
        };
        glsl
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
            Expr::Return(r) => Ok(Return(r.try_into()?)),
            Expr::If(i) => Ok(If(i.try_into()?)),
            Expr::Block(b) => Ok(Block(b.try_into()?)),
            _ => Err(Error::new(
                expr.span(),
                format!("Unsuported Action (Function Scope);\n {:#?}", expr),
            )),
        }
    }
}
