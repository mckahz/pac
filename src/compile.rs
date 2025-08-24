use crate::ast::{optimized::*, ModuleName, Qualified};
use crate::util::{indent, to_camel_case};

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

trait ToJs {
    fn to_js(&self) -> String;
}

impl ToJs for Operator {
    fn to_js(&self) -> String {
        let s = match self {
            Operator::Or => "||",
            Operator::And => "&&",
            Operator::Eq => "===",
            Operator::Neq => "!=",
            Operator::LT => "<",
            Operator::LTE => "<=",
            Operator::GT => ">",
            Operator::GTE => ">=",
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Times => "*",
            Operator::Divide => "/",
            Operator::Mod => "%",
            Operator::Power => "^",
        };
        s.to_owned()
    }
}

impl ToJs for Expr {
    fn to_js(&self) -> String {
        match &self {
            Expr::String(s) => "\"".to_owned() + &s + "\"",
            Expr::Int(num) => num.to_string(),
            Expr::Float(num) => num.to_string(),
            Expr::Identifier(identifier) => match identifier {
                Qualified::Foreign { module, member } => {
                    format!("{}.{}", module, to_camel_case(member))
                }
                Qualified::Local(name) => to_camel_case(name).to_owned(),
                Qualified::Kernel(name) => name.clone(),
            },
            Expr::Constructor { tag, arity } => {
                let args = (0..*arity as usize)
                    .into_iter()
                    .map(|i| format!("__arg{}", (i.to_string())))
                    .collect::<Vec<String>>();

                format!(
                    "({} {{ return {{ tag: {}, arity: {}, args: [{}] }}; }}){}",
                    if args.is_empty() {
                        "() =>".to_owned()
                    } else {
                        args.iter()
                            .map(|arg| format!("({}) => ", arg))
                            .collect::<Vec<String>>()
                            .join("")
                    },
                    tag.to_string(),
                    arity.to_string(),
                    args.join(", "),
                    if args.is_empty() { "()" } else { "" }
                )
            }
            Expr::List(exprs) => {
                "[".to_owned()
                    + &exprs
                        .iter()
                        .map(|expr| expr.to_js())
                        .collect::<Vec<String>>()
                        .join(", ")
                    + "]"
            }
            Expr::Extern(string) => match &string[..] {
                "println" => "console.log".to_owned(),
                "crash" => "console.error".to_owned(),
                "to_string" => "((x) =>
    (typeof x === 'number') ? '' + x
    : (typeof x === 'string') ? '\"' + x + '\"'
    : ('tag' in x) ? 'pack_' + x.tag + '_' + x.arity + '(' + x.args.map(toString).join(', ') + ')'
    : ''
)"
                .to_owned(),
                _ => string.to_owned(),
            },
            Expr::Lambda { arg, body } => "(".to_owned() + &arg + ") => " + &body.to_js(),
            Expr::Op { op, lhs, rhs } => {
                "(".to_owned() + &lhs.to_js() + " " + &op.to_js() + " " + &rhs.to_js() + ")"
            }
            Expr::When {
                expr,
                decision_tree,
            } => todo!("compile when expressions"),
            Expr::Ap { function, arg } => function.to_js() + "(" + &arg.to_js() + ")",
            Expr::Let { name, expr, body } => todo!(),
            Expr::LetRec { defs, body } => todo!(),
            Expr::If {
                cond,
                true_branch,
                false_branch,
            } => format!(
                "({} ? {} : {})",
                cond.to_js(),
                true_branch.to_js(),
                false_branch.to_js()
            ),
            Expr::Bool(bool) => (if *bool { "true" } else { "false" }).to_owned(),
        }
    }
}

const BUILD_DIR: &str = "build/js/";

fn compile_module(
    module_name: ModuleName,
    module: Module,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(&(BUILD_DIR.to_owned() + &format!("{}", module_name) + ".js"))?;
    if module_name != ModuleName(vec!["Basics".to_owned()]) {
        file.write("import { toString } from \"./Basics.js\";\n".as_bytes());
    }

    for import in &module.imports {
        let js = format!("import * as {i} from \"./{i}.js\";\n", i = &import);
        file.write(js.as_bytes()).unwrap();
    }

    file.write("\n".as_bytes());

    for (binding, body) in &module.definitions {
        let export = if module.exports.contains(binding) {
            "export "
        } else {
            ""
        };
        let js = export.to_owned()
            + "const "
            + &to_camel_case(binding)
            + " = "
            + &body.to_js()
            + ";\n\n";
        file.write(js.as_bytes()).unwrap();
    }

    Ok(())
}

pub fn compile(modules: HashMap<ModuleName, Module>) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::remove_dir_all(BUILD_DIR);
    std::fs::create_dir(BUILD_DIR);

    for (module_name, module) in modules {
        println!("compiling {}.pac", module_name);
        compile_module(module_name, module)?;
    }
    Ok(())
}
