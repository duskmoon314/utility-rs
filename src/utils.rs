use std::ops::Deref;

use darling::FromMeta;
use syn::{Ident, Meta};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct IdentList(Vec<Ident>);

impl IdentList {
    pub fn new<T: Into<Ident>>(values: Vec<T>) -> Self {
        Self(values.into_iter().map(T::into).collect())
    }
}

impl Deref for IdentList {
    type Target = Vec<Ident>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<Ident>> for IdentList {
    fn from(values: Vec<Ident>) -> Self {
        Self(values)
    }
}

impl FromMeta for IdentList {
    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        let values = items
            .iter()
            .map(|item| match item {
                darling::ast::NestedMeta::Meta(Meta::Path(path)) => match path.get_ident() {
                    Some(ident) => Ok(ident.clone()),
                    None => Err(darling::Error::unexpected_type("non ident").with_span(item)),
                },
                _ => {
                    Err(darling::Error::unexpected_type("non path, expected ident").with_span(item))
                }
            })
            .collect::<darling::Result<Vec<Ident>>>()?;

        if values.is_empty() {
            return Err(darling::Error::too_few_items(1));
        }

        Ok(Self::new(values))
    }
}
