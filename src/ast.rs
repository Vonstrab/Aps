use std::collections::HashMap;
use std::fmt::{Error, Formatter};

use super::aps_type::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum UnOprim {
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Oprim {
    Eq,
    Lt,
    And,
    Add,
    Mul,
    Div,
    Sub,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Arg {
    pub ident: String,
    pub id_type: Type,
}

impl Arg {
    pub fn new(id: &String, id_t: &Type) -> Arg {
        Arg {
            ident: id.clone(),
            id_type: id_t.clone(),
        }
    }

    pub fn to_prolog(&self) -> String {
        format!("({} , {})", self.ident, self.id_type)
    }
}

#[derive(Clone, PartialEq)]
pub enum AstExp {
    ASTIdent(String),
    ASTInt(i64),
    ASTBool(bool),
    ASTIf(Box<AstExp>, Box<AstExp>, Box<AstExp>),
    ASTUnPrim(UnOprim, Box<AstExp>),
    ASTBinPrim(Oprim, Box<AstExp>, Box<AstExp>),
    ASTApp(Box<AstExp>, Vec<Box<AstExp>>),
    ASTAbs(Vec<Arg>, Box<AstExp>),
}

#[derive(Clone)]
pub enum AstDec {
    ASTConst(String, Type, Box<AstExp>),
    ASTFunc(String, Type, Vec<Arg>, Box<AstExp>),
    ASTFuncRec(String, Type, Vec<Arg>, Box<AstExp>),
    ASTVar(String, Type),
    ASTProc(String, Vec<Arg>, Box<AstCdms>),
    ASTProcRec(String, Vec<Arg>, Box<AstCdms>),
}

#[derive(Clone)]
pub enum AstStat {
    ASTEcho(Box<AstExp>),
    ASTSet(String, Box<AstExp>),
    ASTIf(Box<AstExp>, Box<AstCdms>, Box<AstCdms>),
    ASTWhile(Box<AstExp>, Box<AstCdms>),
    ASTCall(String, Vec<Box<AstExp>>),
}

#[derive(Clone)]
pub enum AstCdms {
    FStat(Box<AstStat>),
    Dec(Box<AstDec>, Box<AstCdms>),
    Stat(Box<AstStat>, Box<AstCdms>),
}

impl std::fmt::Debug for AstExp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::AstExp::*;
        match self {
            ASTBool(b) => write!(fmt, "ASTBool({:?})", b),
            ASTInt(n) => write!(fmt, "ASTInt({:?})", n),
            ASTUnPrim(op, e1) => write!(fmt, "ASTPrim( {:?}, {:?} )", op, e1),
            ASTBinPrim(op, e1, e2) => write!(fmt, "ASTBinPrim( {:?} , {:?} , {:?} )", op, e1, e2),
            ASTAbs(x, e1) => write!(fmt, "ASTAbs( {:?} , {:?} )", x, e1),
            ASTApp(x, e1) => write!(fmt, "ASTApp( {:?} ,{:?})", x, e1),
            ASTIf(e1, e2, e3) => write!(fmt, "ASTif( {:?} ,{:?}, {:?})", e1, e2, e3),
            ASTIdent(id) => write!(fmt, "ASTIdent( {:?} )", id),
        }
    }
}

impl std::fmt::Debug for AstStat {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::AstStat::*;
        match self {
            ASTEcho(e1) => write!(fmt, "ASTEcho ( {:?} )", e1),
            ASTSet(x, e1) => write!(fmt, "ASTSet( {:?} , {:?} )", x, e1),
            ASTIf(e1, bl1, bl2) => write!(fmt, "ASTIF( {:?} , {:?} , {:?} )", e1, bl1, bl2),
            ASTWhile(e1, bl) => write!(fmt, "ASTWHILE( {:?} , {:?} )", e1, bl),
            ASTCall(x, exps) => write!(fmt, "ASTCall( {:?} , {:?} )", x, exps),
        }
    }
}

