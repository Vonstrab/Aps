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

fn main() {
    println!("RUM Version 3");

    let parser_ast = rum::ProgParser::new();

    let arguments = std::env::args();

    for (i, arg) in arguments.enumerate() {
        if i != 0 {
            let code_path = PathBuf::from(&arg);
            let mut code: String = String::new();
            reader_from_file(&code_path)
                .read_to_string(&mut code)
                .expect("Error reading code file");

            println!("Code :\n{}", code);

            println!("Parsing prog : {}", arg);
            let parser_ast = rum::ProgParser::new();
            let ast = parser_ast.parse(&code).expect("Parser failure");
            let type_checher = rum_type::Type::type_check(&ast);
            println!("AST : {:#?}", ast);
        
            print!(" test type check : {:?} ", type_checher);
        
            let mut mem = rum_lib::eval::Memoire { mem: Vec::new() };

            let evalued = ast.eval(&mut HashMap::new(), &mut mem);
            
            println!("memoire : {:?}", mem);

            println!("Fin Du prgramme");
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
