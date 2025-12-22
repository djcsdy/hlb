use crate::ast::Bind;
use derive_more::From;
use derive_more::with_trait::AsRef;
use itertools::Itertools;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, From, AsRef)]
#[as_ref(forward)]
pub struct BindList(pub Vec<Bind>);

impl BindList {
    pub fn binds(&self) -> &Vec<Bind> {
        &self.0
    }
}

impl Display for BindList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.0.iter().format(", "))
    }
}
