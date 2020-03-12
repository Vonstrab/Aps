#[macro_use]
extern crate lalrpop_util;
extern crate rum_lib;

lalrpop_mod!(pub rum);

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::Command;

use std::collections::HashMap;

use rum_lib::ast;

fn reader_from_file(filename: &PathBuf) -> BufReader<File> {
    let file = File::open(filename).expect("Impossible to open file.");
    BufReader::new(file)
}

fn test_type(ast: &Box<ast::AstCdms>) -> bool {
    let mut prolog = ast.to_prolog();
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

    if result_str.eq("void") {
        true
    } else {
        false
    }
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
            println!("AST : {:?}", ast);

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
fn test_prog(filename: String, expected: &Vec<i64>) {
    let parser_ast = rum::ProgParser::new();

    let d = PathBuf::from(filename);
    println!("Parse file : {:?}", d);

    let mut code_reader = reader_from_file(&d);
    let mut code: String = String::new();
    let _ = code_reader.read_to_string(&mut code);
    println!("Code :\n{}", code);

    let ast = parser_ast.parse(&code);
    let past = ast.expect("Parser failure");
    println!("AST : {:?}", past);
    let type_res = test_type(&past);
    if !type_res {
        panic!("Erreur de type");
    }
    let mut mem = rum_lib::eval::Memoire { mem: Vec::new() };
    assert_eq!(*expected, past.eval(&mut HashMap::new(), &mut mem));
    println!("memoire : {:?}", mem);
}

#[cfg(test)]
mod rum0 {

    use super::*;
    extern crate rum_lib;

    #[test]
    fn prog_000() {
        test_prog("test/rum0/prog000.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_001() {
        test_prog("test/rum0/prog001.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_002() {
        test_prog("test/rum0/prog002.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_003() {
        test_prog("test/rum0/prog003.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_004() {
        test_prog("test/rum0/prog004.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_005() {
        test_prog("test/rum0/prog005.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_006() {
        test_prog("test/rum0/prog006.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_007() {
        test_prog("test/rum0/prog007.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_008() {
        test_prog("test/rum0/prog008.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_009() {
        test_prog("test/rum0/prog009.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_010() {
        test_prog("test/rum0/prog010.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_011() {
        test_prog("test/rum0/prog011.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_012() {
        test_prog("test/rum0/prog012.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_013() {
        test_prog("test/rum0/prog013.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_014() {
        test_prog("test/rum0/prog014.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_015() {
        test_prog("test/rum0/prog015.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_016() {
        test_prog("test/rum0/prog016.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_017() {
        test_prog("test/rum0/prog017.rum".to_string(), &vec![42]);
    }
}

#[cfg(test)]
mod rum1 {

    use super::*;
    extern crate rum_lib;

    #[test]
    fn prog_100() {
        test_prog("test/rum1/prog100.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_101() {
        test_prog("test/rum1/prog101.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_102() {
        test_prog("test/rum1/prog102.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_103() {
        test_prog("test/rum1/prog103.rum".to_string(), &vec![42]);
    }
    #[test]
    fn prog_104() {
        test_prog("test/rum1/prog104.rum".to_string(), &vec![42]);
    }

    #[test]
    #[should_panic(expected = "variable not initialised")]
    fn prog_105() {
        test_prog("test/rum1/prog105.rum".to_string(), &vec![42]);
    }

    #[test]
    #[should_panic(expected = "variable not initialised")]
    fn prog_106() {
        test_prog("test/rum1/prog106.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_107() {
        test_prog("test/rum1/prog107.rum".to_string(), &vec![0, 42]);
    }

    #[test]
    fn prog_108() {
        test_prog("test/rum1/prog108.rum".to_string(), &vec![42, 42]);
    }

    // #[test]
    fn prog_109() {
        test_prog("test/rum1/prog109.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_110() {
        test_prog("test/rum1/prog110.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_111() {
        test_prog("test/rum1/prog111.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_112() {
        test_prog("test/rum1/prog112.rum".to_string(), &vec![1]);
    }

    #[test]
    fn prog_113() {
        test_prog("test/rum1/prog113.rum".to_string(), &vec![0]);
    }

    #[test]
    fn prog_114() {
        test_prog("test/rum1/prog114.rum".to_string(), &vec![1]);
    }

    #[test]
    fn prog_115() {
        test_prog("test/rum1/prog115.rum".to_string(), &vec![41, 42]);
    }

    #[test]
    fn prog_116() {
        test_prog("test/rum1/prog116.rum".to_string(), &vec![21, 42]);
    }

    #[test]
    fn prog_117() {
        test_prog("test/rum1/prog117.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_118() {
        test_prog("test/rum1/prog118.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_119() {
        test_prog("test/rum1/prog119.rum".to_string(), &vec![0, 42]);
    }

    #[test]
    fn prog_120() {
        test_prog("test/rum1/prog120.rum".to_string(), &vec![0, 42]);
    }
}
#[cfg(test)]
mod rum2 {

    use super::*;
    extern crate rum_lib;

    #[test]
    fn prog_200() {
        test_prog("test/rum2/prog200.rum".to_string(), &vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn prog_201() {
        test_prog("test/rum2/prog201.rum".to_string(), &vec![]);
    }

    #[test]
    fn prog_202() {
        test_prog("test/rum2/prog202.rum".to_string(), &vec![]);
    }

    #[test]
    fn prog_203() {
        test_prog("test/rum2/prog203.rum".to_string(), &vec![]);
    }
    #[test]
    fn prog_204() {
        test_prog("test/rum2/prog204.rum".to_string(), &vec![]);
    }

    #[test]
    fn prog_205() {
        test_prog("test/rum2/prog205.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_206() {
        test_prog("test/rum2/prog206.rum".to_string(), &vec![42]);
    }

    #[test]
    fn prog_207() {
        test_prog("test/rum2/prog207.rum".to_string(), &vec![]);
    }
}