impl std::fmt::Debug for AstDec {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::AstDec::*;
        match self {
            ASTConst(s, t, e) => write!(fmt, "ASTConst ( {:?} , {:?} , {:?} )", s, t, e),
            ASTFunc(id, t, args, e) => {
                write!(fmt, "ASTFunc ( {:?} , {:?} , {:?} , {:?} )", id, t, args, e)
            }
            ASTFuncRec(id, t, args, e) => write!(
                fmt,
                "ASTFuncRec ( {:?} , {:?} , {:?} , {:?} )",
                id, t, args, e
            ),
            ASTVar(x, t) => write!(fmt, "ASTVar ( {:?} ,{:?} )", x, t),
            ASTProc(id, args, e) => write!(fmt, "ASTProc ( {:?} , {:?} , {:?} )", id, args, e),
            ASTProcRec(id, args, e) => {
                write!(fmt, "ASTProcRec ( {:?}  , {:?} , {:?} )", id, args, e)
            }
        }
    }
}

impl std::fmt::Debug for AstCdms {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::AstCdms::*;
        match self {
            Stat(s, cs) => write!(fmt, "ASTCMDS {:?}, {:?}", s, cs),
            Dec(d, cs) => write!(fmt, "ASTCMDS {:?}, {:?}", d, cs),
            FStat(s) => write!(fmt, "{:?}", s),
        }
    }
}

impl AstExp {
    pub fn to_prolog(&self) -> String {
        use self::AstExp::*;
        let mut out: String = String::new();
        match self {
            ASTInt(n) => {
                let s = format!("integer({})", n);
                out.push_str(s.as_str());
            }
            ASTBool(b) => {
                let s = format!("{}", b);
                out.push_str(s.as_str());
            }
            ASTUnPrim(_op, e1) => {
                let s = format!("unOp( {} ))", e1.to_prolog());
                out.push_str(s.as_str());
            }
            ASTBinPrim(op, e1, e2) => match op {
                Oprim::Add => {
                    let s = format!("binOpInt( {} , {} )", e1.to_prolog(), e2.to_prolog());
                    out.push_str(s.as_str());
                }
                Oprim::Mul => {
                    let s = format!("binOpInt( {} , {} )", e1.to_prolog(), e2.to_prolog());
                    out.push_str(s.as_str());
                }
                Oprim::Div => {
                    let s = format!("binOpInt( {} , {} )", e1.to_prolog(), e2.to_prolog());
                    out.push_str(s.as_str());
                }
                Oprim::Sub => {
                    let s = format!("binOpInt( {} , {} )", e1.to_prolog(), e2.to_prolog());
                    out.push_str(s.as_str());
                }
                Oprim::Eq => {
                    let s = format!("binOpIntBool( {} , {} )", e1.to_prolog(), e2.to_prolog());
                    out.push_str(s.as_str());
                }
                Oprim::Lt => {
                    let s = format!("binOpIntBool( {} , {} )", e1.to_prolog(), e2.to_prolog());
                    out.push_str(s.as_str());
                }
                Oprim::And => {
                    let s = format!("binOpBool( {} , {} )", e1.to_prolog(), e2.to_prolog());
                    out.push_str(s.as_str());
                }
                Oprim::Or => {
                    let s = format!("binOpBool( {} , {} )", e1.to_prolog(), e2.to_prolog());
                    out.push_str(s.as_str());
                }
            },
            ASTIf(e1, e2, e3) => {
                let s = format!(
                    "exprIf({},{},{})",
                    e1.to_prolog(),
                    e2.to_prolog(),
                    e3.to_prolog()
                );
                out.push_str(s.as_str());
            }
            ASTIdent(s1) => {
                let s = format!("id({})", s1);
                out.push_str(s.as_str());
            }
            ASTAbs(a, e1) => {
                let mut args = String::new();
                args.push('[');
                for val in a {
                    args.push_str(val.to_prolog().as_str());
                    if val != a.last().unwrap() {
                        args.push('|');
                    }
                }
                args.push(']');
                let s = format!("abs({},{})", args, e1.to_prolog());
                out.push_str(s.as_str());
            }
            ASTApp(e1, e) => {
                let mut exprs = String::new();
                exprs.push('[');
                for val in e {
                    exprs.push_str(val.to_prolog().as_str());
                    if val != e.last().unwrap() {
                        exprs.push('|');
                    }
                }
                exprs.push(']');
                let s = format!("app({},{})", e1.to_prolog(), exprs);
                out.push_str(s.as_str());
            }
        }
        out
    }
}

