use crate::ast;

use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Int(i64),
    FermetureProc(Box<ast::AstCdms>, Vec<String>, HashMap<String, Value>),
    FermetureProcRec(Box<ast::AstCdms>, Vec<String>, HashMap<String, Value>),
    Fermeture(Box<ast::AstExp>, Vec<String>, HashMap<String, Value>),
    FermetureRec(Box<ast::AstExp>, Vec<String>, HashMap<String, Value>),
    Adress(usize),
    Any,
}

impl Value {
    pub fn as_int(&self, mem: &Vec<Value>) -> i64 {
        use self::Value::*;
        match self {
            Int(i) => *i,
            Adress(a) => mem[*a].as_int(mem),
            Any => panic!("variable not initialised"),
            _ => panic!("not a int"),
        }
    }

    pub fn to_bool(i: i64) -> bool {
        match i {
            1 => true,
            0 => false,
            _ => panic!("not a bool"),
        }
    }

    pub fn to_int(b: bool) -> i64 {
        match b {
            true => 1,
            false => 0,
        }
    }
}

impl ast::AstCdms {
    pub fn eval(&self, env: &mut HashMap<String, Value>, mem: &mut Vec<Value>) -> Vec<i64> {
        println!("\nInto CDMS eval");
        println!("self {:?}", self);
        println!("env {:?}", env);
        println!("mem {:?}", mem);

        use ast::AstCdms::*;
        let mut flux_sortie: Vec<i64> = Vec::new();

        match self {
            Dec(dec, cdms) => {
                dec.eval(env, mem);
                flux_sortie.append(&mut cdms.eval(env, mem));
            }
            Stat(stat, cdms) => {
                flux_sortie.append(&mut stat.eval(env, mem));
                flux_sortie.append(&mut cdms.eval(env, mem));
            }

            FStat(stat) => {
                flux_sortie.append(&mut stat.eval(env, mem));
            }
        }
        flux_sortie
    }
}

impl ast::AstDec {
    pub fn eval(&self, env: &mut HashMap<String, Value>, mem: &mut Vec<Value>) {
        println!("\nInto DEC eval");
        println!("self {:?}", self);
        println!("env {:?}", env);
        println!("mem {:?}", mem);

        use ast::AstDec::*;

        match self {
            ASTConst(ident, _, exp) => {
                env.insert(ident.clone(), exp.eval(&env, mem));
                println!("On ajoute {} a l'env", ident);
            }

            ASTFunc(fname, _, a, e) => {
                let mut args = Vec::new();
                for arg in a {
                    args.push(arg.ident.clone());
                }
                let fenv = env.clone();
                env.insert(fname.clone(), Value::Fermeture(e.clone(), args, fenv));
            }

            ASTFuncRec(x, _, a, e) => {
                let mut args = Vec::new();

                for arg in a {
                    args.push(arg.ident.clone());
                }

                let mut fenv = env.clone();
                let mut ferm = Value::Fermeture(e.clone(), args.clone(), fenv.clone());
                fenv.insert(x.clone(), ferm.clone());
                ferm = Value::Fermeture(e.clone(), args.clone(), fenv.clone());
                env.insert(x.clone(), ferm.clone());
            }

            ASTVar(s, _) => {
                mem.push(Value::Any);
                env.insert(s.clone(), Value::Adress(mem.len() - 1));
            }

            ASTProc(fname, a, e) => {
                let mut args = Vec::new();
                for arg in a {
                    args.push(arg.ident.clone());
                }
                let fenv = env.clone();
                env.insert(fname.clone(), Value::FermetureProc(e.clone(), args, fenv));
            }
            ASTProcRec(x, a, e) => {
                let mut args = Vec::new();

                for arg in a {
                    args.push(arg.ident.clone());
                }

                let mut fenv = env.clone();
                let mut ferm = Value::FermetureProcRec(e.clone(), args.clone(), fenv.clone());
                fenv.insert(x.clone(), ferm.clone());
                ferm = Value::FermetureProcRec(e.clone(), args.clone(), fenv.clone());
                env.insert(x.clone(), ferm.clone());
            }
        }
    }
}

