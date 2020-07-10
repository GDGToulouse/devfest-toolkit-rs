use async_graphql::{InputObject, SimpleObject};

use dftk_common::models::socials::Social;

#[SimpleObject]
#[derive(Debug, Clone, Default)]
pub struct SocialOutputType {
    facebook: Option<String>,
    twitter: Option<String>,
    linkedin: Option<String>,
    website: Option<String>,
    github: Option<String>,
    gitlab: Option<String>,
}

impl SocialOutputType {
    pub fn new(socials: &[Social]) -> Self {
        let mut result = SocialOutputType::default();
        for social in socials {
            match social {
                Social::Facebook(s) => result.facebook = Some(s.clone()),
                Social::Twitter(s) => result.twitter = Some(s.clone()),
                Social::LinkedIn(s) => result.linkedin = Some(s.clone()),
                Social::WebSite(s) => result.website = Some(s.clone()),
                Social::GitHub(s) => result.github = Some(s.clone()),
                Social::GitLab(s) => result.gitlab = Some(s.clone()),
            }
        }

        result
    }
}

#[InputObject]
#[derive(Debug, Clone)]
pub struct SocialInputType {
    facebook: Option<String>,
    twitter: Option<String>,
    linkedin: Option<String>,
    website: Option<String>,
    github: Option<String>,
    gitlab: Option<String>,
}

impl Into<Vec<Social>> for SocialInputType {
    fn into(self) -> Vec<Social> {
        let mut result = vec![];

        if let Some(s) = self.facebook {
            result.push(Social::Facebook(s))
        }
        if let Some(s) = self.twitter {
            result.push(Social::Twitter(s))
        }
        if let Some(s) = self.linkedin {
            result.push(Social::LinkedIn(s))
        }
        if let Some(s) = self.website {
            result.push(Social::WebSite(s))
        }
        if let Some(s) = self.github {
            result.push(Social::GitHub(s))
        }
        if let Some(s) = self.gitlab {
            result.push(Social::GitLab(s))
        }

        result
    }
}
