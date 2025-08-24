use std::collections::HashMap;

use crate::ast::{canonical, optimized, ModuleName};

pub fn optimize(
    modules: &HashMap<ModuleName, canonical::Module>,
) -> HashMap<ModuleName, optimized::Module> {
    let mut optimized = HashMap::new();
    for (name, module) in modules {
        optimized.insert(
            name.clone(),
            optimized::Module {
                imports: module.imports.clone(),
                exports: module
                    .exports
                    .iter()
                    .map(|export| {
                        (match export {
                            canonical::Export::Value(name) => name,
                            canonical::Export::ClosedType(name) => name,
                            canonical::Export::OpenType(name) => name,
                        })
                        .clone()
                    })
                    .collect(),
                definitions: match &module.definitions {
                    canonical::Definitions::None => vec![],
                    canonical::Definitions::Recursive(definitions) => definitions
                        .iter()
                        .map(|definition| {
                            (definition.name.clone(), expression(definition.expr.clone()))
                        })
                        .collect(),
                    canonical::Definitions::NonRecursive(definition) => todo!(),
                },
            },
        );
    }
    optimized
}

fn expression(canonical_expr: canonical::Expr) -> optimized::Expr {
    match canonical_expr.inner {
        canonical::Expr_::Variable(qualified) => optimized::Expr::Identifier(qualified),
        canonical::Expr_::Constructor(qualified) => {
            let constructor = qualified.get();
            optimized::Expr::Constructor {
                tag: constructor.tag,
                arity: constructor.arity,
            }
        }
        canonical::Expr_::Unit => optimized::Expr::Constructor { tag: 0, arity: 0 },
        canonical::Expr_::Bool(bool) => optimized::Expr::Bool(bool),
        canonical::Expr_::Int(int) => optimized::Expr::Int(int),
        canonical::Expr_::Float(float) => optimized::Expr::Float(float),
        canonical::Expr_::String(string) => optimized::Expr::String(string),
        canonical::Expr_::List(list) => optimized::Expr::List(
            list.into_iter()
                .map(|element| expression(element))
                .collect(),
        ),
        canonical::Expr_::Ap { function, arg } => optimized::Expr::Ap {
            function: Box::new(expression(*function)),
            arg: Box::new(expression(*arg)),
        },
        canonical::Expr_::Op { op, lhs, rhs } => optimized::Expr::Op {
            op: operator(op),
            lhs: Box::new(expression(*lhs)),
            rhs: Box::new(expression(*rhs)),
        },
        canonical::Expr_::Let { name, expr, body } => optimized::Expr::Let {
            name,
            expr: Box::new(expression(*expr)),
            body: Box::new(expression(*body)),
        },
        canonical::Expr_::LetRec { defs, body } => optimized::Expr::LetRec {
            defs: defs
                .into_iter()
                .map(|(name, def)| (name, expression(def)))
                .collect(),
            body: Box::new(expression(*body)),
        },
        canonical::Expr_::Lambda { arg, body } => optimized::Expr::Lambda {
            arg,
            body: Box::new(expression(*body)),
        },
        canonical::Expr_::If {
            cond,
            true_branch,
            false_branch,
        } => optimized::Expr::If {
            cond: Box::new(expression(*cond)),
            true_branch: Box::new(expression(*true_branch)),
            false_branch: Box::new(expression(*false_branch)),
        },
        canonical::Expr_::When {
            expr,
            first_alternative,
            rest_alternatives,
        } => optimized::Expr::When {
            expr: Box::new(expression(*expr)),
            decision_tree: todo!("decision trees"),
        },
        canonical::Expr_::Access { record, field } => todo!(),
    }
}

fn operator(op: canonical::Operator) -> optimized::Operator {
    match op {
        canonical::Operator::Or => optimized::Operator::Or,
        canonical::Operator::And => optimized::Operator::And,
        canonical::Operator::Eq => optimized::Operator::Eq,
        canonical::Operator::Neq => optimized::Operator::Neq,
        canonical::Operator::LT => optimized::Operator::LT,
        canonical::Operator::LTE => optimized::Operator::LTE,
        canonical::Operator::GT => optimized::Operator::GT,
        canonical::Operator::GTE => optimized::Operator::GTE,
        canonical::Operator::Concat => optimized::Operator::Plus,
        canonical::Operator::Plus => optimized::Operator::Plus,
        canonical::Operator::Minus => optimized::Operator::Minus,
        canonical::Operator::Times => optimized::Operator::Times,
        canonical::Operator::Divide => optimized::Operator::Divide,
        canonical::Operator::Mod => optimized::Operator::Mod,
        canonical::Operator::Power => optimized::Operator::Power,
    }
}
