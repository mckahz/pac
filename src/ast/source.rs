use super::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: Name,
    pub interface: Vec<Name>,
    pub imports: Vec<Import>,
    pub signatures: Vec<(Name, Type)>,
    pub type_defs: Vec<(Name, TypeDef)>,
    pub defs: Vec<(Name, Expr)>,
}

pub enum Statement {
    Import(Import),
    LetType(Name, TypeDef),
    LetSignature(Name, Type),
    LetValue(Name, Expr),
}

#[derive(Debug, Clone)]
pub enum TypeDef {
    Internal {
        args: Vec<Name>,
        constructors: Vec<Constructor>,
    },
    External(String),
}

#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: Name,
    pub args: Vec<Type>,
}

pub type Type = Located<Type_>;

#[derive(Debug, Clone)]
pub enum Type_ {
    External(String),
    Unit,
    Cons(Name, Vec<Type>),
    Identifier(Name),
    Fn(Box<Type>, Box<Type>),
    Record(HashMap<Name, Type>),
    Tuple(Vec<Type>),
}

pub type Pattern = Located<Pattern_>;

#[derive(Debug, Clone)]
pub enum Pattern_ {
    Wildcard,
    Identifier(Name),
    EmptyList,
    Constructor(Name, Vec<Pattern>),
    Tuple(Vec<Pattern>),
}

pub type Expr = Located<Expr_>;

#[derive(Debug, Clone)]
pub enum Expr_ {
    External(String),
    Let(Pattern, Box<Expr>, Box<Expr>),
    Bind(Pattern, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Ap(Box<Expr>, Box<Expr>),
    Identifier(Name),
    Constructor(Name),
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
    Record(HashMap<Name, Expr>),
    Access(Name, Name),
    List(Vec<Expr>),
    Tuple(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub struct Import {
    pub module: Name,
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
