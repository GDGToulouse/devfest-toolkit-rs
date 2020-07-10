use serde::{Deserialize, Serialize};

use dftk_common::models::language::Lang;
use dftk_common::models::session::category::CategoryKey;
use dftk_common::models::session::format::FormatKey;
use dftk_common::models::session::{Session, SessionId, SessionKey, SessionLevel};
use dftk_common::models::speaker::SpeakerKey;
use dftk_common::models::Markdown;

use crate::markdown_writer::FrontMatterMarkdown;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionFrontMatter {
    id: SessionId,
    key: SessionKey,
    title: String,
    level: Option<SessionLevel>,
    format: FormatKey,
    speakers: Vec<SpeakerKey>,
    tags: Vec<CategoryKey>,
    language: Lang,
    video_id: Option<String>,
    presentation: Option<String>,
    draft: Option<bool>,
    office_hours: Option<Vec<SessionKey>>,
}

impl FrontMatterMarkdown<SessionFrontMatter> for Session {
    fn unique_key(&self) -> String {
        self.key().into()
    }

    fn front_matter(&self) -> SessionFrontMatter {
        SessionFrontMatter {
            id: self.id(),
            key: self.key(),
            title: self.title(),
            level: self.level(),
            format: self.format(),
            speakers: self.speakers(),
            tags: vec![self.category()],
            language: self.language(),
            video_id: self.video_id(),
            presentation: self.presentation(),
            draft: self.draft(),
            office_hours: self.office_hours(),
        }
    }

    fn content(&self) -> Markdown {
        self.description()
    }
}
