use crate::ast::field::Field;
use derive_more::{AsRef, From};
use itertools::Itertools;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, From, AsRef)]
#[as_ref(forward)]
pub struct FieldList(pub Vec<Field>);

impl FieldList {
    pub fn fields(&self) -> &Vec<Field> {
        &self.0
    }
}

impl Display for FieldList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.0.iter().format(" "))
    }
}
