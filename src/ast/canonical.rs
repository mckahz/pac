use std::collections::{HashMap, HashSet};

use super::*;
use crate::ast::Span;

// MODULE

#[derive(Debug, Clone)]
pub struct Module {
    pub unions: HashMap<Name, Union>,
    pub aliases: HashMap<Name, Alias>,
    pub external_types: HashMap<Name, String>,
    pub definitions: Definitions,
    pub imports: Vec<ModuleName>,
    pub exports: Vec<Export>,
}

#[derive(Debug, Clone)]
pub enum Export {
    Value(Name),
    ClosedType(Name),
    OpenType(Name),
}

#[derive(Debug, Clone)]
pub struct Alias {
    pub variables: Vec<Name>,
    pub other: Type,
}

#[derive(Debug, Clone)]
pub struct Union {
    pub variables: Vec<Name>,
    pub constructors: Vec<Name>,
}

#[derive(Debug, Clone)]
pub enum Definitions {
    None,
    Recursive(Vec<Definition>),
    NonRecursive(Definition),
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub annotation: Annotation,
    pub name: Name,
    pub expr: Expr,
}

// EXPRESSIONS

pub type Expr = Located<Expr_>;

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub enum Expr_ {
    Variable(Qualified<Name>),
    Constructor(Qualified<Constructor>),
    Unit,
    Bool(bool),
    Int(f64),
    Float(f64),
    String(Name),
    List(Vec<Expr>),
    Ap { function: Box<Expr>, arg: Box<Expr> },
    Op { op: Operator, lhs: Box<Expr>, rhs: Box<Expr> },
    Let { name: Name, expr: Box<Expr>, body: Box<Expr> },
    LetRec { defs: Vec<(Name, Expr)>, body: Box<Expr> },
    Lambda { arg: Name, body: Box<Expr> },
    If { cond: Box<Expr>, true_branch: Box<Expr>, false_branch: Box<Expr> },
    When { expr: Box<Expr>, first_alternative: Box<(Pattern, Expr)>, rest_alternatives: Vec<(Pattern, Expr)> },
    Access { record: Box<Expr>, field: Name }, // TODO: add records
}

#[derive(Debug, Clone)]
pub struct Constructor {
    pub tag: u16,
    pub arity: u16,
    pub annotation: Annotation,
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

// TYPES

#[derive(Debug, Clone)]
pub enum Type {
    Variable(Name),
    Identifier(Qualified<Name>),
    Application(Box<Type>, Box<Type>),
    Lambda(Box<Type>, Box<Type>),
    Record(HashMap<Name, Type>),
    Unit,
    Tuple(Box<Type>, Box<Type>, Vec<Type>),
}

impl Type {
    pub fn free_variables(&self) -> HashSet<Name> {
        match self {
            Type::Variable(var) => HashSet::from([var.clone()]),
            Type::Identifier(_) => HashSet::new(),
            Type::Unit => HashSet::new(),
            Type::Record(fields) => fields
                .iter()
                .map(|(_, tipe)| tipe.free_variables())
                .fold(HashSet::new(), |all, one| {
                    all.union(&one).map(|s| s.clone()).collect()
                }),
            Type::Tuple(a, b, rest) => {
                let mut vars = vec![];
                vars.push(a.free_variables());
                vars.push(b.free_variables());
                vars.append(&mut rest.iter().map(|tipe| tipe.free_variables()).collect());
                vars.into_iter().fold(HashSet::new(), |all, one| {
                    all.union(&one).map(|s| s.clone()).collect()
                })
            }
            Type::Application(type_constructor, arg) => type_constructor
                .free_variables()
                .union(&arg.free_variables())
                .map(|v| v.clone())
                .collect(),
            Type::Lambda(arg, ret) => arg
                .free_variables()
                .union(&ret.free_variables())
                .map(|v| v.clone())
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Primative {
    Bool,
    Int,
    String,
}

#[derive(Debug, Clone)]
pub struct Annotation {
    pub quantified: HashSet<Name>,
    pub tipe: Type,
}

impl Annotation {
    pub fn anything() -> Self {
        Self {
            quantified: HashSet::from([String::from("a")]),
            tipe: Type::Variable("a".to_owned()),
        }
    }
}

// PATTERN

pub type Pattern = Located<Pattern_>;

#[derive(Debug, Clone)]
pub enum Pattern_ {
    Wildcard,
    Identifier(Name),
    Constructor(Constructor, Vec<Pattern>),
    Tuple(Vec<Pattern>),
}