impl ast::AstStat {
    pub fn eval(&self, env: &mut HashMap<String, Value>, mem: &mut Vec<Value>) -> Vec<i64> {
        println!("\nInto Stat eval");
        println!("self {:?}", self);
        println!("env {:?}", env);
        println!("mem {:?}", mem);

        use ast::AstStat::*;
        let mut flux_sortie: Vec<i64> = Vec::new();

        match self {
            ASTEcho(expr) => {
                let value = expr.eval(env, mem);
                flux_sortie.push(value.as_int(mem))
            }
            ASTSet(s, exp) => {}//match &env[s] {
            //     Value::Adress(a) => {
            //         let adr = *a;
            //         mem[adr] = exp.eval(env, mem);
            //     }
            //     _ => {
            //         panic!("NOT An Adress");
            //     }
            // },
            ASTIf(e, el, th) => {
                if e.eval(env, mem).as_int(mem) == 1 {
                    flux_sortie.append(&mut el.eval(env, mem));
                } else {
                    flux_sortie.append(&mut th.eval(env, mem));
                }
            }
            ASTWhile(e, lop) => {
                while e.eval(env, mem).as_int(mem) == 1 {
                    flux_sortie.append(&mut lop.eval(env, mem));
                }
            }
            ASTCall(s, args) => match &env[s] {
                Value::FermetureProc(fbody, fargs, fenv) => {
                    let mut nfenv = env.clone();

                    for (f, value) in fenv {
                        nfenv.insert(f.clone(), value.clone());
                    }

                    for i in 0..fargs.len() {
                        nfenv.insert(fargs[i].clone(), args[i].eval(env, mem));
                    }

                    flux_sortie.append(&mut fbody.eval(&mut nfenv, mem));
                }

                Value::FermetureProcRec(fbody, fargs, fenv) => {
                    let mut nfenv = env.clone();

                    for (f, value) in fenv {
                        nfenv.insert(f.clone(), value.clone());
                    }

                    for i in 0..fargs.len() {
                        nfenv.insert(fargs[i].clone(), args[i].eval(env, mem));
                    }

                    flux_sortie.append(&mut fbody.eval(&mut nfenv, mem));
                }
                _ => panic!("not a proc"),
            },
            
        }

        println!("FLUX SORTIE : {:?} ", flux_sortie);
        flux_sortie
    }
}

impl ast::AstExp {
    pub fn eval(&self, env: &HashMap<String, Value>, mem: &Vec<Value>) -> Value {
        println!("\nInto Expr eval");
        println!("self {:?}", self);
        println!("env {:?}", env);
        println!("mem {:?}", mem);

        use ast::AstExp::*;

        match self {
            ASTInt(n) => Value::Int(*n),
            ASTBool(b) => Value::Int(Value::to_int(*b)),
            ASTUnPrim(_, e) => {
                let exp = e.eval(env, mem);
                if exp == Value::Int(1) {
                    Value::Int(0)
                } else {
                    Value::Int(1)
                }
            }
            ASTBinPrim(op, e1, e2) => {
                use ast::Oprim;

                let expr1 = e1.eval(env, mem).as_int(mem);
                let expr2 = e2.eval(env, mem).as_int(mem);
                match op {
                    Oprim::Add => Value::Int(expr1 + expr2),
                    Oprim::Mul => Value::Int(expr1 * expr2),
                    Oprim::Div => Value::Int(expr1 / expr2),
                    Oprim::Sub => Value::Int(expr1 - expr2),
                    Oprim::Eq => Value::Int(Value::to_int(expr1 == expr2)),
                    Oprim::Lt => Value::Int(Value::to_int(expr1 < expr2)),
                    Oprim::And => Value::Int(Value::to_int(
                        Value::to_bool(expr1) && Value::to_bool(expr2),
                    )),
                    Oprim::Or => Value::Int(Value::to_int(
                        Value::to_bool(expr1) || Value::to_bool(expr2),
                    )),
                }
            }
            ASTIf(e1, e2, e3) => {
                if e1.eval(env, mem).as_int(mem) == 1 {
                    e2.eval(env, mem)
                } else {
                    e3.eval(env, mem)
                }
            }

            ASTIdent(x) => {
                let id = env.get(x).expect("Not in environment");
                id.clone()
            }

            ASTApp(e, args) => match &env[e] {
                Value::Fermeture(fbody, fargs, fenv) => {
                    let mut nfenv = env.clone();

                    for (f, value) in fenv {
                        nfenv.insert(f.clone(), value.clone());
                    }

                    for i in 0..fargs.len() {
                        nfenv.insert(fargs[i].clone(), args[i].eval(env, mem));
                    }

                    fbody.eval(&nfenv, mem)
                }

                Value::FermetureRec(fbody, fargs, fenv) => {
                    let mut nfenv = fenv.clone();
                    for i in 0..fargs.len() {
                        nfenv.insert(fargs[i].clone(), args[i].eval(env, mem));
                    }
                    fbody.eval(&nfenv, mem)
                }

                _ => panic!("not a fermeture"),
            },

            ASTAbs(args, e) => {
                let mut abs_args = Vec::new();

                for arg in args {
                    abs_args.push(arg.ident.clone());
                }
                Value::Fermeture(e.clone(), abs_args, env.clone())
            }
            ASTLen(e) =>{Value::Any},
            ASTAlloc(e) => {Value::Any},
            ASTNth(e1,e2) => {Value::Any}
        }
    }
}
