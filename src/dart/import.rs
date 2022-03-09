use super::Ident;

pub struct Import {
    path: String,
    selector: Selector,
}

pub enum Selector {
    All,
    Show(Vec<Ident>),
    Hide(Vec<Ident>),
}
