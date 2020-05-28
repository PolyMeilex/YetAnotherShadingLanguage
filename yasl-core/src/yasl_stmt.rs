use crate::glsl::Glsl;
use std::convert::{TryFrom, TryInto};
use syn::{Error, Result, Stmt};

use crate::yasl_expr::YaslExprFunctionScope;
use crate::yasl_item::YaslItem;

mod local;
use local::YaslLocal;

#[derive(Debug)]
pub enum YaslStmt {
    Item(YaslItem),
    Expr(YaslExprFunctionScope),
    // ReturnExpr(YaslExprReturnScope),
    Local(YaslLocal),
}

impl From<&YaslStmt> for Glsl {
    fn from(item: &YaslStmt) -> Glsl {
        use YaslStmt::*;
        let inner = match item {
            Item(i) => i.into(),
            Expr(e) => e.into(),
            // ReturnExpr(e) => e.into(),
            Local(l) => l.into(),
        };
        inner
    }
}
impl TryFrom<Stmt> for YaslStmt {
    type Error = Error;
    fn try_from(stmt: Stmt) -> Result<Self> {
        Ok(match stmt {
            Stmt::Item(i) => Self::Item(i.try_into()?),
            Stmt::Expr(e) => Self::Expr(e.try_into()?),
            Stmt::Semi(e, _) => Self::Expr(e.try_into()?),
            Stmt::Local(l) => Self::Local(l.try_into()?),
        })
    }
}
