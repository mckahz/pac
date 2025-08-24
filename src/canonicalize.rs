use std::collections::{HashMap, HashSet};

use crate::{
    ast::{
        self,
        canonical::{self, Annotation, Definition, Union},
        source, Located, ModuleName, Name, Qualified, Span,
    },
    util,
};

// resolve variable names, cycle detection

pub fn canonicalize(source_modules: Vec<source::Module>) -> HashMap<ModuleName, canonical::Module> {
    let mut uncanonicalized_dependencies: HashMap<ModuleName, HashSet<ModuleName>> = source_modules
        .iter()
        .map(|module| {
            (
                module.name.clone(),
                module
                    .imports
                    .clone()
                    .into_iter()
                    .collect::<HashSet<ModuleName>>(),
            )
        })
        .collect::<HashMap<ModuleName, HashSet<ModuleName>>>();

    let mut env = Environment::new();

    // TODO: do the cycle detection before the loop, since a cycle would result in a hanging loop
    while !uncanonicalized_dependencies.is_empty() {
        let target_module: ModuleName = uncanonicalized_dependencies
            .iter()
            .filter_map(|(name, deps)| if deps.is_empty() { Some(name) } else { None })
            .next()
            .unwrap()
            .clone();

        let source_module = source_modules
            .iter()
            .find(|m| m.name == target_module)
            .unwrap();

        for (name, tipe) in source_module.types.iter() {
            let push_type = |env: &mut Environment| match tipe {
                source::TypeDefinition::Alias(alias) => {
                    env.types
                        .insert(name.clone(), TypeDefinition::Alias(env.alias(alias)));
                }
                source::TypeDefinition::Union(union) => {
                    let (union, constructors) = env.union_artifacts(name.clone(), union);
                    env.types.insert(name.clone(), TypeDefinition::Union(union));
                    for (name, constructor) in constructors.into_iter() {
                        env.constructors.insert(name.clone(), constructor);
                    }
                }
                source::TypeDefinition::External(name) => {
                    env.types
                        .insert(name.clone(), TypeDefinition::External(name.clone()));
                }
            };

            if tipe.uses(name)
            /* is recursive */
            {
                env.recursive_types.insert(name.clone());
                push_type(&mut env);
                env.recursive_types.remove(name);
            } else {
                push_type(&mut env);
            }
        }

        // TODO: cycle detection to split SCC and non-recursive definitions
        let mut defs = vec![];
        for (name, tipe) in source_module.annotations.iter() {
            let expr = source_module.values.get(name).unwrap().clone();
            defs.push(canonical::Definition {
                annotation: canonical::Annotation {
                    quantified: tipe.inner.free_variables(),
                    tipe: env.tipe(tipe),
                },
                name: name.clone(),
                expr: env.expression(expr),
            });
        }

        let mut unions = HashMap::new();
        let mut aliases = HashMap::new();
        let mut external_types = HashMap::new();
        for (name, tipe) in env.types.clone().into_iter() {
            match tipe {
                TypeDefinition::Alias(alias) => {
                    aliases.insert(name, alias);
                }
                TypeDefinition::Union(union) => {
                    unions.insert(name, union);
                }
                TypeDefinition::External(internal_name) => {
                    external_types.insert(name, internal_name);
                }
            };
        }

        let module = canonical::Module {
            unions,
            aliases,
            external_types,
            definitions: canonical::Definitions::Recursive(defs),
            imports: source_module.imports.clone(),
            exports: source_module
                .exports
                .iter()
                .map(|export| match export.clone() {
                    source::Export::Value(name) => canonical::Export::Value(name),
                    source::Export::ClosedType(name) => canonical::Export::ClosedType(name),
                    source::Export::OpenType(name) => canonical::Export::OpenType(name),
                })
                .collect(),
        };
        let module_name = source_module.name.clone();

        env.modules.insert(module_name.clone(), module);

        // RESET ENVIRONMENT FOR NEXT MODULE
        env.qualified_types.insert(
            module_name.clone(),
            std::mem::replace(&mut env.types, HashMap::new()),
        );
        env.qualified_constructors.insert(
            module_name.clone(),
            std::mem::replace(&mut env.constructors, HashMap::new()),
        );
        env.qualified_variables.insert(
            module_name.clone(),
            std::mem::replace(&mut env.variables, HashSet::new()),
        );

        uncanonicalized_dependencies.remove(&target_module);
        uncanonicalized_dependencies
            .iter_mut()
            .for_each(|(name, mut deps)| {
                deps.remove(&target_module);
            })
    }

    env.modules
}

