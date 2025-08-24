use super::*;
use crate::ast::Span;

#[derive(Debug, Clone)]
pub struct Module {
    pub imports: Vec<ModuleName>,
    pub exports: Vec<Name>,
    pub definitions: Vec<(Name, Expr)>,
}

// HACK: Why does this not have concat? is it because JS uses plus? seems pretty janky ngl
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

    Plus,
    Minus,
    Times,
    Divide,
    Mod,
    Power,
}

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub enum Expr {
    Extern(String),
    Identifier(Qualified<Name>),
    Bool(bool),
    Int(f64),
    Float(f64),
    String(String),
    List(Vec<Expr>),
    Ap { function: Box<Expr>, arg: Box<Expr> },
    Op { op: Operator, lhs: Box<Expr>, rhs: Box<Expr> },
    Let { name: Name, expr: Box<Expr>, body: Box<Expr> },
    LetRec { defs: Vec<(Name, Expr)>, body: Box<Expr> },
    Lambda { arg: Name, body: Box<Expr> },
    If { cond: Box<Expr>, true_branch: Box<Expr>, false_branch: Box<Expr> },
    When { expr: Box<Expr>, decision_tree: Box<DecisionTree<Expr>> },
    Constructor { tag: u16, arity: u16 },
}

#[derive(Debug, Clone)]
pub enum Test {
    Always,
    IsConstructor,
}

#[derive(Debug, Clone)]
pub enum DecisionTree<T> {
    Succeed(T),
    If(Test, T, T),
}
