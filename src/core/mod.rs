use crate::core::ast::{Import, Module};

pub mod ast;

pub fn to_camel_case(ident: &str) -> String {
    ident
        .chars()
        .fold(("".to_owned(), false), |(camel_case, next_word), c| {
            let upper_c = c.to_uppercase().next().unwrap().to_string();
            let lower_c = c.to_string();
            let string_char: &str = if next_word { &upper_c } else { &lower_c };
            match c {
                '_' => (camel_case, true),
                '?' => (camel_case + "Hmm", true),
                _ => (camel_case + string_char, false),
            }
        })
        .0
}

// take the parsed AST and turn it into the core language
pub fn canonicalize(parse_modules: Vec<crate::parse::ast::Module>) -> Vec<ast::Module> {
    fn canonicalize_expression(parse_expr: crate::parse::ast::Expr) -> ast::Expr {
        use crate::parse;
        match parse_expr {
            parse::ast::Expr::External(name) => ast::Expr::Extern(name),
            parse::ast::Expr::Let(pattern, expr, body) => match pattern {
                parse::ast::Pattern::Wildcard => canonicalize_expression(*body),
                parse::ast::Pattern::Identifier(ident) => ast::Expr::Let {
                    defs: vec![(to_camel_case(&ident), canonicalize_expression(*expr))],
                    body: Box::new(canonicalize_expression(*body)),
                },
                parse::ast::Pattern::EmptyList => todo!(),
                parse::ast::Pattern::Cons(pattern, pattern1) => todo!(),
                parse::ast::Pattern::Product(_, patterns) => todo!(),
            },
            parse::ast::Expr::Bind(pattern, expr, expr1) => todo!(),
            parse::ast::Expr::If(expr, expr1, expr2) => ast::Expr::When {
                expr: Box::new(canonicalize_expression(*expr)),
                alternatives: vec![
                    ast::Alternative {
                        tag: 0,
                        args: vec![],
                        body: canonicalize_expression(*expr1),
                    },
                    ast::Alternative {
                        tag: 1,
                        args: vec![],
                        body: canonicalize_expression(*expr2),
                    },
                ],
            },
            parse::ast::Expr::Ap(expr, expr1) => ast::Expr::Ap {
                function: Box::new(canonicalize_expression(*expr)),
                arg: Box::new(canonicalize_expression(*expr1)),
            },
            parse::ast::Expr::Identifier(ident) => ast::Expr::Binding(to_camel_case(&ident)),
            parse::ast::Expr::Lambda(pattern, expr) => match pattern {
                parse::ast::Pattern::Wildcard => ast::Expr::Lambda {
                    arg: "_".to_owned(),
                    body: Box::new(canonicalize_expression(*expr)),
                },
                parse::ast::Pattern::Identifier(ident) => ast::Expr::Lambda {
                    arg: to_camel_case(&ident),
                    body: Box::new(canonicalize_expression(*expr)),
                },
                parse::ast::Pattern::EmptyList => todo!(),
                parse::ast::Pattern::Cons(pattern, pattern1) => todo!(),
                parse::ast::Pattern::Product(_, patterns) => todo!(),
            },
            parse::ast::Expr::BinOp { op, lhs, rhs } => {
                let lhs = Box::new(canonicalize_expression(*lhs));
                let rhs = Box::new(canonicalize_expression(*rhs));

                match op {
                    // TODO: find a point free way to do this
                    parse::ast::Operator::Compose => ast::Expr::Lambda {
                        arg: "x".to_owned(),
                        body: Box::new(ast::Expr::Ap {
                            function: lhs,
                            arg: Box::new(ast::Expr::Ap {
                                function: rhs,
                                arg: Box::new(ast::Expr::Binding("x".to_owned())),
                            }),
                        }),
                    },
                    parse::ast::Operator::ComposeRev => ast::Expr::Lambda {
                        arg: "x".to_owned(),
                        body: Box::new(ast::Expr::Ap {
                            function: rhs,
                            arg: Box::new(ast::Expr::Ap {
                                function: lhs,
                                arg: Box::new(ast::Expr::Binding("x".to_owned())),
                            }),
                        }),
                    },
                    parse::ast::Operator::Pipe => ast::Expr::Ap {
                        function: lhs,
                        arg: rhs,
                    },
                    parse::ast::Operator::PipeRev => ast::Expr::Ap {
                        function: rhs,
                        arg: lhs,
                    },
                    parse::ast::Operator::Cons => ast::Expr::Ap {
                        function: Box::new(ast::Expr::Ap {
                            function: Box::new(ast::Expr::Constructor { tag: 1, arity: 2 }),
                            arg: lhs,
                        }),
                        arg: rhs,
                    },

                    parse::ast::Operator::Or => ast::Expr::Op {
                        op: ast::Operator::Or,
                        lhs,
                        rhs,
                    },
                    parse::ast::Operator::And => ast::Expr::Op {
                        op: ast::Operator::And,
                        lhs,
                        rhs,
                    },
                    parse::ast::Operator::Eq => ast::Expr::Op {
                        op: ast::Operator::Eq,
                        lhs,
                        rhs,
                    },
                    parse::ast::Operator::Neq => ast::Expr::Op {
                        op: ast::Operator::Neq,
                        lhs,
                        rhs,
                    },
                    parse::ast::Operator::LT => ast::Expr::Op {
                        op: ast::Operator::LT,
                        lhs,
                        rhs,
                    },
                    parse::ast::Operator::LTE => ast::Expr::Op {
                        op: ast::Operator::LTE,
                        lhs,
                        rhs,
                    },
                    parse::ast::Operator::GT => ast::Expr::Op {
                        op: ast::Operator::GT,
                        lhs,
                        rhs,
                    },
                    parse::ast::Operator::GTE => ast::Expr::Op {
                        op: ast::Operator::GTE,
                        lhs,
                        rhs,
                    },
                    parse::ast::Operator::Concat => ast::Expr::Op {
                        op: ast::Operator::Concat,
                        lhs,
                        rhs,
                    },
                    parse::ast::Operator::Plus => ast::Expr::Op {
                        op: ast::Operator::Plus,
                        lhs,
                        rhs,
                    },
                    parse::ast::Operator::Minus => ast::Expr::Op {
                        op: ast::Operator::Minus,
                        lhs,
                        rhs,
                    },
                    parse::ast::Operator::Times => ast::Expr::Op {
                        op: ast::Operator::Times,
                        lhs,
                        rhs,
                    },
                    parse::ast::Operator::Divide => ast::Expr::Op {
                        op: ast::Operator::Divide,
                        lhs,
                        rhs,
                    },
                    parse::ast::Operator::Mod => ast::Expr::Op {
                        op: ast::Operator::Mod,
                        lhs,
                        rhs,
                    },
                    parse::ast::Operator::Power => ast::Expr::Op {
                        op: ast::Operator::Power,
                        lhs,
                        rhs,
                    },
                }
            }
            parse::ast::Expr::When(expr, items) => ast::Expr::When {
                expr: Box::new(canonicalize_expression(*expr)),
                alternatives: items
                    .iter()
                    .map(|(pattern, body)| ast::Alternative {
                        tag: 0,
                        args: vec![],
                        body: canonicalize_expression(body.clone()),
                    })
                    .collect(),
            },
            parse::ast::Expr::Unit => todo!(),
            parse::ast::Expr::Bool(_) => todo!(),
            parse::ast::Expr::Nat(nat) => ast::Expr::Num(nat as f64),
            parse::ast::Expr::Int(int) => ast::Expr::Num(int as f64),
            parse::ast::Expr::Float(float) => ast::Expr::Num(float as f64),
            parse::ast::Expr::String(string) => ast::Expr::String(string),
            parse::ast::Expr::Record(hash_map) => todo!(),
            parse::ast::Expr::Access(expr, expr1) => ast::Expr::Extern("get".to_owned()),
            parse::ast::Expr::List(exprs) => ast::Expr::List(
                exprs
                    .iter()
                    .map(|expr| canonicalize_expression(expr.clone()))
                    .collect(),
            ),
            parse::ast::Expr::Constructor(name) => ast::Expr::Constructor {
                tag: todo!("tag"),
                arity: todo!("arity"),
            },
        }
    }

    let mut core_modules = vec![];
    for parse_module in parse_modules.iter() {
        let core_module = Module {
            name: parse_module.name.to_owned(),
            imports: match parse_module.imports {
                None => vec![],
                Some(_) => vec![],
            },
            defs: parse_module
                .defs
                .iter()
                .map(|(binding, body)| {
                    (
                        to_camel_case(&binding),
                        canonicalize_expression(body.clone()),
                    )
                })
                .collect(),
        };
        core_modules.push(core_module);
    }
    core_modules
}
