
#[derive(Clone)]
pub enum Value {
    Int(i64),
    Fermeture(Box<Ast>, Vec<String>, HashMap<String, Value>),
}


pub fn eval(ast : ast::Ast, env: &mut HashMap<String, Value>) -> i64 {
        use self::Ast::*;

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

        match self {
            ASTEntier(n) => *n,
            ASTFalse() => 0,
            ASTTrue() => 1,
            ASTUnPrim(_, e) => {
                let exp = e.eval(env);
                if exp == 1 {
                    0
                } else {
                    1
                }
            }
            ASTEcho(e) => {
                let exp = e.eval(env);
                println!("{}", exp);
                0
            }
            ASTBinPrim(op, e1, e2) => {
                let expr1 = e1.eval(env);
                let expr2 = e2.eval(env);
                match op {
                    Oprim::Add => expr1 + expr2,
                    Oprim::Mul => expr1 * expr2,
                    Oprim::Div => expr1 / expr2,
                    Oprim::Sub => expr1 - expr2,
                    Oprim::Eq => to_int(expr1 == expr2),
                    Oprim::Lt => to_int(expr1 < expr2),
                    Oprim::And => to_int(to_bool(expr1) && to_bool(expr2)),
                    Oprim::Or => to_int(to_bool(expr1) || to_bool(expr2)),
                }
            }
            ASTConst(x, _ty, e) => {
                let e = e.eval(env);
                env.insert(x.clone(), Value::Int(e));
                0
            }
            ASTCdms(d, cds) => {
                let _ = d.eval(env);
                cds.eval(env)
            }
            ASTIf(e1, e2, e3) => {
                if e1.eval(env) == 1 {
                    e2.eval(env)
                } else {
                    e3.eval(env)
                }
            }
            ASTPair(e1, e2) => match e1.as_ref() {
                Ast::ASTIdent(x) => match &env[&x.clone()] {
                    Value::Fermeture(e, args, fenv) => {
                        let mut nfenv = fenv.clone();
                        nfenv.insert(args[0].clone(), Value::Int(e2.eval(& mut env.clone())));
                        e.eval(&mut nfenv)
                    }
                    Value::Int(e) => *e,
                },
                _ => {
                    e1.eval(env);
                    e2.eval(env)
                }
            },
            ASTIdent(x) => match &env[x] {
                Value::Fermeture(_e, _args, _fenv) => 0,
                Value::Int(e) => *e,
            },
            ASTFunc(x, t, a, e) => {
                let mut args = Vec::new();
                for arg in a {
                    match arg {
                        Arg::Arg((x, t)) => args.push(x.clone()),
                    }
                }

                let fenv = env.clone();
                env.insert(x.clone(), Value::Fermeture(e.clone(), args, fenv));
                0
            }
            ASTFuncRec(x, t, a, e) => {
                let mut args = Vec::new();
                for arg in a {
                    match arg {
                        Arg::Arg((x, t)) => args.push(x.clone()),
                    }
                }

                let mut fenv = env.clone();
                let ferm =Value::Fermeture(e.clone(), args.clone(), fenv.clone());
                fenv.insert(x.clone(), ferm.clone());
                env.insert(x.clone(),ferm.clone()) ;
                0
            }

            _ => {
                panic!("NOT YET");
            }
        }
    }