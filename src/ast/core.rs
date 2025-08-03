use super::*;
use crate::ast::Span;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: Name,
    pub imports: Vec<Import>,
    pub interface: Vec<Name>,
    pub defs: Vec<(Name, Expr)>,
}

#[derive(Debug, Clone)]
pub struct Import {
    pub name: Name,
}

#[derive(Debug, Clone)]
pub enum Operator {
    Or,
    And,

    Eq,
    Neq,
    LT,
    LTE,
    GT,
    GTE,

    Concat,

    Plus,
    Minus,
    Times,
    Divide,
    Mod,
    Power,
}

pub type Expr = Located<Expr_>;

#[derive(Debug, Clone)]
pub enum Expr_ {
    Extern(String),
    Ap {
        function: Box<Expr>,
        arg: Box<Expr>,
    },
    Op {
        op: Operator,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Let {
        defs: Vec<(Name, Expr)>,
        body: Box<Expr>,
    },
    LetRec {
        defs: Vec<(Name, Expr)>,
        body: Box<Expr>,
    },
    Lambda {
        arg: Name,
        body: Box<Expr>,
    },
    If {
        cond: Box<Expr>,
        true_branch: Box<Expr>,
        false_branch: Box<Expr>,
    },
    When {
        expr: Box<Expr>,
        alternatives: Vec<Alternative>,
    },
    Binding(Name),
    Num(f64),
    String(Name),
    List(Vec<Expr>),
    ModuleAccess {
        module: Name,
        member: Name,
    },
    Constructor {
        tag: u8,
        arity: u8,
    },
}

#[derive(Debug, Clone)]
pub struct Alternative {
    pub tag: u8,
    pub args: Vec<Name>,
    pub body: Expr,
}
