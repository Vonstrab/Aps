use std::fmt::{Error, Formatter};
// use std::hash::Hash;

use super::rum_type::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnOprim {
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
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
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AstExp {
    ASTPrint(Box<AstExp>),
    ASTIdent(String),
    ASTInt(i64),
    ASTBool(bool),
    ASTIf(Box<AstExp>, Box<AstExp>, Box<AstExp>),
    ASTUnPrim(UnOprim, Box<AstExp>),
    ASTBinPrim(Oprim, Box<AstExp>, Box<AstExp>),
    ASTApp(String, Vec<Box<AstExp>>),
    ASTAbs(Vec<Arg>, Box<AstExp>),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AstDec {
    ASTConst(String, Type, Box<AstExp>),
    ASTFunc(String, Type, Vec<Arg>, Box<AstExp>),
    ASTFuncRec(String, Type, Vec<Arg>, Box<AstExp>),
    ASTVar(String, Type),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AstStat {
    ASTCall(String, Vec<Box<AstExp>>),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AstCdms {
    FStat(Box<AstStat>),
    Dec(Box<AstDec>, Box<AstCdms>),
    Stat(Box<AstStat>, Box<AstCdms>),
}

impl std::fmt::Debug for AstExp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::AstExp::*;
        match self {
            ASTPrint(e1) => write!(fmt, "ASTPrint ( {:?} )", e1),
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
        }
    }
}

impl std::fmt::Debug for AstCdms {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::AstCdms::*;
        match self {
            Stat(s, cs) => write!(fmt, "ASTCMDS {:?}, {:?}", s, cs),
            Dec(d, cs) => write!(fmt, "ASTCMDS {:?}, {:?}", d, cs),
            FStat(s) => write!(fmt, "FSTAT( {:?} )", s),
        }
    }
}
