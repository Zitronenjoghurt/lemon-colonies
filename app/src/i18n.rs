use crate::ui::windows::WindowId;
use heck::ToSnakeCase;
use lemon_colonies_core::game::object::data::bush::BushKind;
use lemon_colonies_core::game::object::kind::ObjectKind;
use lemon_colonies_core::lingo::Lingo;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Display;
use strum_macros::EnumIter;

#[derive(Debug, Default, Clone, Copy, PartialEq, EnumIter, Serialize, Deserialize)]
pub enum Locale {
    #[default]
    English = 0,
    German = 1,
}

impl Locale {
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "en" => Some(Self::English),
            "de" => Some(Self::German),
            _ => None,
        }
    }

    pub fn id(&self) -> &'static str {
        match self {
            Self::English => "en",
            Self::German => "de",
        }
    }

    pub fn get() -> Self {
        Self::from_id(rust_i18n::locale().as_ref()).unwrap_or_default()
    }

    pub fn apply(&self) {
        rust_i18n::set_locale(self.id());
    }
}

impl Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::English => write!(f, "English"),
            Self::German => write!(f, "Deutsch"),
        }
    }
}

#[macro_export]
macro_rules! tl {
    ($translatable:expr) => {{
        use $crate::i18n::Translatable;
        rust_i18n::t!($translatable.key())
    }};
    ($translatable:expr, $($key:ident = $value:expr),* $(,)?) => {{
        use $crate::i18n::Translatable;
        rust_i18n::t!($translatable.key(), $($key = $value),*)
    }};
}

pub trait Translatable {
    fn key(&self) -> String;

    fn t(&'_ self) -> Cow<'_, str> {
        tl!(self)
    }

    fn tl(&self) -> String {
        self.t().to_lowercase()
    }

    fn tu(&self) -> String {
        self.t().to_uppercase()
    }
}

impl Translatable for ObjectKind {
    fn key(&self) -> String {
        format!("object.{}", format!("{:?}", self).to_snake_case())
    }
}

impl Translatable for BushKind {
    fn key(&self) -> String {
        format!("object.bush.{}", format!("{:?}", self).to_snake_case())
    }
}

impl Translatable for Lingo {
    fn key(&self) -> String {
        format!("common.{}", format!("{:?}", self).to_snake_case())
    }
}

impl Translatable for WindowId {
    fn key(&self) -> String {
        format!("window.{}", format!("{:?}", self).to_snake_case())
    }
}

#[cfg(test)]
mod tests {
    use crate::i18n::*;
    use lemon_colonies_core::game::object::data::bush::BushKind;
    use lemon_colonies_core::game::object::kind::ObjectKind;
    use strum::IntoEnumIterator;

    fn assert_all_translated<T: Translatable + IntoEnumIterator + std::fmt::Debug>() {
        for locale in Locale::iter() {
            for item in T::iter() {
                let key = item.key();
                let result = rust_i18n::t!(key, locale = locale.id());
                assert_ne!(
                    result, key,
                    "{locale:?} is missing translation for {item:?} (key: {key})"
                );
            }
        }
    }

    #[test]
    fn test_translations() {
        assert_all_translated::<BushKind>();
        assert_all_translated::<Lingo>();
        assert_all_translated::<ObjectKind>();
        assert_all_translated::<WindowId>();
    }
}
