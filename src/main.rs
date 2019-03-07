#[macro_use]
extern crate lalrpop_util;
extern crate aps_lib;

lalrpop_mod!(pub aps);

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::Command;

use std::collections::HashMap;

use aps_lib::ast;

fn reader_from_file(filename: &PathBuf) -> BufReader<File> {
    let file = File::open(filename).expect("Impossible to open file.");
    BufReader::new(file)
}

fn test_type(ast: &Box<ast::AstCdms>) -> bool {
    let mut prolog = ast.to_prolog();
    prolog.push('.');

    println!("\nOutProlog : {}", prolog);

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

    if result_str.eq("void") {
        true
    } else {
        false
    }
}

fn main() {
    println!("Version AST1");

    let parser_ast = aps::ProgParser::new();

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
            println!("AST : {:?}", ast);

            let type_res = test_type(&ast);

            if !type_res {
                println!("Fin Du prgramme");
                println!("Erreur de typage!");
            } else {
                println!("Typage Correct!");
                // println!("Evaluation :");
                //    let eval = ast.eval(&mut HashMap::new());
            }
        }
    }
}
fn test_prog(filename: String) {
    let parser_ast = aps::ProgParser::new();

    let d = PathBuf::from(filename);
    println!("Parse file : {:?}", d);

    let mut code_reader = reader_from_file(&d);
    let mut code: String = String::new();
    let _ = code_reader.read_to_string(&mut code);
    let ast = parser_ast.parse(&code);
    println!("Code :\n{}", code);
    assert!(ast.is_ok());
    let past = ast.unwrap();
    println!("AST : {:?}", past);
    let type_res = test_type(&past);
    assert!(type_res);
}

#[cfg(test)]
mod aps0 {

    use super::*;
    extern crate aps_lib;

    #[test]
    fn prog_000() {
        test_prog("test/aps0/prog000.aps".to_string());
    }

    #[test]
    fn prog_001() {
        test_prog("test/aps0/prog001.aps".to_string());
    }

    #[test]
    fn prog_002() {
        test_prog("test/aps0/prog002.aps".to_string());
    }

    #[test]
    fn prog_003() {
        test_prog("test/aps0/prog003.aps".to_string());
    }

    #[test]
    fn prog_004() {
        test_prog("test/aps0/prog004.aps".to_string());
    }

    #[test]
    fn prog_005() {
        test_prog("test/aps0/prog005.aps".to_string());
    }

    #[test]
    fn prog_006() {
        test_prog("test/aps0/prog006.aps".to_string());
    }

    #[test]
    fn prog_007() {
        test_prog("test/aps0/prog007.aps".to_string());
    }

    #[test]
    fn prog_008() {
        test_prog("test/aps0/prog008.aps".to_string());
    }

    #[test]
    fn prog_009() {
        test_prog("test/aps0/prog009.aps".to_string());
    }

    #[test]
    fn prog_010() {
        test_prog("test/aps0/prog010.aps".to_string());
    }

    #[test]
    fn prog_011() {
        test_prog("test/aps0/prog011.aps".to_string());
    }

    #[test]
    fn prog_012() {
        test_prog("test/aps0/prog012.aps".to_string());
    }

    #[test]
    fn prog_013() {
        test_prog("test/aps0/prog013.aps".to_string());
    }

    #[test]
    fn prog_014() {
        test_prog("test/aps0/prog014.aps".to_string());
    }

    #[test]
    fn prog_015() {
        test_prog("test/aps0/prog015.aps".to_string());
    }

    #[test]
    fn prog_016() {
        test_prog("test/aps0/prog016.aps".to_string());
    }

    #[test]
    fn prog_017() {
        test_prog("test/aps0/prog017.aps".to_string());
    }
}

#[cfg(test)]
mod aps1 {

    use super::*;
    extern crate aps_lib;

    #[test]
    fn prog_100() {
        test_prog("test/aps1/prog100.aps".to_string());
    }

    #[test]
    fn parse_prog_101() {
        test_prog("test/aps1/prog101.aps".to_string());
    }

    #[test]
    fn parse_prog_102() {
        test_prog("test/aps1/prog102.aps".to_string());
    }

    #[test]
    fn parse_prog_103() {
        test_prog("test/aps1/prog103.aps".to_string());
    }
    #[test]
    fn parse_prog_104() {
        test_prog("test/aps1/prog104.aps".to_string());
    }

    #[test]
    fn parse_prog_105() {
        test_prog("test/aps1/prog105.aps".to_string());
    }

    #[test]
    fn parse_prog_106() {
        test_prog("test/aps1/prog106.aps".to_string());
    }

    #[test]
    fn parse_prog_107() {
        test_prog("test/aps1/prog107.aps".to_string());
    }

    #[test]
    fn parse_prog_108() {
        test_prog("test/aps1/prog108.aps".to_string());
    }

    #[test]
    fn parse_prog_109() {
        test_prog("test/aps1/prog109.aps".to_string());
    }

    #[test]
    fn parse_prog_110() {
        test_prog("test/aps1/prog110.aps".to_string());
    }

    #[test]
    fn parse_prog_111() {
        test_prog("test/aps1/prog111.aps".to_string());
    }

    #[test]
    fn parse_prog_112() {
        test_prog("test/aps1/prog112.aps".to_string());
    }

    #[test]
    fn parse_prog_113() {
        test_prog("test/aps1/prog113.aps".to_string());
    }

    #[test]
    fn parse_prog_114() {
        test_prog("test/aps1/prog114.aps".to_string());
    }

    #[test]
    fn parse_prog_115() {
        test_prog("test/aps1/prog115.aps".to_string());
    }

    #[test]
    fn parse_prog_116() {
        test_prog("test/aps1/prog116.aps".to_string());
    }

    #[test]
    fn parse_prog_117() {
        test_prog("test/aps1/prog117.aps".to_string());
    }

    #[test]
    fn parse_prog_118() {
        test_prog("test/aps1/prog118.aps".to_string());
    }

    #[test]
    fn parse_prog_119() {
        test_prog("test/aps1/prog119.aps".to_string());
    }

    #[test]
    fn parse_prog_120() {
        test_prog("test/aps1/prog120.aps".to_string());
    }
}
