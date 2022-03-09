use std::fmt;

use crate::dart::Ident;

use super::{FnArgs, ItemBody};

pub struct Method {
    pub class_name: Option<Ident>,
    pub name: Ident,
    pub is_factory: bool,
    pub fn_args: FnArgs,
    pub body: Option<ItemBody>,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { class_name, name, fn_args, body, .. } = self;

        if self.is_factory {
            write!(f, "factory ")?;
        }

        match class_name {
            Some(class_name) => write!(f, "{class_name}.{name}")?,
            None => write!(f, "{name}")?,
        };


        write!(f, "{fn_args}")?;

        match body {
            Some(body) => write!(f, "{body}"),
            None => write!(f, ";"),
        }
    }
}
