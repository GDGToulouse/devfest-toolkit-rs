use serde::{Deserialize, Serialize};

pub mod language;
pub mod schedule;
pub mod session;
pub mod site;
pub mod socials;
pub mod speaker;
pub mod sponsor;
pub mod team;

pub type Duration = u8;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Markdown(String);

impl From<String> for Markdown {
    fn from(s: String) -> Self {
        Markdown(s)
    }
}

impl Into<String> for Markdown {
    fn into(self) -> String {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod markdown {
        use super::*;

        #[test]
        fn should_create_from_string() {
            let given = String::from("plop");
            let result: Markdown = given.clone().into();
            assert_eq!(result.0, given);
        }

        #[test]
        fn should_transform_to_string() {
            let md = Markdown(String::from("plop"));
            let result: String = md.clone().into();
            assert_eq!(result, md.0);
        }

        #[test]
        fn should_be_serializable() {
            let md = Markdown("Plop".into());
            let result = serde_json::to_string(&md);
            assert!(result.is_ok());
        }

        #[test]
        fn should_be_deserializable() {
            let json = r#""Plop""#;
            let result = serde_json::from_str::<Markdown>(json);
            assert!(result.is_ok());
        }
    }
}
