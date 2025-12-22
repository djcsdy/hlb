use crate::ast::field_list::FieldList;
use crate::ast::{EffectsClause, Identifier, Kind};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct FunctionSignature {
    pub return_type: Kind,
    pub name: Identifier,
    pub parameters: FieldList,
    pub effects: Option<EffectsClause>,
}

impl Display for FunctionSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}{}", self.return_type, self.name, self.parameters)?;
        if let Some(effects) = &self.effects {
            write!(f, " {}", effects)?;
        }
        Ok(())
    }
}