impl AstDec {
    pub fn to_prolog(&self) -> String {
        use self::AstDec::*;

        let mut out: String = String::new();
        match self {
            ASTProc(x, a, e) => {
                let mut args = String::new();
                args.push('[');
                for val in a {
                    args.push_str(val.to_prolog().as_str());
                    if val != a.last().unwrap() {
                        args.push('|');
                    }
                }
                args.push(']');

                let s = format!("dec(proc({},{},{}))", x, args, e.to_prolog());
                out.push_str(s.as_str());
            }
            ASTProcRec(x, a, e) => {
                let mut args = String::new();
                args.push('[');
                for val in a {
                    args.push_str(val.to_prolog().as_str());
                    if val != a.last().unwrap() {
                        args.push('|');
                    }
                }
                args.push(']');

                let s = format!("dec(procRec({},{},{}))", x, args, e.to_prolog());
                out.push_str(s.as_str());
            }
            ASTFunc(x, t, a, e) => {
                let mut args = String::new();
                args.push('[');
                for val in a {
                    args.push_str(val.to_prolog().as_str());
                    if val != a.last().unwrap() {
                        args.push('|');
                    }
                }
                args.push(']');

                let s = format!("dec(fonction({},{:?},{},{}))", x, t, args, e.to_prolog());
                out.push_str(s.as_str());
            }
            ASTFuncRec(x, t, a, e) => {
                let mut args = String::new();
                args.push('[');
                for val in a {
                    args.push_str(val.to_prolog().as_str());
                    if val != a.last().unwrap() {
                        args.push('|');
                    }
                }
                args.push(']');

                let s = format!("dec(fonctionRec({},{:?},{},{}))", x, t, args, e.to_prolog());
                out.push_str(s.as_str());
            }

            ASTVar(var, t) => {
                let s = format!("dec(var({}, {}))", var, t);
                out.push_str(s.as_str());
            }

            ASTConst(var, t, exp) => {
                let s = format!("dec(const({}, {:?}, {}))", var, t, exp.to_prolog());
                out.push_str(s.as_str());
            }
        }
        out
    }
}

impl AstStat {
    pub fn to_prolog(&self) -> String {
        use self::AstStat::*;
        let mut out: String = String::new();
        match self {
            ASTEcho(e1) => {
                let s = format!("stat(echo({}))", e1.to_prolog());
                out.push_str(s.as_str());
            }
            ASTIf(e1, bl1, bl2) => {
                let s = format!(
                    "stat(statIf({},{},{}))",
                    e1.to_prolog(),
                    bl1.to_prolog(),
                    bl2.to_prolog()
                );
                out.push_str(s.as_str());
            }
            ASTWhile(e1, bl) => {
                let s = format!("stat(while({},{}))", e1.to_prolog(), bl.to_prolog(),);
                out.push_str(s.as_str());
            }
            ASTCall(s, e) => {
                let mut exprs = String::new();
                exprs.push('[');
                for val in e {
                    exprs.push_str(val.to_prolog().as_str());
                    if val != e.last().unwrap() {
                        exprs.push('|');
                    }
                }
                exprs.push(']');
                let s = format!("stat(call({},{}))", s, exprs);
                out.push_str(s.as_str());
            }
            ASTSet(s, e) => {
                let s = format!("stat(set({},{}))", s, e.to_prolog());
                out.push_str(s.as_str());
            }
        }
        out
    }
}

impl AstCdms {
    pub fn to_prolog(&self) -> String {
        use self::AstCdms::*;
        let mut out: String = String::new();
        match self {
            FStat(s) => {
                let s = format!("{}", s.to_prolog());
                out.push_str(s.as_str());
            }
            Stat(s, cs) => {
                let s = format!("cdms({},{})", s.to_prolog(), cs.to_prolog());
                out.push_str(s.as_str());
            }
            Dec(d, cs) => {
                let s = format!("cdms({},{})", d.to_prolog(), cs.to_prolog());
                out.push_str(s.as_str());
            }
        }
        out
    }
}
