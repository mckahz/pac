#![allow(warnings)]

mod compile;
mod core;
mod parse;
mod pretty;
mod util;

use nom_supreme::{error::ErrorTree, final_parser::final_parser};
use pretty::PrettyPrint;
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

pub fn load(root_dir: &Path) -> Result<Vec<parse::ast::Module>, Box<dyn std::error::Error>> {
    let paths = get_paths(root_dir);
    let mut modules: Vec<parse::ast::Module> = vec![];
    // TODO: remove head operation
    for path in paths {
        let file = std::fs::read_to_string(&path).unwrap();
        let parse_results: std::result::Result<parse::ast::Module, ErrorTree<&str>> =
            final_parser(crate::parse::file)(&file);
        match parse_results {
            Ok(module) => {
                modules.push(module);
            }
            Err(e) => {
                eprintln!(
                    "I encountered an error when parsing the {:?}.pac module",
                    path.file_name().unwrap()
                );
                eprintln!("{}", e.pretty_print());
            }
        }
    }
    Ok(modules)
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
    let modules = core::canonicalize::canonicalize(all_modules);

    compile::compile(modules);
}
