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
pub enum AstCdms {
    FExp(Box<AstExp>),
    Dec(Box<AstDec>, Box<AstCdms>),
    Exp(Box<AstExp>, Box<AstCdms>),
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


impl std::fmt::Debug for AstDec {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::AstDec::*;
        match self {
            ASTConst(s, t, e) => write!(fmt, "ASTConst (\n\t{:?},\n\t{:?},\n\t{:?})", s, t, e),
            ASTFunc(id, t, args, e) => {
                write!(fmt, "ASTFunc (\n\t{:?}\n\t,{:?}\n\t,{:?},\n\t{:?})", id, t, args, e)
            }
            ASTFuncRec(id, t, args, e) => write!(
                fmt,
                "ASTFuncRec ( \n\t{:?},\n\t{:?},\n\t{:?},\n\t{:?})",
                id, t, args, e
            ),
            ASTVar(x, t) => write!(fmt, "ASTVar (\n\t{:?}\n\t,{:?})", x, t),
        }
    }
}

impl std::fmt::Debug for AstCdms {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::AstCdms::*;
        match self {
            Exp(s, cs) => write!(fmt, "ASTCMDS \n\t{:?}\n\t,{:?}", s, cs),
            Dec(d, cs) => write!(fmt, "ASTCMDS \n\t{:?}\n\t,{:?}", d, cs),
            FExp(s) => write!(fmt, "FSTAT(\n\t{:?})", s),
        }
    }
}
