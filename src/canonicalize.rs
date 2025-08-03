use crate::{
    ast::{core, source},
    util,
};

fn canonicalize_expression(
    parse_expr: source::Expr,
    type_defs: &[(String, source::TypeDef)],
) -> core::Expr {
    match parse_expr {
        source::Expr::External(name) => core::Expr::Extern(name),
        source::Expr::Let(pattern, expr, body) => match pattern {
            source::Pattern::Identifier(ident) => core::Expr::Let {
                defs: vec![(ident, canonicalize_expression(*expr, type_defs))],
                body: Box::new(canonicalize_expression(*body, type_defs)),
            },
            _ => todo!(),
        },
        source::Expr::Bind(pattern, expr, expr1) => todo!(),
        source::Expr::If(cond, t, f) => core::Expr::If {
            cond: Box::new(canonicalize_expression(*cond, type_defs)),
            true_branch: Box::new(canonicalize_expression(*t, type_defs)),
            false_branch: Box::new(canonicalize_expression(*f, type_defs)),
        },
        source::Expr::Ap(expr, expr1) => core::Expr::Ap {
            function: Box::new(canonicalize_expression(*expr, type_defs)),
            arg: Box::new(canonicalize_expression(*expr1, type_defs)),
        },
        source::Expr::Identifier(ident) => core::Expr::Binding(util::to_camel_case(&ident)),
        source::Expr::Lambda(pattern, expr) => match pattern {
            source::Pattern::Identifier(ident) => core::Expr::Lambda {
                arg: ident,
                body: Box::new(canonicalize_expression(*expr, type_defs)),
            },
            source::Pattern::Wildcard => core::Expr::Lambda {
                arg: "__wildcard".to_owned(),
                body: Box::new(canonicalize_expression(*expr, type_defs)),
            },
            _ => todo!(),
        },
        source::Expr::BinOp { op, lhs, rhs } => {
            let lhs = Box::new(canonicalize_expression(*lhs, type_defs));
            let rhs = Box::new(canonicalize_expression(*rhs, type_defs));

            match op {
                // TODO: find a point free way to do this
                source::Operator::Compose => core::Expr::Lambda {
                    arg: "x".to_owned(),
                    body: Box::new(core::Expr::Ap {
                        function: lhs,
                        arg: Box::new(core::Expr::Ap {
                            function: rhs,
                            arg: Box::new(core::Expr::Binding("x".to_owned())),
                        }),
                    }),
                },
                source::Operator::ComposeRev => core::Expr::Lambda {
                    arg: "x".to_owned(),
                    body: Box::new(core::Expr::Ap {
                        function: rhs,
                        arg: Box::new(core::Expr::Ap {
                            function: lhs,
                            arg: Box::new(core::Expr::Binding("x".to_owned())),
                        }),
                    }),
                },
                source::Operator::Pipe => core::Expr::Ap {
                    function: lhs,
                    arg: rhs,
                },
                source::Operator::PipeRev => core::Expr::Ap {
                    function: rhs,
                    arg: lhs,
                },
                source::Operator::Cons => core::Expr::Ap {
                    function: Box::new(core::Expr::Ap {
                        function: Box::new(core::Expr::Constructor { tag: 1, arity: 2 }),
                        arg: lhs,
                    }),
                    arg: rhs,
                },

                source::Operator::Or => core::Expr::Op {
                    op: core::Operator::Or,
                    lhs,
                    rhs,
                },
                source::Operator::And => core::Expr::Op {
                    op: core::Operator::And,
                    lhs,
                    rhs,
                },
                source::Operator::Eq => core::Expr::Op {
                    op: core::Operator::Eq,
                    lhs,
                    rhs,
                },
                source::Operator::Neq => core::Expr::Op {
                    op: core::Operator::Neq,
                    lhs,
                    rhs,
                },
                source::Operator::LT => core::Expr::Op {
                    op: core::Operator::LT,
                    lhs,
                    rhs,
                },
                source::Operator::LTE => core::Expr::Op {
                    op: core::Operator::LTE,
                    lhs,
                    rhs,
                },
                source::Operator::GT => core::Expr::Op {
                    op: core::Operator::GT,
                    lhs,
                    rhs,
                },
                source::Operator::GTE => core::Expr::Op {
                    op: core::Operator::GTE,
                    lhs,
                    rhs,
                },
                source::Operator::Concat => core::Expr::Op {
                    op: core::Operator::Concat,
                    lhs,
                    rhs,
                },
                source::Operator::Plus => core::Expr::Op {
                    op: core::Operator::Plus,
                    lhs,
                    rhs,
                },
                source::Operator::Minus => core::Expr::Op {
                    op: core::Operator::Minus,
                    lhs,
                    rhs,
                },
                source::Operator::Times => core::Expr::Op {
                    op: core::Operator::Times,
                    lhs,
                    rhs,
                },
                source::Operator::Divide => core::Expr::Op {
                    op: core::Operator::Divide,
                    lhs,
                    rhs,
                },
                source::Operator::Mod => core::Expr::Op {
                    op: core::Operator::Mod,
                    lhs,
                    rhs,
                },
                source::Operator::Power => core::Expr::Op {
                    op: core::Operator::Power,
                    lhs,
                    rhs,
                },
            }
        }
        source::Expr::When(expr, items) => core::Expr::When {
            expr: Box::new(canonicalize_expression(*expr, type_defs)),
            alternatives: items
                .into_iter()
                .enumerate()
                .map(|(i, (pattern, body))| match pattern {
                    source::Pattern::Wildcard => core::Alternative {
                        tag: 0,
                        args: vec!["_".to_owned()],
                        body: canonicalize_expression(body, type_defs),
                    },
                    source::Pattern::Identifier(ident) => core::Alternative {
                        tag: 0,
                        args: vec![ident],
                        body: canonicalize_expression(body, type_defs),
                    },
                    source::Pattern::EmptyList => {
                        let (tag, _) = canonicalize_constructor("Empty", type_defs);
                        core::Alternative {
                            tag,
                            args: vec![],
                            body: canonicalize_expression(body, type_defs),
                        }
                    }
                    source::Pattern::Constructor(name, sub_patterns) => {
                        let (tag, arity) = canonicalize_constructor(&name, type_defs);
                        let args = sub_patterns
                            .into_iter()
                            .enumerate()
                            .filter_map(|(i, sub_pattern)| match sub_pattern {
                                source::Pattern::Identifier(ident) => Some(ident.to_owned()),
                                source::Pattern::Wildcard => {
                                    Some("__arg".to_owned() + &i.to_string())
                                }
                                _ => None,
                            })
                            .collect::<Vec<String>>();
                        core::Alternative {
                            tag,
                            args,
                            body: canonicalize_expression(body.clone(), type_defs),
                        }
                    }
                    source::Pattern::Tuple(patterns) => todo!(),
                })
                .collect(),
        },
        source::Expr::Unit => todo!(),
        source::Expr::Bool(_) => todo!(),
        source::Expr::Nat(nat) => core::Expr::Num(nat as f64),
        source::Expr::Int(int) => core::Expr::Num(int as f64),
        source::Expr::Float(float) => core::Expr::Num(float as f64),
        source::Expr::String(string) => core::Expr::String(string),
        source::Expr::Record(hash_map) => todo!(),
        source::Expr::Access(module, member) => core::Expr::ModuleAccess {
            module: module,
            member: member,
        },
        source::Expr::List(exprs) => exprs.into_iter().rev().fold(
            {
                let (tag, arity) = canonicalize_constructor("Empty", type_defs);
                core::Expr::Constructor { tag, arity }
            },
            |list, expr| core::Expr::Ap {
                function: Box::new(core::Expr::Ap {
                    function: {
                        let (tag, arity) = canonicalize_constructor("Cons", type_defs);
                        Box::new(core::Expr::Constructor { tag, arity })
                    },
                    arg: Box::new(canonicalize_expression(expr, type_defs)),
                }),
                arg: Box::new(list),
            },
        ),
        // TODO: represent lists as arrays
        // core::Expr::List(
        //     exprs
        //         .iter()
        //         .map(|expr| canonicalize_expression(expr.clone(), type_defs))
        //         .collect(),
        // ),
        source::Expr::Constructor(name) => {
            let (tag, arity) = canonicalize_constructor(&name, type_defs);
            core::Expr::Constructor { tag, arity }
        }
        source::Expr::Tuple(exprs) => exprs.into_iter().fold(
            core::Expr::Constructor { tag: 0, arity: 0 },
            |function, expr| core::Expr::Ap {
                function: Box::new(function),
                arg: Box::new(canonicalize_expression(expr, type_defs)),
            },
        ),
    }
}

