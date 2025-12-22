use crate::ast::{BindClause, Expression, Identifier, WithClause};
use itertools::Itertools;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct CallStatement {
    pub name: Identifier,
    pub arguments: Vec<Expression>,
    pub with_clause: Option<WithClause>,
    pub bind_clause: Option<BindClause>,
}

impl Display for CallStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if !self.arguments.is_empty() {
            write!(f, " {}", self.arguments.iter().format(" "))?;
        }
        if let Some(with_clause) = &self.with_clause {
            write!(f, " {}", with_clause)?;
        }
        if let Some(bind_clause) = &self.bind_clause {
            write!(f, " {}", bind_clause)?;
        }
        write!(f, ";")
    }
}
