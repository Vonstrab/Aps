#[macro_use]
extern crate lalrpop_util;
extern crate rum_lib;

lalrpop_mod!(pub rum);

use std::fs::File;
use std::fs::{self, DirEntry};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use std::collections::HashMap;

use rum_lib::ast;
use rum_lib::rum_type;
use rum_lib::type_checker;

fn reader_from_file(filename: &PathBuf) -> BufReader<File> {
    let file = File::open(filename).expect("Impossible to open file.");
    BufReader::new(file)
}

fn test_type(ast: &ast::AstCdms) -> bool {
    let mut prolog = String::new();
    prolog.push('.');

    println!("\nOutProlog : {} ", prolog);

    let script_prolog = Command::new("./typage.sh")
        .arg(prolog)
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    let result = script_prolog.stdout;
    let mut result_str = String::new();

    for c in result {
        result_str.push(c as char);
    }

    result_str.pop();

    result_str.eq("void")
}

fn main() {
    println!("RUM Version 3");

    let parser_ast = rum::ProgParser::new();

    let arguments = std::env::args();

    for (i, arg) in arguments.enumerate() {
        if i != 0 {
            println!("Parsing prog : {}", arg);

            let path = PathBuf::from(arg.as_str());

            let mut code_reader = reader_from_file(&path);
            let mut code: String = String::new();
            let _rey = code_reader.read_to_string(&mut code);
            println!("Code :\n{}", code);

            let ast = parser_ast.parse(&code).unwrap();
            println!("AST : {:#?}", ast);

            let type_res = test_type(&ast);

            if !type_res {
                println!("Fin Du prgramme");
                println!("Erreur de typage!");
            } else {
                println!("Typage Correct!");
                ast.eval(
                    &mut HashMap::new(),
                    &mut rum_lib::eval::Memoire { mem: Vec::new() },
                );
                println!("Fin Du prgramme");
            }
        }
    }
}

#[allow(dead_code)]

fn test_folder(folder: &PathBuf) {
    if folder.is_dir() {
        let fichiers_test = fs::read_dir(folder)
            .expect("Folder not found")
            .filter_map(|entry| {
                let entry_path = entry.expect("IO Error").path();
                if !entry_path.is_dir()
                    && entry_path.extension().unwrap_or_default().to_str().unwrap() == "rum"
                {
                    Some(entry_path)
                } else {
                    None
                }
            });

        fichiers_test.map(|fichier| test_prog(&fichier));
    }
}

fn test_prog(filename: &PathBuf) {
    let parser_ast = rum::ProgParser::new();

    let code_path = PathBuf::from(filename);
    let mut code: String = String::new();
    reader_from_file(&code_path)
        .read_to_string(&mut code)
        .expect("Error reading code file");

    let mut output_path = PathBuf::from(filename);
    output_path.set_extension("out");
    let mut output: String = String::new();
    reader_from_file(&output_path)
        .read_to_string(&mut output)
        .expect("Error reading output file");

    let mut result_path = PathBuf::from(filename);
    result_path.set_extension("result");
    let mut result: String = String::new();
    reader_from_file(&result_path)
        .read_to_string(&mut result)
        .expect("Error reading output file");

    println!("Code :\n{}", code);
    println!("Expected result :\n{}", result);
    println!("Expected output :\n{}", output);

    let ast = parser_ast.parse(&code).expect("Parser failure");
    let type_checher = rum_type::Type::type_check(&ast);
    print!(" test type check : {:?} ", type_checher);

    println!("AST : {:#?}", ast);

    let mut mem = rum_lib::eval::Memoire { mem: Vec::new() };
    let evalued = ast.eval(&mut HashMap::new(), &mut mem);
    println!("memoire : {:?}", mem);
}

#[test]
fn rum0() {
    test_folder(&PathBuf::from("test/rum0"));
}