#[derive(Debug)]
struct Environment {
    modules: HashMap<ModuleName, canonical::Module>,
    recursive_types: HashSet<Name>,
    types: HashMap<Name, TypeDefinition>,
    constructors: HashMap<Name, canonical::Constructor>,
    variables: HashSet<Name>,
    qualified_types: HashMap<ModuleName, HashMap<Name, TypeDefinition>>,
    qualified_constructors: HashMap<ModuleName, HashMap<Name, canonical::Constructor>>,
    qualified_variables: HashMap<ModuleName, HashSet<Name>>,
}

#[derive(Debug, Clone)]
enum TypeDefinition {
    Alias(canonical::Alias),
    Union(canonical::Union),
    External(Name),
}

impl Environment {
    fn new() -> Self {
        Self {
            modules: HashMap::new(),
            recursive_types: HashSet::new(),
            types: HashMap::new(),
            constructors: HashMap::new(),
            variables: HashSet::new(),
            qualified_types: HashMap::new(),
            qualified_constructors: HashMap::new(),
            qualified_variables: HashMap::new(),
        }
    }

    fn qualify_type(&self, name: Name) -> crate::ast::Qualified<Name> {
        if self.recursive_types.contains(&name) || self.types.contains_key(&name) {
            return Qualified::Local(name);
        }

        for (module_name, types) in self.qualified_types.iter() {
            if types.contains_key(&name) {
                return crate::ast::Qualified::Foreign {
                    module: module_name.clone(),
                    member: name,
                };
            }
        }

        return todo!("Could not qualify {}", name);
    }

    fn tipe(&self, tipe: &source::Type) -> canonical::Type {
        match &tipe.inner {
            source::Type_::Unit => canonical::Type::Unit,
            source::Type_::Constructor(cons, first_type, types) => types.iter().fold(
                canonical::Type::Application(
                    Box::new(self.tipe(cons)),
                    Box::new(self.tipe(first_type)),
                ),
                |cons, arg| canonical::Type::Application(Box::new(cons), Box::new(self.tipe(arg))),
            ),
            source::Type_::Variable(name) => canonical::Type::Variable(name.clone()),
            source::Type_::Identifier(name) => {
                canonical::Type::Identifier(self.qualify_type(name.to_owned()))
            }
            source::Type_::QualifiedIdentifier(module_name, name) => {
                canonical::Type::Identifier(Qualified::Foreign {
                    module: module_name.clone(),
                    member: name.clone(),
                })
            }
            source::Type_::Fn(f, x) => {
                canonical::Type::Lambda(Box::new(self.tipe(f)), Box::new(self.tipe(x)))
            }
            source::Type_::Record(fields) => canonical::Type::Record(
                fields
                    .iter()
                    .map(|(field, tipe)| (field.clone(), self.tipe(tipe)))
                    .collect(),
            ),
            source::Type_::Tuple(first, second, rest) => canonical::Type::Tuple(
                Box::new(self.tipe(first)),
                Box::new(self.tipe(second)),
                rest.iter().map(|tipe| self.tipe(tipe)).collect(),
            ),
        }
    }

