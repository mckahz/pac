#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub imports: Vec<Import>,
    pub interface: Vec<String>,
    pub defs: Vec<(String, Expr)>,
}

#[derive(Debug, Clone)]
pub struct Import {
    pub name: String,
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
        defs: Vec<(String, Expr)>,
        body: Box<Expr>,
    },
    LetRec {
        defs: Vec<(String, Expr)>,
        body: Box<Expr>,
    },
    Lambda {
        arg: String,
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
    Binding(String),
    Num(f64),
    String(String),
    List(Vec<Expr>),
    ModuleAccess {
        module: String,
        member: String,
    },
    Constructor {
        tag: u8,
        arity: u8,
    },
}

#[derive(Debug, Clone)]
pub struct Alternative {
    pub tag: u8,
    pub args: Vec<String>,
    pub body: Expr,
}
