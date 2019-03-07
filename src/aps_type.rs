use std::fmt::{Error, Formatter};

#[derive(PartialEq, Clone)]
pub enum Type {
    Int,
    Bool,
    Void,
    Func(Vec<Type>, Box<Type>),
}

impl std::fmt::Debug for Type {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Type::*;
        match self {
            Int => write!(fmt, "int"),
            Void => write!(fmt, "int"),
            Bool => write!(fmt, "bool"),

            Func(args, retour) => {
                for (i, arg) in args.iter().enumerate() {
                    let _r = write!(fmt, "{:?}", arg);
                    if i != 0 {
                        let _r2 = write!(fmt, " * ");
                    }
                }
                write!(fmt, " -> {:?}", retour)
            }
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Type::*;
        match self {
            Int => write!(fmt, "int"),
            Void => write!(fmt, "int"),
            Bool => write!(fmt, "bool"),

            Func(args, retour) => {
                for (i, arg) in args.iter().enumerate() {
                    let _r = write!(fmt, "{}", arg);
                    if i != 0 {
                        let _r2 = write!(fmt, " * ");
                    }
                }
                write!(fmt, " -> {}", retour)
            }
        }
    }
}