    fn expression(&self, source_expr: source::Expr) -> canonical::Expr {
        // TODO: this doesn't generalize to custom subexpressions built in the canonicalization of operator expressions.
        let locate = |expr: canonical::Expr_| ast::Located {
            region: source_expr.region.clone(),
            inner: expr,
        };
        match source_expr.inner {
            source::Expr_::External(name) => {
                locate(canonical::Expr_::Variable(Qualified::Kernel(name)))
            }
            source::Expr_::Let(pattern, expr, body) => match &pattern.inner {
                source::Pattern_::Identifier(ident) => locate(canonical::Expr_::Let {
                    name: ident.to_owned(),
                    expr: Box::new(self.expression(*expr)),
                    body: Box::new(self.expression(*body)),
                }),
                _ => todo!(),
            },
            source::Expr_::Bind(pattern, expr, expr1) => todo!(),
            source::Expr_::If(cond, t, f) => locate(canonical::Expr_::If {
                cond: Box::new(self.expression(*cond)),
                true_branch: Box::new(self.expression(*t)),
                false_branch: Box::new(self.expression(*f)),
            }),
            source::Expr_::Ap(expr, expr1) => locate(canonical::Expr_::Ap {
                function: Box::new(self.expression(*expr)),
                arg: Box::new(self.expression(*expr1)),
            }),
            source::Expr_::Identifier(name) => {
                // TODO: this would be where we determine
                // a) whether this identifier is local or from another module and
                // b) which module it comes from
                locate(canonical::Expr_::Variable(Qualified::Local(name)))
            }
            source::Expr_::Lambda(pattern, expr) => match &pattern.inner {
                source::Pattern_::Identifier(ident) => locate(canonical::Expr_::Lambda {
                    arg: ident.to_owned(),
                    body: Box::new(self.expression(*expr)),
                }),
                source::Pattern_::Wildcard => locate(canonical::Expr_::Lambda {
                    arg: "__wildcard".to_owned(),
                    body: Box::new(self.expression(*expr)),
                }),
                _ => todo!(),
            },
            source::Expr_::BinOp { op, lhs, rhs } => {
                let lhs = Box::new(self.expression(*lhs));
                let rhs = Box::new(self.expression(*rhs));

                match op {
                    // TODO: find a point free way to do this
                    source::Operator::Compose => locate(canonical::Expr_::Lambda {
                        arg: "__arg".to_owned(),
                        body: Box::new(locate(canonical::Expr_::Ap {
                            function: lhs,
                            arg: Box::new(locate(canonical::Expr_::Ap {
                                function: rhs,
                                arg: Box::new(locate(canonical::Expr_::Variable(
                                    Qualified::Local("__arg".to_owned()),
                                ))),
                            })),
                        })),
                    }),
                    source::Operator::ComposeRev => locate(canonical::Expr_::Lambda {
                        arg: "__arg".to_owned(),
                        body: (Box::new(locate(canonical::Expr_::Ap {
                            function: rhs,
                            arg: Box::new(locate(canonical::Expr_::Ap {
                                function: lhs,
                                arg: Box::new(locate(canonical::Expr_::Variable(
                                    Qualified::Local("__arg".to_owned()),
                                ))),
                            })),
                        }))),
                    }),
                    source::Operator::Pipe => locate(canonical::Expr_::Ap {
                        function: lhs,
                        arg: rhs,
                    }),
                    source::Operator::PipeRev => locate(canonical::Expr_::Ap {
                        function: rhs,
                        arg: lhs,
                    }),
                    source::Operator::Cons => locate(canonical::Expr_::Ap {
                        function: Box::new(locate(canonical::Expr_::Ap {
                            function: Box::new(locate(canonical::Expr_::Constructor(
                                self.constructor("Cons".to_owned()),
                            ))),
                            arg: lhs,
                        })),
                        arg: rhs,
                    }),

                    source::Operator::Or => locate(canonical::Expr_::Op {
                        op: canonical::Operator::Or,
                        lhs,
                        rhs,
                    }),
                    source::Operator::And => locate(canonical::Expr_::Op {
                        op: canonical::Operator::And,
                        lhs,
                        rhs,
                    }),
                    source::Operator::Eq => locate(canonical::Expr_::Op {
                        op: canonical::Operator::Eq,
                        lhs,
                        rhs,
                    }),
                    source::Operator::Neq => locate(canonical::Expr_::Op {
                        op: canonical::Operator::Neq,
                        lhs,
                        rhs,
                    }),
                    source::Operator::LT => locate(canonical::Expr_::Op {
                        op: canonical::Operator::LT,
                        lhs,
                        rhs,
                    }),
                    source::Operator::LTE => locate(canonical::Expr_::Op {
                        op: canonical::Operator::LTE,
                        lhs,
                        rhs,
                    }),
                    source::Operator::GT => locate(canonical::Expr_::Op {
                        op: canonical::Operator::GT,
                        lhs,
                        rhs,
                    }),
                    source::Operator::GTE => locate(canonical::Expr_::Op {
                        op: canonical::Operator::GTE,
                        lhs,
                        rhs,
                    }),
                    source::Operator::Concat => locate(canonical::Expr_::Op {
                        op: canonical::Operator::Concat,
                        lhs,
                        rhs,
                    }),
                    source::Operator::Plus => locate(canonical::Expr_::Op {
                        op: canonical::Operator::Plus,
                        lhs,
                        rhs,
                    }),
                    source::Operator::Minus => locate(canonical::Expr_::Op {
                        op: canonical::Operator::Minus,
                        lhs,
                        rhs,
                    }),
                    source::Operator::Times => locate(canonical::Expr_::Op {
                        op: canonical::Operator::Times,
                        lhs,
                        rhs,
                    }),
                    source::Operator::Divide => locate(canonical::Expr_::Op {
                        op: canonical::Operator::Divide,
                        lhs,
                        rhs,
                    }),
                    source::Operator::Mod => locate(canonical::Expr_::Op {
                        op: canonical::Operator::Mod,
                        lhs,
                        rhs,
                    }),
                    source::Operator::Power => locate(canonical::Expr_::Op {
                        op: canonical::Operator::Power,
                        lhs,
                        rhs,
                    }),
                }
            }
            source::Expr_::When(expr, first, rest) => locate(canonical::Expr_::When {
                expr: Box::new(self.expression(*expr)),
                first_alternative: Box::new((self.pattern(&first.0), self.expression(first.1))),
                rest_alternatives: rest
                    .into_iter()
                    .map(|(pattern, body)| (self.pattern(&pattern), self.expression(body)))
                    .collect(),
            }),
            source::Expr_::Unit => locate(canonical::Expr_::Unit),
            source::Expr_::Bool(bool) => locate(canonical::Expr_::Bool(bool)),
            source::Expr_::Int(int) => locate(canonical::Expr_::Int(int as f64)),
            source::Expr_::Float(float) => locate(canonical::Expr_::Float(float as f64)),
            source::Expr_::String(string) => locate(canonical::Expr_::String(string)),
            source::Expr_::Record(hash_map) => todo!(),
            source::Expr_::QualifiedIdentifier(module, member) => {
                locate(canonical::Expr_::Variable(Qualified::Foreign {
                    module: module,
                    member: member,
                }))
            }
            source::Expr_::List(exprs) => {
                let type_str = "a".to_owned();
                let type_var = Box::new(canonical::Type::Variable(type_str.clone()));
                let list_type = canonical::Type::Application(
                    Box::new(canonical::Type::Identifier(Qualified::Foreign {
                        module: ModuleName(vec!["List".to_owned()]),
                        member: "List".to_owned(),
                    })),
                    type_var.clone(),
                );
                let qualify = |constructor| Qualified::Foreign {
                    module: ModuleName(vec!["List".to_owned()]),
                    member: constructor,
                };
                let empty = canonical::Expr_::Constructor(qualify(canonical::Constructor {
                    tag: 0,
                    arity: 0,
                    annotation: canonical::Annotation {
                        quantified: HashSet::from([type_str.clone()]),
                        tipe: list_type.clone(),
                    },
                }));
                let cons = canonical::Expr_::Constructor(qualify(canonical::Constructor {
                    tag: 1,
                    arity: 2,
                    annotation: {
                        canonical::Annotation {
                            quantified: HashSet::from([type_str.clone()]),
                            tipe: canonical::Type::Lambda(
                                type_var.clone(),
                                Box::new(canonical::Type::Lambda(
                                    Box::new(list_type.clone()),
                                    Box::new(list_type.clone()),
                                )),
                            ),
                        }
                    },
                }));
                exprs.into_iter().rev().fold(locate(empty), |list, expr| {
                    locate(canonical::Expr_::Ap {
                        function: Box::new(locate(canonical::Expr_::Ap {
                            function: { Box::new(locate(cons.clone())) },
                            arg: Box::new(self.expression(expr)),
                        })),
                        arg: Box::new(list),
                    })
                })
            }
            source::Expr_::Constructor(name) => {
                locate(canonical::Expr_::Constructor(self.constructor(name)))
            }
            source::Expr_::Tuple(exprs) => todo!(),
            source::Expr_::QualifiedConstructor(module_name, name) => {
                locate(canonical::Expr_::Variable(Qualified::Foreign {
                    module: module_name,
                    member: name,
                }))
            }
        }
    }

