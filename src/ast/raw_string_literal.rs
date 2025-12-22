use derive_more::{AsRef, Display};
use std::str::FromStr;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display, AsRef)]
#[display("`{}`", _0)]
pub struct RawStringLiteral(String);

impl RawStringLiteral {
    pub fn text(&self) -> &str {
        &self.0
    }
}

impl FromStr for RawStringLiteral {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("`") {
            Err(())
        } else {
            Ok(Self(s.to_string()))
        }
    }
}
