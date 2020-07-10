use crate::models::session::SessionKey;
use crate::models::speaker::SpeakerKey;
use crate::models::sponsor::SponsorKey;

pub enum Operation {
    Administration,
    ViewSite,
    ViewSession(SessionKey),
    ViewSpeaker(SpeakerKey),
    ViewSponsor(SponsorKey),
    EditSite,
    EditSession(SessionKey),
    EditSpeaker(SpeakerKey),
    EditSponsor(SponsorKey),
}

impl Operation {
    pub fn is_view(&self) -> bool {
        match self {
            Operation::ViewSite => true,
            Operation::ViewSession(_) => true,
            Operation::ViewSpeaker(_) => true,
            Operation::ViewSponsor(_) => true,
            _ => false,
        }
    }

    pub fn session(&self) -> Option<SessionKey> {
        match self {
            Operation::ViewSession(k) => Some(k.clone()),
            Operation::EditSession(k) => Some(k.clone()),
            _ => None,
        }
    }

    pub fn sponsor(&self) -> Option<SponsorKey> {
        match self {
            Operation::ViewSponsor(k) => Some(k.clone()),
            Operation::EditSponsor(k) => Some(k.clone()),
            _ => None,
        }
    }
}
