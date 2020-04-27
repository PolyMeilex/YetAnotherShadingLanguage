use crate::convert::{AsGlsl, Glsl};
use std::convert::{TryFrom, TryInto};
use syn::{Error, Result, Stmt};

use crate::yasl_expr::{YaslExprFunctionScope, YaslExprReturnScope};
use crate::yasl_item::YaslItem;
use crate::yasl_local::YaslLocal;

#[derive(Debug)]
pub enum YaslStmt {
    Item(YaslItem),
    Expr(YaslExprFunctionScope),
    ReturnExpr(YaslExprReturnScope),
    Local(YaslLocal),
}

impl AsGlsl for YaslStmt {
    fn as_glsl(&self) -> Glsl {
        let inner = match self {
            Self::Item(i) => i.as_glsl(),
            Self::Expr(e) => e.as_glsl(),
            Self::ReturnExpr(e) => e.as_glsl(),
            Self::Local(l) => l.as_glsl(),
        };
        inner

        // Glsl::Fragment(format!("\t{}", inner))
    }
}

impl TryFrom<Stmt> for YaslStmt {
    type Error = Error;
    fn try_from(stmt: Stmt) -> Result<Self> {
        Ok(match stmt {
            Stmt::Item(i) => Self::Item(i.try_into()?),
            Stmt::Expr(e) => Self::ReturnExpr(e.try_into()?),
            Stmt::Semi(e, _) => Self::Expr(e.try_into()?),
            Stmt::Local(l) => Self::Local(l.try_into()?),
        })
    }
}
