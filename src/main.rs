#[macro_use]
extern crate lalrpop_util;
extern crate rum_lib;

lalrpop_mod!(pub rum);

use std::fs::File;
use std::fs::{self};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

use std::collections::HashMap;

use rum_lib::config::Config;
use rum_lib::rum_type;

fn reader_from_file(filename: &PathBuf) -> BufReader<File> {
    let file = File::open(filename).expect("Impossible to open file.");
    BufReader::new(file)
}

fn main() {
    let config = Config::new();

    if config.debug {
        println!("RUM Version 0.6");
    }

    let arguments = std::env::args();

    for (i, arg) in arguments.enumerate() {
        if i != 0 {
            let code_path = PathBuf::from(&arg);
            let mut code: String = String::new();
            reader_from_file(&code_path)
                .read_to_string(&mut code)
                .expect("Error reading code file");
            if config.debug {
                println!("Code :\n{}", code);
                println!("Parsing prog : {}", arg);
            }
            let parser_ast = rum::ProgParser::new();
            let ast = parser_ast.parse(&code).expect("Parser failure");
            let type_checher = rum_type::Type::type_check(&ast);
            if config.debug {
                println!("AST : {:#?}", ast);
                print!(" test type check : {:?} ", type_checher);
            }
            let mut mem = rum_lib::eval::Memoire { mem: Vec::new() };

            let evalued = ast.eval(&mut HashMap::new(), &mut mem, &config);
            if config.debug {
                println!("memoire : {:?}", mem);
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

            for fichier in fichiers_test{
                test_prog(&fichier)
            }
    }
}

fn test_prog(filename: &PathBuf) {
    let config = Config::new();
    let parser_ast = rum::ProgParser::new();

    let code_path = PathBuf::from(filename);
    let mut code: String = String::new();
    reader_from_file(&code_path)
        .read_to_string(&mut code)
        .expect("Error reading code file");

    println!("Code :\n{}", code);

    let ast = parser_ast.parse(&code).expect("Parser failure");
    let type_checher = rum_type::Type::type_check(&ast);
    print!(" test type check : {:?} ", type_checher);

    println!("AST : {:#?}", ast);

    let mut mem = rum_lib::eval::Memoire { mem: Vec::new() };
    let evalued = ast.eval(&mut HashMap::new(), &mut mem, &config);
    println!("memoire : {:?}", mem);
}

#[test]
fn rum0() {
    test_folder(&PathBuf::from("test/rum0"));
}
