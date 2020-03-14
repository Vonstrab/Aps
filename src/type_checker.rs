use crate::ast::AstCdms;
use crate::ast::AstExp;
use crate::ast::Oprim;

use crate::rum_type::Type;

use std::collections::HashMap;

// impl AstExp {
//     pub fn type_check(&self, type_cache: &HashMap<String, Type>) -> Type {
//         println!("\nInto Expr eval");
//         println!("expr : {:?}", self);
//         println!("type_cache {:?}", type_cache);

//         use AstExp::*;

//         match self {
//             ASTInt(n) => Type::Int,
//             ASTBool(b) => Type::Bool,
//             ASTUnPrim(_, exp) => {
//                 let exp_type = exp.type_check(type_cache);
//                 if exp_type == Type::Bool {
//                     Type::Bool
//                 } else {
//                     Type::TypeError(format!("Not primitive function require a boolean argument"))
//                 }
//             }
//             ASTBinPrim(op, e1, e2) => {
//                 let expr1 = e1.type_check(type_cache);
//                 let expr2 = e2.type_check(type_cache);

//                 match op {
//                     Oprim::Add => Value::Int(expr1 + expr2),
//                     Oprim::Mul => Value::Int(expr1 * expr2),
//                     Oprim::Div => Value::Int(expr1 / expr2),
//                     Oprim::Sub => Value::Int(expr1 - expr2),
//                     Oprim::Eq => Value::Int(Value::to_int(expr1 == expr2)),
//                     Oprim::Lt => Value::Int(Value::to_int(expr1 < expr2)),
//                     Oprim::And => Value::Int(Value::to_int(
//                         Value::to_bool(expr1) && Value::to_bool(expr2),
//                     )),
//                     Oprim::Or => Value::Int(Value::to_int(
//                         Value::to_bool(expr1) || Value::to_bool(expr2),
//                     )),
//                 }
//             }
//             ASTIf(condition, then, _else) => {
//                 let type_condition = condition.type_check(type_cache);
//                 let type_then = then.type_check(type_cache);
//                 let type_else = _else.type_check(type_cache);

//                 if
//             }

//             ASTIdent(x) => {
//                 let id = env.get(x).expect("Not in environment");
//                 match id {
//                     Value::Adress(adr) => mem.mem[*adr].clone(),
//                     _ => id.clone(),
//                 }
//             }

//             ASTApp(e, args) => match &env[e] {
//                 Value::Fermeture(fbody, fargs, fenv) => {
//                     let mut nfenv = env.clone();

//                     for (f, value) in fenv {
//                         nfenv.insert(f.clone(), value.clone());
//                     }

//                     for i in 0..fargs.len() {
//                         nfenv.insert(fargs[i].clone(), args[i].eval(env, mem));
//                     }

//                     fbody.eval(&nfenv, mem)
//                 }

//                 Value::FermetureRec(fbody, fargs, fenv) => {
//                     let mut nfenv = fenv.clone();
//                     for i in 0..fargs.len() {
//                         nfenv.insert(fargs[i].clone(), args[i].eval(env, mem));
//                     }
//                     fbody.eval(&nfenv, mem)
//                 }

//                 _ => panic!("not a fermeture"),
//             },

//             ASTAbs(args, e) => {
//                 let mut abs_args = Vec::new();

//                 for arg in args {
//                     abs_args.push(arg.ident.clone());
//                 }
//                 Value::Fermeture(e.clone(), abs_args, env.clone())
//             }

//             // rum2
//             ASTAlloc(e) => match e.eval(&env, mem) {
//                 Value::Int(n) => mem.allocn(n as usize),
//                 _ => panic!(format!(" {:?} is not a Number", e)),
//             },
//             ASTNth(e1, e2) => match e1.eval(&env, mem) {
//                 Value::Block(adr, n) => match e2.eval(&env, mem) {
//                     Value::Int(i) => {
//                         if i > n as i64 {
//                             panic!("ASTNTH out of the block");
//                         }
//                         mem.mem[adr + i as usize].clone()
//                     }
//                     _ => panic!(format!(" {:?} is not a Number", e2)),
//                 },
//                 _ => panic!(format!(" {:?} is not a Block", e1)),
//             },
//             ASTLen(e) => match e.eval(&env, mem) {
//                 Value::Block(_, n) => Value::Int(n as i64),
//                 _ => panic!(format!(" {:?} is not a Block", e)),
//             },
//         }
//     }
// }
