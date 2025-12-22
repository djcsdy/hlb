use crate::ast::Interpolated;
use derive_more::{Display, From, IsVariant, TryUnwrap, Unwrap};
use regex::Regex;
use std::borrow::Cow;
use std::sync::OnceLock;

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display, From, IsVariant, Unwrap, TryUnwrap,
)]
pub enum StringFragment {
    Interpolated(Interpolated),
    #[display("{}", Self::escape(_0))]
    Text(String),
}

impl StringFragment {
    fn escape_regex() -> &'static Regex {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        REGEX.get_or_init(|| Regex::new(r#"[\\"]"#).unwrap())
    }

    fn escape(text: &'_ str) -> Cow<'_, str> {
        StringFragment::escape_regex().replace_all(text, "\\$0")
    }
}

impl From<&str> for StringFragment {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}
