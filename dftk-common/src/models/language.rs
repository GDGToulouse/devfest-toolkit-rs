use core::fmt;
use std::str::FromStr;

use anyhow::Result;
use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use unic_langid::{langid, LanguageIdentifier};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Lang(LanguageIdentifier);

impl Default for Lang {
    fn default() -> Self {
        Lang(langid!("en-US"))
    }
}

impl FromStr for Lang {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
        Lang::from_str(value).map_err(|err| E::custom(format!("Error in deserializer {:?}", err)))
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

impl Lang {
    pub fn from_user_field(s: &str) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    mod lang {
        use super::*;

        #[test]
        fn should_be_parsed_from_str() {
            assert_eq!(Lang(langid!("fr-FR")), Lang::from_user_field(" francais"));
            assert_eq!(Lang(langid!("fr-FR")), Lang::from_user_field("franCais"));
            assert_eq!(Lang(langid!("fr-FR")), Lang::from_user_field("français"));
            assert_eq!(Lang(langid!("fr-FR")), Lang::from_user_field("franÇais"));
            assert_eq!(Lang(langid!("fr-FR")), Lang::from_user_field(" fr "));
            assert_eq!(Lang(langid!("fr-FR")), Lang::from_user_field("FRench"));

            assert_eq!(Lang(langid!("en-US")), Lang::from_user_field("Anglais"));
            assert_eq!(Lang(langid!("en-US")), Lang::from_user_field("English"));

            assert_eq!(Lang::default(), Lang::from_user_field("Plop"));
        }

        #[test]
        fn could_be_transform_to_string() {
            let s: String = Lang(langid!("fr-FR")).into();
            assert_eq!(s, "fr-FR");
        }

        #[test]
        fn should_be_serializable() {
            let lang = Lang::default();
            let result = serde_json::to_string(&lang);
            assert!(result.is_ok());
        }

        #[test]
        fn should_be_deserializable() {
            let json = r#""fr-FR""#;
            let result = serde_json::from_str::<Lang>(json);
            assert!(result.is_ok());
        }
    }

    mod languages {
        use super::*;

        #[test]
        fn should_be_serializable() {
            let languages = Languages::new(
                Lang::default(),
                vec![Lang(langid!("fr-FR")), Lang(langid!("de-DE"))],
            );
            let result = serde_json::to_string(&languages);
            assert!(result.is_ok());
        }

        #[test]
        fn should_be_deserializable() {
            let json = r#"{
  "main": "en",
  "others": [ "fr", "de" ]
}"#;
            let result = serde_json::from_str::<Languages>(json);
            assert!(result.is_ok());
        }

        #[test]
        fn should_get_main() {
            let languages = Languages::new(
                Lang::default(),
                vec![Lang(langid!("fr-FR")), Lang(langid!("de-DE"))],
            );
            let result = languages.main();
            assert_eq!(result, Lang::default());
        }

        #[test]
        fn should_get_other() {
            let languages = Languages::new(
                Lang::default(),
                vec![Lang(langid!("fr-FR")), Lang(langid!("de-DE"))],
            );
            let result = languages.others().to_vec();
            assert!(result.contains(&Lang(langid!("fr-FR"))));
            assert!(result.contains(&Lang(langid!("de-DE"))));
            assert_eq!(result.contains(&Lang(langid!("en-US"))), false);
        }
    }
}
