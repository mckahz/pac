#![allow(warnings)]

mod ast;
mod canonicalize;
mod compile;
mod optimize;
mod parse;
mod report;
mod type_check;
mod util;

use nom::{combinator::complete, Finish, Parser};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::{
    ast::{ModuleName, Name, Span},
    canonicalize::canonicalize,
    optimize::optimize,
    report::{code::Source, pretty::PrettyPrint, Report},
    type_check::type_check,
};

fn get_paths(root_dir: &Path) -> Vec<PathBuf> {
    //return vec![root_dir.join(Path::new("List.lang")).to_path_buf()];
    let entries = std::fs::read_dir(root_dir).unwrap();
    let mut paths = vec![];
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            paths.push(path);
        } else {
            let sub_paths = get_paths(&path);
            paths.extend(sub_paths.into_iter())
        }
    }
    return paths;
}

fn load_module(path: PathBuf) -> Option<ast::source::Module> {
    let file_str = std::fs::read_to_string(&path).unwrap();
    let file = Span::new(&file_str);
    let file_source = Source::new(&file_str);
    let file_name = path.as_os_str().to_str().unwrap();

    let parse_results = complete(parse::file).parse(file).finish();

    match parse_results {
        Ok((_, module)) => Some(module),
        Err(err) => {
            eprintln!(
                "\n{}",
                err.to_report(file_source, file_name)
                    .render(std::cmp::min(termsize::get().unwrap().cols as u32, 80))
            );
            None
        }
    }
}

fn load(root_dir: &Path) -> Option<Vec<ast::source::Module>> {
    let paths = get_paths(root_dir);
    let num_paths = paths.len();
    let modules: Vec<ast::source::Module> = paths.into_iter().filter_map(load_module).collect();
    if modules.len() < num_paths {
        None
    } else {
        Some(modules)
    }
}

fn main() {
    //std::env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = std::env::args().collect();
    let root_dir = Path::new("test"); //(&args[1]);

    let mut modules = vec![];
    let Some(mut builtin_modules) = load(Path::new("src/basics")) else {
        eprintln!("\n\nI couldn't load all of the built in modules!");
        std::process::exit(1);
    };
    modules.append(&mut builtin_modules);

    let Some(mut user_modules) = load(root_dir) else {
        eprintln!(
            "\n\nI couldn't load all the modules in the {:?} folder!",
            root_dir
        );
        std::process::exit(1);
    };
    // TODO: import all builtin modules to the user modules
    modules.append(&mut user_modules);

    let modules = canonicalize(modules);
    type_check(&modules).expect("failed type checking");
    let modules = optimize(&modules);
    compile::compile(modules);
}
