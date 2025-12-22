use crate::ast::Field;
use crate::ast::field_list::FieldList;
use derive_more::{Display, From};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display, From)]
#[display("binds {}", _0)]
pub struct EffectsClause(pub FieldList);

impl EffectsClause {
    pub fn fields(&self) -> &FieldList {
        &self.0
    }
}

impl From<Vec<Field>> for EffectsClause {
    fn from(fields: Vec<Field>) -> Self {
        Self(FieldList::from(fields))
    }
}
