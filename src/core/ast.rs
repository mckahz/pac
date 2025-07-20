#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub imports: Vec<Import>,
    pub defs: Vec<(String, Expr)>,
}

#[derive(Debug, Clone)]
pub struct Import {
    name : String,
    path : String,
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


#[derive(Debug, Clone)]
pub enum Expr {
    Ap{function: Box<Expr>, arg: Term},
    Op { op: Operator, lhs: Box<Expr>, rhs: Box<Expr> },
    Let{ defs : Vec<(String, Expr)>, body : Box<Expr>},
    LetRec{ defs : Vec<(String, Expr)>, body : Box<Expr>},
    Lambda{ arg: String, body: Box<Expr>},
    Term(Term),
    When{ expr: Box<Expr>, alternatives: Vec<Alternative>},
    String(String),
    List(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub struct Alternative { pub tag: u8, pub args: Vec<String>, pub body: Expr}


#[derive(Debug, Clone)]
pub enum Term {
    Binding(String),
    Num(f64),
    String(String),
    Constructor{tag: u8, arity: u8},
}
