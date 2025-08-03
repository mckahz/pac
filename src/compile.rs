use crate::ast::core::*;
use crate::util::{indent, to_camel_case};

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
            Operator::Concat => "+",
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
        match &self.inner {
            Expr_::String(s) => "\"".to_owned() + &s + "\"",
            Expr_::Num(num) => num.to_string(),
            Expr_::Binding(binding) => to_camel_case(binding).to_owned(),
            Expr_::Constructor { tag, arity } => {
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
            Expr_::List(exprs) => {
                "[".to_owned()
                    + &exprs
                        .iter()
                        .map(|expr| expr.to_js())
                        .collect::<Vec<String>>()
                        .join(", ")
                    + "]"
            }
            Expr_::Extern(string) => match &string[..] {
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
            Expr_::Lambda { arg, body } => "(".to_owned() + &arg + ") => " + &body.to_js(),
            Expr_::Op { op, lhs, rhs } => {
                "(".to_owned() + &lhs.to_js() + " " + &op.to_js() + " " + &rhs.to_js() + ")"
            }
            Expr_::When { expr, alternatives } => {
                format!(
                            "(() => {{
    const __expr = {};
    const __functions = [
{}
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
}})()",
                            &expr.to_js(),
                            &alternatives
                                .iter()
                                .map(|Alternative { tag, args, body }| {
                                    indent(&&indent(
                                        &("(".to_owned() + &args.join(",") + ") => " + &body.to_js()),
                                    ))
                                })
                                .collect::<Vec<String>>()
                                .join(",\n")
                        )
            }
            Expr_::Ap { function, arg } => function.to_js() + "(" + &arg.to_js() + ")",
            Expr_::Let { defs, body } => todo!(),
            Expr_::LetRec { defs, body } => todo!(),
            Expr_::ModuleAccess { module, member } => module.to_owned() + "." + &member,
            Expr_::If {
                cond,
                true_branch,
                false_branch,
            } => format!(
                "({} ? {} : {})",
                cond.to_js(),
                true_branch.to_js(),
                false_branch.to_js()
            ),
        }
    }
}

const BUILD_DIR: &str = "build/js/";

fn compile_module(module: Module) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(&(BUILD_DIR.to_owned() + &module.name + ".js"))?;
    if module.name != "Basics" {
        file.write("import { toString } from \"./Basics.js\";\n".as_bytes());
    }

    for import in &module.imports {
        let js = format!(
            "import * as {} from {};\n",
            &import.name,
            "\"./".to_owned() + &import.name + ".js\""
        );
        file.write(js.as_bytes()).unwrap();
    }

    file.write("\n".as_bytes());

    for (binding, body) in &module.defs {
        let export = if module.interface.contains(binding) {
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

pub fn compile(modules: Vec<Module>) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::remove_dir_all(BUILD_DIR);
    std::fs::create_dir(BUILD_DIR);

    for module in modules {
        println!("compiling {}.pac", module.name);
        compile_module(module)?;
    }
    Ok(())
}
