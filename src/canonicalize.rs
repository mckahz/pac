use crate::{
    ast::{self, core, source, Name, Span},
    util,
};

fn canonicalize_expression(
    source_expr: source::Expr,
    type_defs: &[(Name, source::TypeDef)],
) -> core::Expr {
    // TODO: this doesn't generalize to custom subexpressions built in the canonicalization of operator expressions.
    let locate = |expr: core::Expr_| ast::Located {
        region: source_expr.region.clone(),
        inner: expr,
    };
    match source_expr.inner {
        source::Expr_::External(name) => locate(core::Expr_::Extern(name)),
        source::Expr_::Let(pattern, expr, body) => match &pattern.inner {
            source::Pattern_::Identifier(ident) => locate(core::Expr_::Let {
                defs: vec![(ident.to_owned(), canonicalize_expression(*expr, type_defs))],
                body: Box::new(canonicalize_expression(*body, type_defs)),
            }),
            _ => todo!(),
        },
        source::Expr_::Bind(pattern, expr, expr1) => todo!(),
        source::Expr_::If(cond, t, f) => locate(core::Expr_::If {
            cond: Box::new(canonicalize_expression(*cond, type_defs)),
            true_branch: Box::new(canonicalize_expression(*t, type_defs)),
            false_branch: Box::new(canonicalize_expression(*f, type_defs)),
        }),
        source::Expr_::Ap(expr, expr1) => locate(core::Expr_::Ap {
            function: Box::new(canonicalize_expression(*expr, type_defs)),
            arg: Box::new(canonicalize_expression(*expr1, type_defs)),
        }),
        source::Expr_::Identifier(ident) => locate(core::Expr_::Binding(ident)),
        source::Expr_::Lambda(pattern, expr) => match &pattern.inner {
            source::Pattern_::Identifier(ident) => locate(core::Expr_::Lambda {
                arg: ident.to_owned(),
                body: Box::new(canonicalize_expression(*expr, type_defs)),
            }),
            source::Pattern_::Wildcard => locate(core::Expr_::Lambda {
                arg: "__wildcard".to_owned(),
                body: Box::new(canonicalize_expression(*expr, type_defs)),
            }),
            _ => todo!(),
        },
        source::Expr_::BinOp { op, lhs, rhs } => {
            let lhs = Box::new(canonicalize_expression(*lhs, type_defs));
            let rhs = Box::new(canonicalize_expression(*rhs, type_defs));

            match op {
                // TODO: find a point free way to do this
                source::Operator::Compose => locate(core::Expr_::Lambda {
                    arg: "__arg".to_owned(),
                    body: Box::new(locate(core::Expr_::Ap {
                        function: lhs,
                        arg: Box::new(locate(core::Expr_::Ap {
                            function: rhs,
                            arg: Box::new(locate(core::Expr_::Binding("__arg".to_owned()))),
                        })),
                    })),
                }),
                source::Operator::ComposeRev => locate(core::Expr_::Lambda {
                    arg: "__arg".to_owned(),
                    body: (Box::new(locate(core::Expr_::Ap {
                        function: rhs,
                        arg: Box::new(locate(core::Expr_::Ap {
                            function: lhs,
                            arg: Box::new(locate(core::Expr_::Binding("__arg".to_owned()))),
                        })),
                    }))),
                }),
                source::Operator::Pipe => locate(core::Expr_::Ap {
                    function: lhs,
                    arg: rhs,
                }),
                source::Operator::PipeRev => locate(core::Expr_::Ap {
                    function: rhs,
                    arg: lhs,
                }),
                source::Operator::Cons => locate(core::Expr_::Ap {
                    function: Box::new(locate(core::Expr_::Ap {
                        function: Box::new(locate(core::Expr_::Constructor { tag: 1, arity: 2 })),
                        arg: lhs,
                    })),
                    arg: rhs,
                }),

                source::Operator::Or => locate(core::Expr_::Op {
                    op: core::Operator::Or,
                    lhs,
                    rhs,
                }),
                source::Operator::And => locate(core::Expr_::Op {
                    op: core::Operator::And,
                    lhs,
                    rhs,
                }),
                source::Operator::Eq => locate(core::Expr_::Op {
                    op: core::Operator::Eq,
                    lhs,
                    rhs,
                }),
                source::Operator::Neq => locate(core::Expr_::Op {
                    op: core::Operator::Neq,
                    lhs,
                    rhs,
                }),
                source::Operator::LT => locate(core::Expr_::Op {
                    op: core::Operator::LT,
                    lhs,
                    rhs,
                }),
                source::Operator::LTE => locate(core::Expr_::Op {
                    op: core::Operator::LTE,
                    lhs,
                    rhs,
                }),
                source::Operator::GT => locate(core::Expr_::Op {
                    op: core::Operator::GT,
                    lhs,
                    rhs,
                }),
                source::Operator::GTE => locate(core::Expr_::Op {
                    op: core::Operator::GTE,
                    lhs,
                    rhs,
                }),
                source::Operator::Concat => locate(core::Expr_::Op {
                    op: core::Operator::Concat,
                    lhs,
                    rhs,
                }),
                source::Operator::Plus => locate(core::Expr_::Op {
                    op: core::Operator::Plus,
                    lhs,
                    rhs,
                }),
                source::Operator::Minus => locate(core::Expr_::Op {
                    op: core::Operator::Minus,
                    lhs,
                    rhs,
                }),
                source::Operator::Times => locate(core::Expr_::Op {
                    op: core::Operator::Times,
                    lhs,
                    rhs,
                }),
                source::Operator::Divide => locate(core::Expr_::Op {
                    op: core::Operator::Divide,
                    lhs,
                    rhs,
                }),
                source::Operator::Mod => locate(core::Expr_::Op {
                    op: core::Operator::Mod,
                    lhs,
                    rhs,
                }),
                source::Operator::Power => locate(core::Expr_::Op {
                    op: core::Operator::Power,
                    lhs,
                    rhs,
                }),
            }
        }
        source::Expr_::When(expr, items) => locate(core::Expr_::When {
            expr: Box::new(canonicalize_expression(*expr, type_defs)),
            alternatives: items
                .into_iter()
                .enumerate()
                .map(|(i, (pattern, body))| match &pattern.inner {
                    source::Pattern_::Wildcard => core::Alternative {
                        tag: 0,
                        args: vec!["__wildcard".to_owned()],
                        body: canonicalize_expression(body, type_defs),
                    },
                    source::Pattern_::Identifier(ident) => core::Alternative {
                        tag: 0,
                        args: vec![ident.to_owned()],
                        body: canonicalize_expression(body, type_defs),
                    },
                    source::Pattern_::EmptyList => {
                        let (tag, _) = canonicalize_constructor("Empty".to_owned(), type_defs);
                        core::Alternative {
                            tag,
                            args: vec![],
                            body: canonicalize_expression(body, type_defs),
                        }
                    }
                    source::Pattern_::Constructor(name, sub_patterns) => {
                        let (tag, arity) = canonicalize_constructor(name.to_owned(), type_defs);
                        let args = sub_patterns
                            .into_iter()
                            .enumerate()
                            .filter_map(|(i, sub_pattern)| match &sub_pattern.inner {
                                source::Pattern_::Identifier(ident) => Some(ident.to_owned()),
                                source::Pattern_::Wildcard => {
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
                    source::Pattern_::Tuple(patterns) => todo!(),
                })
                .collect(),
        }),
        source::Expr_::Unit => todo!(),
        source::Expr_::Bool(_) => todo!(),
        source::Expr_::Nat(nat) => locate(core::Expr_::Num(nat as f64)),
        source::Expr_::Int(int) => locate(core::Expr_::Num(int as f64)),
        source::Expr_::Float(float) => locate(core::Expr_::Num(float as f64)),
        source::Expr_::String(string) => locate(core::Expr_::String(string)),
        source::Expr_::Record(hash_map) => todo!(),
        source::Expr_::Access(module, member) => locate(core::Expr_::ModuleAccess {
            module: module,
            member: member,
        }),
        source::Expr_::List(exprs) => exprs.into_iter().rev().fold(
            {
                let (tag, arity) = canonicalize_constructor("Empty".to_owned(), type_defs);
                locate(core::Expr_::Constructor { tag, arity })
            },
            |list, expr| {
                locate(core::Expr_::Ap {
                    function: Box::new(locate(core::Expr_::Ap {
                        function: {
                            let (tag, arity) =
                                canonicalize_constructor("Cons".to_owned(), type_defs);
                            Box::new(locate(core::Expr_::Constructor { tag, arity }))
                        },
                        arg: Box::new(canonicalize_expression(expr, type_defs)),
                    })),
                    arg: Box::new(list),
                })
            },
        ),
        // TODO: represent lists as arrays
        // core::Expr_::List(
        //     exprs
        //         .iter()
        //         .map(|expr| canonicalize_expression(expr.clone(), type_defs))
        //         .collect(),
        // ),
        source::Expr_::Constructor(name) => {
            let (tag, arity) = canonicalize_constructor(name, type_defs);
            locate(core::Expr_::Constructor { tag, arity })
        }
        source::Expr_::Tuple(exprs) => exprs.into_iter().fold(
            locate(core::Expr_::Constructor { tag: 0, arity: 0 }),
            |function, expr| {
                locate(core::Expr_::Ap {
                    function: Box::new(function),
                    arg: Box::new(canonicalize_expression(expr, type_defs)),
                })
            },
        ),
    }
}

fn canonicalize_constructor(name: Name, type_defs: &[(Name, source::TypeDef)]) -> (u8, u8) {
    type_defs
        .iter()
        .filter_map(|(_, type_def)| match type_def {
            source::TypeDef::Internal { constructors, .. } => {
                let i = constructors.iter().position(|c| c.name == name)?;
                Some((i as u8, constructors.get(i)?.args.len() as u8))
            }
            source::TypeDef::External(_) => None,
        })
        .next()
        .expect("name should resolve due to type checking")
}

pub fn canonicalize_module(
    source_module: source::Module,
    type_defs: &[(Name, source::TypeDef)],
) -> core::Module {
    core::Module {
        name: source_module.name.to_owned(),
        imports: source_module
            .imports
            .iter()
            .map(|source::Import { module, .. }| core::Import {
                name: module.clone(),
            })
            .collect(),
        interface: source_module.interface.into_iter().collect::<Vec<Name>>(),
        defs: source_module
            .defs
            .into_iter()
            .map(|(binding, body)| (binding, canonicalize_expression(body.clone(), type_defs)))
            .collect::<Vec<(Name, core::Expr)>>(),
    }
}

pub fn canonicalize(mut source_modules: Vec<source::Module>) -> Vec<core::Module> {
    let mut type_defs = vec![];
    for source_module in source_modules.iter_mut() {
        type_defs.append(&mut source_module.type_defs);
    }

    let mut core_modules = vec![];
    for source_module in source_modules.into_iter() {
        core_modules.push(canonicalize_module(source_module, &type_defs));
    }
    core_modules
}
