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

#[cfg(test)]
mod tests {
    use super::*;

    mod admin {
        use super::*;

        fn operation() -> Operation {
            Operation::Administration
        }

        #[test]
        fn should_not_be_view() {
            let result = operation().is_view();
            assert_eq!(result, false);
        }

        #[test]
        fn should_have_no_sponsor() {
            let result = operation().sponsor();
            assert_eq!(result, None);
        }

        #[test]
        fn should_have_no_session() {
            let result = operation().session();
            assert_eq!(result, None);
        }
    }

    mod view_site {
        use super::*;

        fn operation() -> Operation {
            Operation::ViewSite
        }

        #[test]
        fn should_be_view() {
            let result = operation().is_view();
            assert_eq!(result, true);
        }

        #[test]
        fn should_have_no_sponsor() {
            let result = operation().sponsor();
            assert_eq!(result, None);
        }

        #[test]
        fn should_have_no_session() {
            let result = operation().session();
            assert_eq!(result, None);
        }
    }

    mod view_session {
        use super::*;

        fn key() -> SessionKey {
            SessionKey::new("plop")
        }

        fn operation() -> Operation {
            Operation::ViewSession(key())
        }

        #[test]
        fn should_be_view() {
            let result = operation().is_view();
            assert_eq!(result, true);
        }

        #[test]
        fn should_have_no_sponsor() {
            let result = operation().sponsor();
            assert_eq!(result, None);
        }

        #[test]
        fn should_have_session() {
            let result = operation().session();
            assert_eq!(result, Some(key()));
        }
    }

    mod view_speaker {
        use super::*;

        fn key() -> SpeakerKey {
            SpeakerKey::new("plop")
        }

        fn operation() -> Operation {
            Operation::ViewSpeaker(key())
        }

        #[test]
        fn should_be_view() {
            let result = operation().is_view();
            assert_eq!(result, true);
        }

        #[test]
        fn should_have_no_sponsor() {
            let result = operation().sponsor();
            assert_eq!(result, None);
        }

        #[test]
        fn should_have_no_session() {
            let result = operation().session();
            assert_eq!(result, None);
        }
    }

    mod view_sponsor {
        use super::*;

        fn key() -> SponsorKey {
            SponsorKey::new("plop")
        }

        fn operation() -> Operation {
            Operation::ViewSponsor(key())
        }

        #[test]
        fn should_be_view() {
            let result = operation().is_view();
            assert_eq!(result, true);
        }

        #[test]
        fn should_have_no_sponsor() {
            let result = operation().sponsor();
            assert_eq!(result, Some(key()));
        }

        #[test]
        fn should_have_no_session() {
            let result = operation().session();
            assert_eq!(result, None);
        }
    }

    mod edit_site {
        use super::*;

        fn operation() -> Operation {
            Operation::EditSite
        }

        #[test]
        fn should_not_be_view() {
            let result = operation().is_view();
            assert_eq!(result, false);
        }

        #[test]
        fn should_have_no_sponsor() {
            let result = operation().sponsor();
            assert_eq!(result, None);
        }

        #[test]
        fn should_have_no_session() {
            let result = operation().session();
            assert_eq!(result, None);
        }
    }

    mod edit_session {
        use super::*;

        fn key() -> SessionKey {
            SessionKey::new("plop")
        }

        fn operation() -> Operation {
            Operation::EditSession(key())
        }

        #[test]
        fn should_not_be_view() {
            let result = operation().is_view();
            assert_eq!(result, false);
        }

        #[test]
        fn should_have_no_sponsor() {
            let result = operation().sponsor();
            assert_eq!(result, None);
        }

        #[test]
        fn should_have_session() {
            let result = operation().session();
            assert_eq!(result, Some(key()));
        }
    }

    mod edit_speaker {
        use super::*;

        fn key() -> SpeakerKey {
            SpeakerKey::new("plop")
        }

        fn operation() -> Operation {
            Operation::EditSpeaker(key())
        }

        #[test]
        fn should_not_be_view() {
            let result = operation().is_view();
            assert_eq!(result, false);
        }

        #[test]
        fn should_have_no_sponsor() {
            let result = operation().sponsor();
            assert_eq!(result, None);
        }

        #[test]
        fn should_have_no_session() {
            let result = operation().session();
            assert_eq!(result, None);
        }
    }

    mod edit_sponsor {
        use super::*;

        fn key() -> SponsorKey {
            SponsorKey::new("plop")
        }

        fn operation() -> Operation {
            Operation::EditSponsor(key())
        }

        #[test]
        fn should_not_be_view() {
            let result = operation().is_view();
            assert_eq!(result, false);
        }

        #[test]
        fn should_have_no_sponsor() {
            let result = operation().sponsor();
            assert_eq!(result, Some(key()));
        }

        #[test]
        fn should_have_no_session() {
            let result = operation().session();
            assert_eq!(result, None);
        }
    }
}
