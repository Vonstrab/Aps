use crate::ast::AstCdms;
use crate::ast::AstDec;
use crate::ast::AstExp;
use crate::ast::Oprim;

use crate::rum_type::Type;

use std::collections::HashMap;



impl AstExp {
    pub fn type_check(&self, type_cache: &HashMap<String, Type>) -> Type {

        use AstExp::*;

        match self {
            ASTInt(_) => Type::Int,
            ASTBool(_) => Type::Bool,
            ASTUnPrim(_, exp) => {
                let exp_type = exp.type_check(type_cache);
                if exp_type == Type::Bool {
                    Type::Bool
                } else {
                    Type::TypeError(format!("Not primitive function require a boolean argument"))
                }
            }

            ASTPrint(e1) => {
                let exp_type = e1.type_check(type_cache);
                if  !exp_type.is_err() {
                    return Type::Void;
                } else {
                    return exp_type;
                }
            }
            ASTBinPrim(op, e1, e2) => {
                let expr1 = e1.type_check(type_cache);
                let expr2 = e2.type_check(type_cache);

                if expr1.is_err() || expr2.is_err() {
                    return Type::TypeError(format!("TODO"));
                }

                match op {
                    Oprim::Add => {
                        if expr1 != Type::Int || expr2 != Type::Int {
                            Type::TypeError(format!("TODO"))
                        } else {
                            Type::Int
                        }
                    }
                    Oprim::Mul => {
                        if expr1 != Type::Int || expr2 != Type::Int {
                            Type::TypeError(format!("TODO"))
                        } else {
                            Type::Int
                        }
                    }
                    Oprim::Div => {
                        if expr1 != Type::Int || expr2 != Type::Int {
                            Type::TypeError(format!("TODO"))
                        } else {
                            Type::Int
                        }
                    }
                    Oprim::Sub => {
                        if expr1 != Type::Int || expr2 != Type::Int {
                            Type::TypeError(format!("TODO"))
                        } else {
                            Type::Int
                        }
                    }
                    Oprim::Eq => {
                        if expr1 != Type::Bool || expr2 != Type::Bool {
                            Type::TypeError(format!("TODO"))
                        } else {
                            Type::Int
                        }
                    }
                    Oprim::Lt => {
                        if expr1 != Type::Bool || expr2 != Type::Bool {
                            Type::TypeError(format!("TODO"))
                        } else {
                            Type::Int
                        }
                    }
                    Oprim::And => {
                        if expr1 != Type::Bool || expr2 != Type::Bool {
                            Type::TypeError(format!("TODO"))
                        } else {
                            Type::Int
                        }
                    }
                    Oprim::Or => {
                        if expr1 != Type::Bool || expr2 != Type::Bool {
                            Type::TypeError(format!("TODO"))
                        } else {
                            Type::Int
                        }
                    }
                }
            }

            ASTIf(condition, then, _else) => {
                let type_condition = condition.type_check(type_cache);
                let type_then = then.type_check(type_cache);
                let type_else = _else.type_check(type_cache);

                if type_condition != Type::Bool {
                    return Type::TypeError(format!("TODO"));
                }

                if type_then != type_else {
                    return Type::TypeError(format!("TODO"));
                }

                type_then
            }

            ASTIdent(x) => type_cache.get(x).expect("Not in environment").clone(),

            ASTApp(e, args) => {
                let func_type = type_cache.get(e).expect("Not in environment").clone();
                let mut type_args = Vec::with_capacity(args.len());

                for arg in args {
                    type_args.push(arg.type_check(type_cache));
                }

                Type::check_fun(func_type, type_args)
            }

            _ => panic!("not yet, come back later"),
        }
    }
}

impl AstDec {
    pub fn type_check(&self, type_cache: &mut HashMap<String, Type>) -> Type {
        use crate::ast::AstDec::*;

        match self {
          
            ASTFunc(name , fn_type, args, expr) => {
                
                for arg in args {
                
                    type_cache.insert(arg.ident.clone(), arg.id_type.clone());
                    
                }

                let exp_type = expr.type_check(type_cache);

                if *fn_type == exp_type {
                    type_cache.insert(name.to_string(), Type::Func(vec![], Box::new(fn_type.clone())));
                    return Type::Void;
                }

                return Type::TypeError("function declaration is wrong".to_string());
            }
            // ASTFuncRec(x, t, a, e) => {
            // }

            // ASTVar(var, t) => {}
            ASTConst(var, t, exp) => {
                let exp_type = exp.type_check(type_cache);

                if *t == exp_type {
                    type_cache.insert(var.to_string(), t.clone());
                    return Type::Void;
                }
                return Type::TypeError("const declaration is wrong".to_string());
            }
            _ => panic!("not yet, come back later"),
        }
    }
}

impl AstCdms {
    pub fn type_check(&self, type_cache: &mut HashMap<String, Type>) -> Type {
        use crate::ast::AstCdms::*;

        match self {
            FExp(e) => e.type_check(type_cache),
            Exp(e, cs) => {
                e.type_check(type_cache);
                cs.type_check(type_cache)
            }
            Dec(d, cs) => {
                d.type_check(type_cache);
                cs.type_check(type_cache)
            }
        }
    }
}

impl Type {
    pub fn type_check (prog : & AstCdms ) -> Type {
        prog.type_check(& mut HashMap::new())
    }
}