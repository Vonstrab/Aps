use std::fmt::{Error, Formatter};

#[derive(PartialEq, PartialOrd, Clone, Eq, Hash)]
pub enum Type {
    Int,
    Bool,
    Void,
    Func(Vec<Type>, Box<Type>),
    Vector(Box<Type>),
    TypeError(String),
}

impl Type {
    pub fn is_err(&self) -> bool {
        match self {
            Type::TypeError(_) => true,
            _ => false,
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            Type::TypeError(_) => false,
            _ => true,
        }
    }

    pub fn err_message(&self) -> String {
        match self {
            Type::TypeError(err_msg) => err_msg.clone(),
            _ => panic!("the type is not a type error"),
        }
    }

    pub fn check_fun(func_type: Type, type_args: Vec<Type>) -> Type {
        match &func_type {
            Type::Func(args , _ )  => {
                for i in 0..args.len() {
                    if args[i] != type_args[i] {
                        return Type::TypeError(
                            "la signature correspond pas aux argumements".to_string(),
                        );
                    }
                }
                return func_type;
            }
            _ => panic!("we expect a function"),
        }
    }
}

impl std::fmt::Debug for Type {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Type::*;
        match self {
            Int => write!(fmt, "int"),
            Void => write!(fmt, "int"), //TODO Check if its right
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
            Vector(t) => write!(fmt, "vec( {:?} )", t),
            TypeError(error_text) => write!(fmt, "Type Error( {:?} )", error_text),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Type::*;
        match self {
            Int => write!(fmt, "int"),
            Void => write!(fmt, "int"), //TODO Check if its right
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
            Vector(t) => write!(fmt, "vec( {} )", t),
            TypeError(error_text) => write!(fmt, "Type Error( {} )", error_text),
        }
    }
}
