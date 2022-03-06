use anyhow::{Error, Result, bail};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Ident(String);

impl TryFrom<syn::Ident> for Ident {
    type Error = Error;

    fn try_from(ident: syn::Ident) -> Result<Self> {
        let ident = ident.to_string();

        if ident.starts_with("r#") {
            bail!("Raw idents are not supported yet: {}", ident);
        }

        Ok(Self(ident))
    }
}