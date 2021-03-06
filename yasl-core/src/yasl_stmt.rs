use crate::glsl::Glsl;
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
};
use syn::{Error, Result, Stmt};

use crate::yasl_expr::YaslExprFunctionScope;
use crate::{yasl_ident::YaslIdent, yasl_item::YaslItem, yasl_type::YaslType};

mod local;
use local::YaslLocal;

#[derive(Debug)]
pub enum YaslStmt {
    Item(YaslItem),
    Expr(YaslExprFunctionScope),
    // ReturnExpr(YaslExprReturnScope),
    Local(YaslLocal),
}

impl YaslStmt {
    pub fn attempt_type_anotation(&mut self, idents: &HashMap<String, YaslType>) {
        match self {
            YaslStmt::Local(l) => l.attempt_type_anotation(idents),
            YaslStmt::Expr(e) => e.attempt_type_anotation(idents),
            _ => {}
        }
    }
    pub fn update_idents(&mut self) -> Vec<YaslIdent> {
        match self {
            YaslStmt::Local(l) => {
                let mut out = Vec::new();
                if let Some(i) = l.get_ident() {
                    out.push(i);
                }
                out
            }
            _ => vec![],
        }
    }
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
