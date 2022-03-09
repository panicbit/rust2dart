use super::Ident;
pub use super::import::Selector;

pub struct Export {
    path: String,
    selector: Selector,
}
