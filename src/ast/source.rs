use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub interface: Vec<String>,
    pub imports: Vec<Import>,
    pub signatures: Vec<(String, Type)>,
    pub type_defs: Vec<(String, TypeDef)>,
    pub defs: Vec<(String, Expr)>,
}

pub enum Statement {
    Import(Import),
    LetType(String, TypeDef),
    LetSignature(String, Type),
    LetValue(String, Expr),
}

#[derive(Debug, Clone)]
pub enum TypeDef {
    Internal {
        args: Vec<String>,
        constructors: Vec<Constructor>,
    },
    External(String),
}

#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: String,
    pub args: Vec<Type>,
}

#[derive(Debug, Clone)]
pub enum Type {
    External(String),
    Unit,
    Cons(String, Vec<Type>),
    Identifier(String),
    Fn(Box<Type>, Box<Type>),
    Record(HashMap<String, Type>),
    Tuple(Vec<Type>),
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Wildcard,
    Identifier(String),
    EmptyList,
    Constructor(String, Vec<Pattern>),
    Tuple(Vec<Pattern>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    External(String),
    Let(Pattern, Box<Expr>, Box<Expr>),
    Bind(Pattern, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Ap(Box<Expr>, Box<Expr>),
    Identifier(String),
    Constructor(String),
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
    Access(String, String),
    List(Vec<Expr>),
    Tuple(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub struct Import {
    pub module: String,
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
