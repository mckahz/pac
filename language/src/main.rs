mod ast;
mod parse;
mod pretty;

use ast::*;
use nom_supreme::{
    error::{ErrorTree, GenericErrorTree},
    final_parser::final_parser,
};
use pretty::*;
use std::{
    collections::HashMap,
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

fn load(root_dir: &Path) -> Result<Project, Box<dyn std::error::Error>> {
    let paths = get_paths(root_dir);
    let mut modules: Vec<Module> = vec![];
    for path in paths {
        let file = std::fs::read_to_string(&path).unwrap();
        let parse_results: std::result::Result<Module, ErrorTree<&str>> =
            final_parser(parse::file)(&file);
        match parse_results {
            Ok(module) => modules.push(module),
            Err(e) => {
                eprintln!("{}", e.pretty_print());
                break;
            }
        }
    }
    Ok(Project { modules })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let root_dir = Path::new(&args[1]);
    let project = load(root_dir).unwrap();
    println!("Project Loaded!\n\n");

    let mut env = Env {
        modules: project.modules,
        stack_frames: vec![],
    };

    let main_module = env.module("Main");
    let entry_point = main_module.value("main").clone();
    env.push_frame(Frame { statements: main_module.statements.clone() });

    &entry_point.clone().eval(&mut env).execute();

    Ok(())
}
