use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Social {
    Facebook(String),
    Twitter(String),
    LinkedIn(String),
    WebSite(String),
    GitHub(String),
    GitLab(String),
}
