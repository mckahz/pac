use crate::pretty::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Project {
    pub modules: Vec<Module>,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub interface: Vec<String>,
    pub statements: Vec<Statement>,
}

impl Module {
    pub fn value(&self, name: &str) -> &Expr {
        self.statements
            .iter()
            .find_map(|stmt| match stmt {
                Statement::Let(n, val) if *n == name => Some(val),
                _ => None,
            })
            .unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Import {
    pub module: String,
    pub alias: Option<String>,
    pub children: Vec<Import>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Import(Import),
    Let(String, Expr),
    Bind(String, Expr),
    Type(String, Type),
    Signature(String, Type),
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
pub enum Task {
    PrintLn(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    External(String),
    Crash(String),
    Block(Vec<Statement>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Task(Box<Task>),
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

#[derive(Debug)]
pub struct Frame {
    pub statements: Vec<Statement>,
}

impl Frame {
    fn tipe(&self, name: &str) -> Option<&Type> {
        self.statements.iter().find_map(|stmt| match stmt {
            Statement::Type(n, tipe) if n == name => Some(tipe),
            _ => None,
        })
    }

    fn value(&self, name: &str) -> Option<&Expr> {
        self.statements.iter().find_map(|stmt| match stmt {
            Statement::Let(n, expr) if n == name => Some(expr),
            _ => None,
        })
    }

    fn signature(&self, name: &str) -> Option<&Type> {
        self.statements.iter().find_map(|stmt| match stmt {
            Statement::Signature(n, tipe) if n == name => Some(tipe),
            _ => None,
        })
    }
}

#[derive(Debug)]
pub struct Env {
    pub modules: Vec<Module>,
    pub stack_frames: Vec<Frame>,
}

impl Env {
    pub fn module(&self, name: &str) -> &Module {
        self.modules
            .iter()
            .find(|module| module.name == name)
            .unwrap()
    }

    pub fn tipe(&self, name: &str) -> &Type {
        self.stack_frames
            .iter()
            .find_map(|frame| frame.tipe(name))
            .unwrap()
    }

    pub fn value(&self, name: &str) -> &Expr {
        self.stack_frames
            .iter()
            .find_map(|frame| frame.value(name))
            .unwrap()
    }

    pub fn push_frame(&mut self, frame: Frame) -> () {
        self.stack_frames.push(frame);
    }

    pub fn pop_frame(&mut self) -> Option<Frame> {
        self.stack_frames.pop()
    }
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

        // match self.precedence() {
        //     1 | 6 | 7 => Assoc::Left,
        //     0 | 2 | 3 | 5 | 8 | 9 => Assoc::Right,
        //     4 => Assoc::None,
        //     _ => panic!("fuck you"),
        // }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Assoc {
    Left,
    Right,
    None,
}

impl Expr {
    pub fn optimise(self) -> Self {
        match self {
            Expr::Block(statements, expr) if statements.is_empty() => *expr,
            Expr::External(_)
            | Expr::Crash(_)
            | Expr::Block(_, _)
            | Expr::Task(_)
            | Expr::Identifier(_)
            | Expr::Unit
            | Expr::Bool(_)
            | Expr::Nat(_)
            | Expr::Int(_)
            | Expr::Float(_)
            | Expr::String(_)
            | Expr::Access(_, _) => self,

            Expr::If(cond, t, f) => Expr::If(
                Box::new(cond.optimise()),
                Box::new(t.optimise()),
                Box::new(f.optimise()),
            ),
            Expr::Ap(f, x) => Expr::Ap(Box::new(f.optimise()), Box::new(x.optimise())),
            Expr::Lambda(args, body) => Expr::Lambda(args, Box::new(body.optimise())),
            Expr::BinOp { op, lhs, rhs } => Expr::BinOp {
                op,
                lhs: Box::new(lhs.optimise()),
                rhs: Box::new(rhs.optimise()),
            },
            Expr::When(expr, branches) => Expr::When(
                Box::new(expr.optimise()),
                branches
                    .into_iter()
                    .map(|(pattern, body)| (pattern, body.optimise()))
                    .collect(),
            ),
            Expr::Record(_) => self,
            Expr::List(_) => self,
        }
    }

    pub fn matches(&self, pattern: Pattern) -> bool {
        match (self, pattern) {
            (Expr::External(_), _)
            | (Expr::Crash(_), _)
            | (Expr::Block(_, _), _)
            | (Expr::If(_, _, _), _)
            | (Expr::Task(_), _)
            | (Expr::Ap(_, _), _)
            | (Expr::Identifier(_), _)
            | (Expr::Lambda(_, _), _)
            | (Expr::BinOp { .. }, _)
            | (Expr::When(_, _), _) => false,

            (_, Pattern::Wildcard) => true,
            (_, Pattern::Identifier(_)) => true,
            (Expr::List(list), Pattern::EmptyList) if list.is_empty() => true,
            (_, Pattern::EmptyList) => false,
            (Expr::List(list), Pattern::Cons(_, _)) if !list.is_empty() => true,
            (_, Pattern::Cons(_, _)) => false,

            // (Expr::Unit, Pattern::Product(_, _)) => todo!(),
            // (Expr::Bool(_), Pattern::Product(_, _)) => todo!(),
            // (Expr::Nat(_), Pattern::Product(_, _)) => todo!(),
            // (Expr::Int(_), Pattern::Product(_, _)) => todo!(),
            // (Expr::Float(_), Pattern::Product(_, _)) => todo!(),
            // (Expr::String(_), Pattern::Product(_, _)) => todo!(),
            // (Expr::Record(_), Pattern::Product(_, _)) => todo!(),
            // (Expr::Access(_, _), Pattern::Product(_, _)) => todo!(),
            // (Expr::List(_), Pattern::Product(_, _)) => todo!(),
            _ => todo!(),
        }
    }

    pub fn eq(&self, other: &Self) -> Self {
        use Expr::*;
        Bool(match (self, other) {
            (Nat(a1), Nat(a2)) => a1 == a2,
            (Int(a1), Int(a2)) => a1 == a2,
            _ => false,
        })
    }

    pub fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (Expr::Nat(a1), Expr::Nat(a2)) => Expr::Nat(a1 + a2),
            (Expr::Int(n1), Expr::Int(n2)) => Expr::Int(n1 + n2),
            (Expr::Float(x1), Expr::Float(x2)) => Expr::Float(x1 + x2),
            _ => panic!(
                "cannot add {} to {}",
                &self.pretty_print(),
                &other.pretty_print()
            ),
        }
    }

    pub fn minus(&self, other: &Self) -> Self {
        match (self, other) {
            (Expr::Nat(a1), Expr::Nat(a2)) => Expr::Nat(a1 - a2),
            (Expr::Int(n1), Expr::Int(n2)) => Expr::Int(n1 - n2),
            (Expr::Float(x1), Expr::Float(x2)) => Expr::Float(x1 - x2),
            _ => panic!(
                "cannot add {} to {}",
                &self.pretty_print(),
                &other.pretty_print()
            ),
        }
    }

    pub fn mul(&self, other: &Self) -> Self {
        match (self, other) {
            (Expr::Nat(a1), Expr::Nat(a2)) => Expr::Nat(a1 * a2),
            (Expr::Int(n1), Expr::Int(n2)) => Expr::Int(n1 * n2),
            (Expr::Float(x1), Expr::Float(x2)) => Expr::Float(x1 * x2),
            _ => panic!(
                "cannot add {} to {}",
                &self.pretty_print(),
                &other.pretty_print()
            ),
        }
    }

    pub fn div(&self, other: &Self) -> Self {
        match (self, other) {
            (Expr::Nat(a1), Expr::Nat(a2)) => Expr::Nat(a1 / a2),
            (Expr::Int(n1), Expr::Int(n2)) => Expr::Int(n1 / n2),
            (Expr::Float(x1), Expr::Float(x2)) => Expr::Float(x1 / x2),
            _ => panic!(
                "cannot add {} to {}",
                &self.pretty_print(),
                &other.pretty_print()
            ),
        }
    }

    pub fn modulo(&self, other: &Self) -> Self {
        match (self, other) {
            (Expr::Nat(a1), Expr::Nat(a2)) => Expr::Nat(a1 % a2),
            (Expr::Int(n1), Expr::Int(n2)) => Expr::Int(n1 % n2),
            //(Expr::Float(x1), Expr::Float(x2)) => Expr::Float(x1 / x2),
            _ => panic!(
                "cannot add {} to {}",
                &self.pretty_print(),
                &other.pretty_print()
            ),
        }
    }

    pub fn pow(&self, other: &Self) -> Self {
        match (self, other) {
            (Expr::Nat(a1), Expr::Nat(a2)) => Expr::Nat(a1 ^ a2),
            (Expr::Int(n1), Expr::Int(n2)) => Expr::Int(n1 ^ n2),
            (Expr::Float(x1), Expr::Float(x2)) => Expr::Float(x1.powf(*x2)),
            _ => panic!(
                "cannot add {} to {}",
                &self.pretty_print(),
                &other.pretty_print()
            ),
        }
    }

    pub fn or(&self, other: &Self) -> Self {
        Expr::Bool(match (self, other) {
            (Expr::Bool(b1), Expr::Bool(b2)) => *b1 || *b2,
            _ => panic!(""),
        })
    }

    pub fn and(&self, other: &Self) -> Self {
        Expr::Bool(match (self, other) {
            (Expr::Bool(b1), Expr::Bool(b2)) => *b1 && *b2,
            _ => panic!(""),
        })
    }

    pub fn not(&self) -> Self {
        Expr::Bool(match self {
            Expr::Bool(b) => !b,
            _ => panic!(),
        })
    }

    pub fn neq(&self, other: &Self) -> Self {
        Expr::eq(self, other).not()
    }

    pub fn lt(&self, other: &Self) -> Self {
        Expr::Bool(match (self, other) {
            (Expr::Nat(n1), Expr::Nat(n2)) => n1 < n2,
            (Expr::Int(n1), Expr::Int(n2)) => n1 < n2,
            (Expr::Float(n1), Expr::Float(n2)) => n1 < n2,
            _ => panic!()
        })
    }

    pub fn lte(&self, other: &Self) -> Self {
        Expr::gt(self, other).not()
    }

    pub fn gt(&self, other: &Self) -> Self {
        Expr::lt(other, self).not()
    }

    pub fn gte(&self, other: &Self) -> Self {
        Expr::lt(self, other).not()
    }

    pub fn concat(&self, other: &Self) -> Self {
        match (self, other) {
            (Expr::String(s1), Expr::String(s2)) => Expr::String(s1.clone() + &s2.clone()),
            (Expr::List(l1), Expr::List(l2)) => {
                let mut list = l1.clone();
                list.append(&mut l2.clone());
                Expr::List(list)
            }
            _ => panic!(),
        }
    }

    fn substitute(self, expr: Expr, identifier: String) -> Self {
        match self {
            Expr::External(_)
            | Expr::Crash(_)
            | Expr::Access(_, _)
            | Expr::Unit
            | Expr::Bool(_)
            | Expr::Nat(_)
            | Expr::Int(_)
            | Expr::Float(_)
            | Expr::String(_) => self,
            Expr::Task(task) => Expr::Task(match *task {
                Task::PrintLn(line) => Box::new(Task::PrintLn(line.substitute(expr, identifier))),
            }),
            Expr::Block(statements, body) => {
                let statements = statements
                    .into_iter()
                    .map(|stmt| match stmt {
                        Statement::Import(_)
                        | Statement::Type(_, _)
                        | Statement::Signature(_, _) => stmt,
                        Statement::Let(pat, val) => {
                            Statement::Let(pat, val.substitute(expr.clone(), identifier.clone()))
                        }
                        Statement::Bind(name, val) => {
                            Statement::Bind(name, val.substitute(expr.clone(), identifier.clone()))
                        }
                    })
                    .collect();
                let body = Box::new(body.substitute(expr, identifier));
                Expr::Block(statements, body)
            }
            Expr::If(cond, t, f) => Expr::If(
                Box::new(cond.substitute(expr.clone(), identifier.clone())),
                Box::new(t.substitute(expr.clone(), identifier.clone())),
                Box::new(f.substitute(expr.clone(), identifier.clone())),
            ),
            Expr::Ap(f, x) => Expr::Ap(
                Box::new(f.substitute(expr.clone(), identifier.clone())),
                Box::new(x.substitute(expr, identifier)),
            ),
            Expr::Identifier(ident) if ident == identifier => expr,
            Expr::Identifier(_) => self,
            Expr::Lambda(pat, body) => match pat.clone() {
                Pattern::Identifier(string_arg) if string_arg == identifier => {
                    Expr::Lambda(pat, body)
                }
                Pattern::Identifier(_) => {
                    Expr::Lambda(pat, Box::new(body.substitute(expr, identifier)))
                }
                _ => todo!(),
            },
            Expr::BinOp { op, lhs, rhs } => Expr::BinOp {
                op,
                lhs: Box::new(lhs.substitute(expr.clone(), identifier.clone())),
                rhs: Box::new(rhs.substitute(expr, identifier)),
            },
            Expr::When(operand, branches) => {
                let branches: Vec<(Pattern, Expr)> = branches
                    .into_iter()
                    .map(|(pat, branch)| (pat, branch.substitute(expr.clone(), identifier.clone())))
                    .collect();
                Expr::When(Box::new(operand.substitute(expr, identifier)), branches)
            }
            Expr::Record(_) => todo!(),
            Expr::List(list) => Expr::List(
                list.into_iter()
                    .map(|element| element.substitute(expr.clone(), identifier.clone()))
                    .collect::<Vec<Expr>>(),
            ),
        }
    }

    pub fn eval(self, env: &mut Env) -> Self {
        match self {
            Expr::Nat(_)
            | Expr::Int(_)
            | Expr::Float(_)
            | Expr::String(_)
            | Expr::Unit
            | Expr::Lambda(_, _) => self,
            Expr::List(list) => Expr::List(list.into_iter().map(|expr| expr.eval(env)).collect()),
            Expr::Ap(f, x) => {
                let x = x.eval(env);
                match f.clone().eval(env) {
                    Expr::Lambda(arg, body) => match arg {
                        Pattern::Identifier(i) => body.substitute(x.eval(env), i),
                        Pattern::Wildcard => *body,
                        Pattern::EmptyList => *body,
                        Pattern::Cons(first, rest) => match (*first, *rest, x) {
                            (
                                Pattern::Identifier(first),
                                Pattern::Identifier(rest),
                                Expr::List(list),
                            ) => body
                                .substitute(list[0].clone(), first)
                                .substitute(Expr::List(list[1..].to_vec()), rest),
                            _ => todo!(),
                        },
                        Pattern::Product(_, _) => todo!(),
                    },
                    _ => panic!("cannot apply {} to {}", f.pretty_print(), x.pretty_print()),
                }
                .eval(env)
            } //f.substitute(x),
            Expr::BinOp { op, lhs, rhs } => {
                let lhs = Box::new(lhs.eval(env));
                let rhs = Box::new(rhs.eval(env));
                match op {
                    // TODO: Make this not just a constant pattern
                    Operator::Compose => Expr::Lambda(
                        Pattern::Identifier("haaaaa".to_string()),
                        Box::new(Expr::Ap(
                            lhs,
                            Box::new(Expr::Ap(
                                rhs,
                                Box::new(Expr::Identifier("haaaaa".to_string())),
                            )),
                        )),
                    ),
                    Operator::ComposeRev => Expr::Lambda(
                        Pattern::Identifier("haaaaa".to_string()),
                        Box::new(Expr::Ap(
                            rhs,
                            Box::new(Expr::Ap(
                                lhs,
                                Box::new(Expr::Identifier("haaaaa".to_string())),
                            )),
                        )),
                    ),
                    Operator::Pipe => Expr::Ap(lhs, rhs).eval(env),
                    Operator::PipeRev => Expr::Ap(rhs, lhs).eval(env),
                    Operator::Or => Expr::or(&lhs, &rhs),
                    Operator::And => Expr::and(&lhs, &rhs),
                    Operator::Eq => Expr::eq(&lhs, &rhs),
                    Operator::Neq => Expr::neq(&lhs, &rhs),
                    Operator::LT => Expr::lt(&lhs, &rhs),
                    Operator::LTE => Expr::lte(&lhs, &rhs),
                    Operator::GT => Expr::lte(&lhs, &rhs),
                    Operator::GTE => Expr::gte(&lhs, &rhs),
                    Operator::Concat => Expr::concat(&lhs, &rhs),
                    Operator::Plus => Expr::add(&lhs, &rhs),
                    Operator::Minus => Expr::minus(&lhs, &rhs),
                    Operator::Times => Expr::mul(&lhs, &rhs),
                    Operator::Divide => Expr::div(&lhs, &rhs),
                    Operator::Mod => Expr::modulo(&lhs, &rhs),
                    Operator::Power => Expr::pow(&lhs, &rhs),
                    Operator::Cons => match *rhs {
                        Expr::List(mut list) => {
                            list.reverse();
                            list.push(*lhs);
                            list.reverse();
                            Expr::List(list)
                        }
                        _ => panic!("type error"),
                    },
                }
            }
            Expr::Identifier(ident) => env.value(&ident).clone().eval(env),
            Expr::Record(_) => todo!(),
            Expr::Access(namespace, member) => match (*namespace, *member) {
                (Expr::Identifier(namespace), Expr::Identifier(member)) => {
                    env.module(&namespace).value(&member).clone()
                }
                _ => panic!(""),
            },
            Expr::When(expr, branches) => {
                let expr = expr.clone().eval(env);
                match expr {
                    Expr::External(_)
                    | Expr::Crash(_)
                    | Expr::Block(_, _)
                    | Expr::If(_, _, _)
                    | Expr::Task(_)
                    | Expr::Ap(_, _)
                    | Expr::Identifier(_)
                    | Expr::Lambda(_, _)
                    | Expr::BinOp { .. }
                    | Expr::Float(_)
                    | Expr::Access(_, _)
                    | Expr::When(_, _) => {
                        panic!("Cannot pattern match on {}", expr.eval(env).pretty_print())
                    }
                    Expr::Unit
                    | Expr::Bool(_)
                    | Expr::Nat(_)
                    | Expr::Int(_)
                    | Expr::String(_)
                    | Expr::Record(_)
                    | Expr::List(_) => branches
                        .into_iter()
                        .find_map(|(pattern, body)| {
                            if expr.clone().eval(env).matches(pattern.clone()) {
                                Some(Expr::Ap(
                                    Box::new(Expr::Lambda(pattern, Box::new(body))),
                                    Box::new(expr.clone()),
                                ))
                            } else {
                                None
                            }
                        })
                        .unwrap()
                        .eval(env),
                }
            }
            Expr::Task(_) => self,
            Expr::Bool(_) => self,
            Expr::If(cond, then_branch, else_branch) => match cond.eval(env) {
                Expr::Bool(true) => then_branch.eval(env),
                _ => else_branch.eval(env),
            },
            Expr::Crash(msg) => {
                println!("{}", msg);
                panic!("crash expression reached")
            }
            Expr::Block(statements, value) => {
                env.push_frame(Frame { statements });
                let ret = value.eval(env);
                env.pop_frame();
                ret
            }
            Expr::External(name) => match &*name {
                "println" => Expr::Lambda(
                    Pattern::Identifier("string_to_print".to_string()),
                    Box::new(Expr::Task(Box::new(Task::PrintLn(Expr::Identifier(
                        "string_to_print".to_string(),
                    ))))),
                ),
                _ => {
                    panic!("");
                }
            },
        }
    }

    pub fn execute(&self) -> () {
        let task = match self {
            Expr::Task(task) => task,
            _ => {
                panic!("expression is not a task:\n\n{}", self.pretty_print());
            }
        };
        match *task.clone() {
            Task::PrintLn(expr) => println!("{}", expr.pretty_print()),
        }
    }
}
