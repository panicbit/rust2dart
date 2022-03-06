use std::collections::HashMap;

use super::Ident;

pub struct Class {
    name: Ident,
    fields: HashMap<Ident, Field>,
}

pub enum Field {
    Variable(),
    Getter(),
    Setter(),
    GetterSetter(),
}

pub struct Variable {
    name: Ident,
    ty: Type,
}

pub struct Getter {
    name: Ident,
    ty: Type,
    body: Body,
}

pub struct Setter {
    name: Ident,
    ty: Type,
    body: Body,
}

pub struct SetterGetter {
    name: Ident,
    ty: Type,
    getter_body: Body,
    setter_body: Body,
}

pub struct Type;

pub struct Body;
