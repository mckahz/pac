use crate::pretty::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub interface: Vec<String>,
    pub imports: Option<Import>,
    pub signatures: Vec<(String, Expr)>,
    pub typeDefs: Vec<(String, Expr)>,
    pub defs: Vec<(String, Expr)>,
}


#[derive(Debug, Clone)]
pub enum Type {
    External(String),
    Unit,
    Bool,
    Int,
    Nat,
    Float,
    String,
    Cons(Box<Type>, Box<Type>),
    Identifier(String),
    Fn(Box<Type>, Box<Type>),
    Record(HashMap<String, Type>),
    Tuple(Vec<Type>),
    Product(Vec<Type>),
    Sum(Vec<Type>),
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Wildcard,
    Identifier(String),
    EmptyList,
    Cons(Box<Pattern>, Box<Pattern>),
    Product(String, Vec<Pattern>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    External(String),
    Let(Pattern, Box<Expr>, Box<Expr>),
    Bind(Pattern, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Ap(Box<Expr>, Box<Expr>),
    Identifier(String),
    Lambda(Pattern, Box<Expr>),
    BinOp {
        op: Operator,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    When(Box<Expr>, Vec<(Pattern, Expr)>),
    Unit,
    Bool(bool),
    Nat(u32),
    Int(i32),
    Float(f32),
    String(String),
    Record(HashMap<String, Expr>),
    Access(Box<Expr>, Box<Expr>),
    List(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub struct Import {
    pub module: String,
    pub alias: Option<String>,
    pub children: Vec<Import>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Compose,
    ComposeRev,
    Pipe,
    PipeRev,

    Or,
    And,

    Eq,
    Neq,
    LT,
    LTE,
    GT,
    GTE,

    Cons,
    Concat,

    Plus,
    Minus,
    Times,
    Divide,
    Mod,
    Power,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Assoc {
    Left,
    Right,
    None,
}

impl Operator {
    pub fn precedence(&self) -> usize {
        use Operator::*;
        match self {
            Pipe | PipeRev => 0,
            Or => 2,
            And => 3,
            Eq | Neq | LT | LTE | GT | GTE => 4,
            Cons | Concat => 5,
            Plus | Minus => 6,
            Times | Divide | Mod => 7,
            Power => 8,
            Compose | ComposeRev => 9,
        }
    }

    pub fn associativity(&self) -> Assoc {
        use Operator::*;
        match self {
            PipeRev | ComposeRev | Or | And | Concat | Plus | Minus | Times | Divide | Mod => {
                Assoc::Left
            }
            Pipe | Compose | Power | Cons => Assoc::Right,
            Eq | Neq | LT | LTE | GT | GTE => Assoc::None,
        }
    }
}
