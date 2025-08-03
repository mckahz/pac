#![allow(warnings)]

mod ast;
mod canonicalize;
mod compile;
mod parse;
mod report;
mod util;

use ast::Span;
use canonicalize::canonicalize;
use nom::{combinator::complete, Parser};
use report::{pretty::PrettyPrint, Report};
use std::{
    fs,
    path::{Path, PathBuf},
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

pub fn load(root_dir: &Path) -> Result<Vec<ast::source::Module>, Vec<()>> {
    let paths = get_paths(root_dir);
    let mut modules = vec![];
    let mut error_reports = vec![];
    for path in paths {
        let file_str = std::fs::read_to_string(&path).unwrap();
        let file = Span::new(&file_str);
        let parse_results: Result<ast::source::Module, _> =
            complete(parse::file).parse(file).map(|tuple| tuple.1);
        match parse_results {
            Ok(module) => modules.push(module),
            Err(err) => {
                eprintln!(
                    "error parsing {}:\n{:?}",
                    &path.as_os_str().to_str().unwrap(),
                    err
                );
                error_reports.push(());
            }
        }
    }

    if error_reports.is_empty() {
        Ok(modules)
    } else {
        Err(todo!("collecting error reports"))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    //std::env::set_var("RUST_BACKTRACE", "1");
    let root_dir = Path::new(&args[1]);
    let all_modules = {
        let mut user_modules = load(root_dir).unwrap();
        let mut builtin_modules = load(Path::new("src/basics")).unwrap();

        let mut all = vec![];
        all.append(&mut user_modules);
        all.append(&mut builtin_modules);
        all
    };
    let modules = canonicalize(all_modules);

    compile::compile(modules);
}