fn canonicalize_constructor(name: &str, type_defs: &[(String, source::TypeDef)]) -> (u8, u8) {
    type_defs
        .iter()
        .filter_map(|(_, type_def)| match type_def {
            source::TypeDef::Internal { constructors, .. } => {
                let i = constructors.iter().position(|c| &c.name == name)?;
                Some((i as u8, constructors.get(i)?.args.len() as u8))
            }
            source::TypeDef::External(_) => None,
        })
        .next()
        .expect("name should resolve due to type checking")
}

pub fn canonicalize_module(
    parse_module: source::Module,
    type_defs: &[(String, source::TypeDef)],
) -> core::Module {
    core::Module {
        name: parse_module.name.to_owned(),
        imports: parse_module
            .imports
            .iter()
            .map(|source::Import { module, .. }| core::Import {
                name: module.clone(),
            })
            .collect(),
        interface: parse_module
            .interface
            .into_iter()
            .map(|export| util::to_camel_case(&export))
            .collect::<Vec<String>>(),
        defs: parse_module
            .defs
            .iter()
            .map(|(binding, body)| {
                (
                    util::to_camel_case(&binding),
                    canonicalize_expression(body.clone(), type_defs),
                )
            })
            .collect(),
    }
}

// take the parsed AST and turn it into the core language
pub fn canonicalize(mut parse_modules: Vec<source::Module>) -> Vec<core::Module> {
    let mut type_defs = vec![];
    for parse_module in parse_modules.iter_mut() {
        type_defs.append(&mut parse_module.type_defs);
    }

    let mut core_modules = vec![];
    for parse_module in parse_modules.into_iter() {
        core_modules.push(canonicalize_module(parse_module, &type_defs));
    }
    core_modules
}