    fn constructor(&self, constructor: Name) -> Qualified<canonical::Constructor> {
        // CHECK FOR LOCAL
        if let Some(cons) = self.constructors.get(&constructor) {
            Qualified::Local(cons.clone())
        } else {
            todo!("cannot find constructor {}", constructor)
        }
    }

    fn alias(&self, alias: &source::Alias) -> canonical::Alias {
        canonical::Alias {
            variables: alias.variables.clone(),
            other: self.tipe(&alias.other),
        }
    }

    fn union_artifacts(
        &self,
        name: Name,
        union: &source::Union,
    ) -> (canonical::Union, HashMap<Name, canonical::Constructor>) {
        let constructors = union
            .variants
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, constructor)| {
                (
                    constructor.name,
                    canonical::Constructor {
                        tag: i as u16,
                        arity: constructor.args.len() as u16,
                        annotation: {
                            let tipe = constructor
                                .args
                                .clone()
                                .into_iter()
                                .map(|arg| self.tipe(&arg))
                                .rev()
                                .fold(
                                    canonical::Type::Identifier(Qualified::Local(name.clone())),
                                    |ret, arg| {
                                        canonical::Type::Lambda(Box::new(arg), Box::new(ret))
                                    },
                                );

                            canonical::Annotation {
                                quantified: tipe.free_variables(),
                                tipe,
                            }
                        },
                    },
                )
            })
            .collect::<HashMap<Name, canonical::Constructor>>();

        let union = canonical::Union {
            variables: union.variables.clone(),
            constructors: constructors.keys().map(|k| k.clone()).collect(),
        };

        (union, constructors)
    }

    fn pattern(&self, pattern: &source::Pattern) -> canonical::Pattern {
        Located {
            region: pattern.region.clone(),
            inner: match &pattern.inner {
                source::Pattern_::Wildcard => canonical::Pattern_::Wildcard,
                source::Pattern_::Identifier(name) => canonical::Pattern_::Identifier(name.clone()),
                source::Pattern_::Constructor(name, args) => canonical::Pattern_::Constructor(
                    self.constructors.get(name).unwrap().clone(),
                    args.clone()
                        .into_iter()
                        .map(|arg| self.pattern(&arg))
                        .collect::<Vec<canonical::Pattern>>(),
                ),
                source::Pattern_::Cons(element, list) => {
                    self.pattern(&source::Pattern {
                        region: pattern.region.clone(),
                        inner: source::Pattern_::Constructor(
                            "Cons".to_owned(),
                            vec![*element.clone(), *list.clone()],
                        ),
                    })
                    .inner
                }
                source::Pattern_::Tuple(locateds) => todo!(),
            },
        }
    }
}
