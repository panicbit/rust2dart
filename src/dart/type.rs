use std::fmt;

use super::Ident;

#[derive(Debug, Clone)]
pub struct Type {
    pub name: Ident,
    pub arguments: Arguments,
    pub import: Option<Ident>,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(import) = &self.import {
            write!(f, "{}.", import)?;
        }

        write!(f, "{}", self.name)?;

        match &self.arguments {
            Arguments::None => {},
            Arguments::Generic(generic_arguments) => {
                write!(f, "<")?;

                for argument in generic_arguments {
                    write!(f, "{argument}")?;
                }

                write!(f, ">")?;
            },
            Arguments::Fn(_) => todo!(),
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Arguments {
    None,
    Generic(Vec<Type>),
    Fn(FnTypeArguments)
}

#[derive(Debug, Clone)]
pub struct FnTypeArguments {
    return_type: Box<Type>,
    args: Vec<Type>,
}
