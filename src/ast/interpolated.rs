use derive_more::{Display, From};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display, From)]
#[display("${{{}}}", "")] // TODO _0
pub struct Interpolated(pub ()); // TODO Expression

impl Interpolated {
    pub fn expression(&self) -> &() {
        // TODO Expression
        &self.0
    }
}
