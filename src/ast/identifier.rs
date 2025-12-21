use derive_more::AsRef;
use derive_more::with_trait::Display;
use regex::Regex;
use std::str::FromStr;
use std::sync::OnceLock;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display, AsRef)]
pub struct Identifier(String);

impl Identifier {
    fn regex() -> &'static Regex {
        static IDENTIFIER: OnceLock<Regex> = OnceLock::new();
        IDENTIFIER.get_or_init(|| Regex::new(r"^\w+$").unwrap())
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}

impl FromStr for Identifier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Identifier::regex()
            .is_match(s)
            .then(|| Self(s.to_string()))
            .ok_or(())
    }
}
