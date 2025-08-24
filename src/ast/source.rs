use super::*;
use std::collections::{HashMap, HashSet};

// MODULE

#[derive(Debug, Clone)]
pub struct Module {
    pub name: ModuleName,
    pub exports: Vec<Export>,
    pub imports: Vec<ModuleName>,
    pub types: HashMap<Name, TypeDefinition>,
    pub values: HashMap<Name, Expr>,
    pub annotations: HashMap<Name, Type>,
}

#[derive(Debug, Clone)]
pub enum Export {
    Value(Name),
    ClosedType(Name),
    OpenType(Name),
}

#[derive(Debug, Clone)]
pub enum TypeDefinition {
    Alias(Alias),
    Union(Union),
    External(String),
}

impl TypeDefinition {
    pub fn uses(&self, name: &Name) -> bool {
        match self {
            TypeDefinition::Alias(alias) => todo!(),
            TypeDefinition::Union(union) => union
                .variants
                .iter()
                .any(|variant| variant.args.iter().any(|arg| arg.inner.uses(name))),
            TypeDefinition::External(_) => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Alias {
    pub variables: Vec<Name>,
    pub other: Type,
}

#[derive(Debug, Clone)]
pub struct Union {
    pub variables: Vec<Name>,
    pub variants: Vec<Constructor>,
}

#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: Name,
    pub args: Vec<Type>,
}

#[derive(Debug, Clone)]
pub struct Value {
    pub signature: Option<Type>,
    pub value: Expr,
}

pub enum Statement {
    Import(ModuleName),
    LetType(Name, TypeDefinition),
    LetSignature(Name, Type),
    LetValue(Name, Expr),
}

// TYPE

pub type Type = Located<Type_>;

#[derive(Debug, Clone)]
pub enum Type_ {
    Unit,
    Constructor(Box<Type>, Box<Type>, Vec<Type>), // Constructor, Argument, Rest of Arguments. If there were no arguments it should be an identifier
    Identifier(Name),
    QualifiedIdentifier(ModuleName, Name),
    Variable(Name),
    Fn(Box<Type>, Box<Type>),
    Record(HashMap<Name, Type>),
    Tuple(Box<Type>, Box<Type>, Vec<Type>),
}

impl Type_ {
    pub fn free_variables(&self) -> HashSet<Name> {
        match self {
            Type_::Variable(var) => HashSet::from([var.clone()]),
            Type_::Constructor(cons, first_arg, args) => {
                args.iter().map(|arg| arg.inner.free_variables()).fold(
                    cons.inner
                        .free_variables()
                        .union(&first_arg.inner.free_variables())
                        .map(|s| s.clone())
                        .collect(),
                    |all, one| all.union(&one).map(|s| s.clone()).collect(),
                )
            }
            Type_::Fn(f, x) => f
                .inner
                .free_variables()
                .union(&x.inner.free_variables())
                .map(|s| s.clone())
                .collect(),
            Type_::Identifier(_) => HashSet::new(),
            Type_::QualifiedIdentifier(_, _) => HashSet::new(),
            Type_::Unit => HashSet::new(),
            Type_::Record(fields) => fields
                .iter()
                .map(|(_, tipe)| tipe.inner.free_variables())
                .fold(HashSet::new(), |all, one| {
                    all.union(&one).map(|s| s.clone()).collect()
                }),
            Type_::Tuple(a, b, rest) => {
                let mut vars = vec![];
                vars.push(a.inner.free_variables());
                vars.push(b.inner.free_variables());
                vars.append(
                    &mut rest
                        .iter()
                        .map(|tipe| tipe.inner.free_variables())
                        .collect(),
                );
                vars.into_iter().fold(HashSet::new(), |all, one| {
                    all.union(&one).map(|s| s.clone()).collect()
                })
            }
        }
    }

    fn uses(&self, name: &Name) -> bool {
        match self {
            Type_::Identifier(arg_name) => arg_name == name,
            Type_::Constructor(cons, arg, rest) => {
                cons.inner.uses(name)
                    || arg.inner.uses(name)
                    || rest.iter().any(|arg| arg.inner.uses(name))
            }
            Type_::Unit => false,
            Type_::QualifiedIdentifier(_, _) => false,
            Type_::Variable(_) => false,
            Type_::Fn(lhs, rhs) => lhs.inner.uses(name) || rhs.inner.uses(name),
            Type_::Record(fields) => fields.iter().any(|(_, tipe)| tipe.inner.uses(name)),
            Type_::Tuple(a, b, rest) => {
                a.inner.uses(name)
                    || b.inner.uses(name)
                    || rest.iter().any(|arg| arg.inner.uses(name))
            }
        }
    }
}

// PATTERN

pub type Pattern = Located<Pattern_>;

#[derive(Debug, Clone)]
pub enum Pattern_ {
    Wildcard,
    Identifier(Name),
    Constructor(Name, Vec<Pattern>),
    Cons(Box<Pattern>, Box<Pattern>),
    Tuple(Vec<Pattern>),
}

// EXPRESSION

pub type Expr = Located<Expr_>;

#[derive(Debug, Clone)]
pub enum Expr_ {
    External(String),
    Let(Pattern, Box<Expr>, Box<Expr>),
    Bind(Pattern, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Ap(Box<Expr>, Box<Expr>),
    Identifier(Name),
    QualifiedIdentifier(ModuleName, Name),
    Constructor(Name),
    QualifiedConstructor(ModuleName, Name),
    Lambda(Pattern, Box<Expr>),
    BinOp {
        op: Operator,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    When(Box<Expr>, Box<(Pattern, Expr)>, Vec<(Pattern, Expr)>),
    Unit,
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Record(HashMap<Name, Expr>),
    List(Vec<Expr>),
    Tuple(Vec<Expr>),
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
