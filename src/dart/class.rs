use std::fmt;

use super::{Ident, Type};

mod method;
pub use method::Method;

pub struct Class {
    pub name: Ident,
    pub variables: Vec<Variable>,
    pub constructors: Vec<Method>,
    pub methods: Vec<Method>,
}

impl Class {
    pub fn new(name: Ident) -> Self {
        Self {
            name,
            variables: Vec::new(),
            constructors: Vec::new(),
            methods: Vec::new(),
        }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "class {name} {{", name = self.name)?;

        for variable in &self.variables {
            writeln!(f, "{variable}")?;
        }

        writeln!(f)?;

        for constructor in &self.constructors {
            writeln!(f, "{constructor}")?;
        }

        writeln!(f)?;

        for method in &self.methods {
            writeln!(f, "{method}")?;
        }

        write!(f, "}}")?;

        Ok(())
    }
}

pub enum Member {
    Variable(Variable),
    Method(Method),
}

impl fmt::Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Member::Variable(variable) => variable.fmt(f),
            Member::Method(method) => method.fmt(f),
        }
    }
}

pub struct Variable {
    pub name: Ident,
    pub ty: Type,
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {};", self.ty, self.name)
    }
}

pub enum ItemBody {
    Short(Expr),
    Long(Statements)
}

impl fmt::Display for ItemBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Short(expr) => write!(f, "=> {expr};"),
            Self::Long(statements) => write!(f, "{statements}"),

        }
    }
}

pub struct Expr;

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<TODO EXPR>")
    }
}

pub struct Statements;

impl fmt::Display for Statements {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<TODO STMTS>")
    }
}

pub struct FnArgs {
    pub positional: Vec<FnArg>,
    pub named: Vec<NamedFnArg>,
}

impl FnArgs {
    pub fn new() -> Self {
        Self {
            positional: Vec::new(),
            named: Vec::new(),
        }
    }
}

impl fmt::Display for FnArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;

        for arg in &self.positional {
            write!(f, "{arg}, ")?;
        }

        if !self.named.is_empty() {
            write!(f, "{{")?;

            for named_arg in &self.named {
                write!(f, "{named_arg}, ")?;
            }

            write!(f, "}}")?;
        }

        write!(f, ")")
    }
}

pub struct FnArg {
    pub name: Ident,
    pub ty: Type,
    pub initializing: bool,
}

impl fmt::Display for FnArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { name, ty, initializing } = self;

        write!(f, "{ty} ")?;

        if *initializing {
            write!(f, "this.")?;
        }

        write!(f, "{name}")
    }
}

pub struct NamedFnArg {
    pub name: Ident,
    pub ty: Type,
    pub required: bool,
    pub initializing: bool,
}

impl fmt::Display for NamedFnArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { name, ty, required, initializing } = self;

        if *required {
            write!(f, "required ")?;
        };

        write!(f, "{ty} ")?;

        if *initializing {
            write!(f, "this.")?;
        }

        write!(f, "{name}")
    }
}
