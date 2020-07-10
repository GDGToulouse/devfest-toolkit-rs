use core::fmt;

use anyhow::Result;
use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use unic_langid::{langid, LanguageIdentifier};

#[derive(Debug, Clone)]
pub struct Lang(LanguageIdentifier);

impl Default for Lang {
    fn default() -> Self {
        Lang(langid!("en-US"))
    }
}

impl Lang {
    pub fn build(s: String) -> Result<Lang> {
        let langid = s.parse::<LanguageIdentifier>()?;
        let result = Self(langid);

        Ok(result)
    }
}

impl Into<String> for Lang {
    fn into(self) -> String {
        self.0.to_string()
    }
}

impl Serialize for Lang {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.language.as_str())
    }
}

struct LangVisitor;

impl<'de> Visitor<'de> for LangVisitor {
    type Value = Lang;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "a valid language identifier string (see https://unicode.org/reports/tr35/#Unicode_language_identifier)")
    }
    fn visit_str<E: de::Error>(self, value: &str) -> Result<Lang, E> {
        Lang::build(value.into())
            .map_err(|err| E::custom(format!("Error in deserializer {:?}", err)))
    }
}

impl<'de> Deserialize<'de> for Lang {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(LangVisitor)
    }
}

impl From<Option<String>> for Lang {
    fn from(option: Option<String>) -> Self {
        match option {
            Some(s) => {
                let low = s.to_lowercase();
                if low.contains("francais")
                    || low.contains("français")
                    || low.contains("french")
                    || low.contains("fr")
                {
                    Lang(langid!("fr-FR"))
                } else if low.contains("english") || low.contains("anglais") || low.contains("en") {
                    Lang(langid!("en-US"))
                } else {
                    Lang::default()
                }
            }
            None => Lang::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Languages {
    main: Lang,
    others: Vec<Lang>,
}

impl Languages {
    pub fn new(main: Lang, others: Vec<Lang>) -> Self {
        Self { main, others }
    }

    pub fn main(&self) -> Lang {
        self.main.clone()
    }
    pub fn others(&self) -> &[Lang] {
        self.others.as_slice()
    }
}
