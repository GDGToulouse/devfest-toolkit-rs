#[cfg(feature = "graphql")]
use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};

use crate::models::language::Lang;
use crate::models::schedule::{RoomKey, SlotKey};
use crate::models::session::category::CategoryKey;
use crate::models::session::format::FormatKey;
use crate::models::session::{SessionId, SessionKey};
use crate::models::site::EventId;
use crate::models::speaker::{SpeakerId, SpeakerKey};
use crate::models::sponsor::category::SponsorCategoryKey;
use crate::models::sponsor::SponsorKey;
use crate::models::team::member_type::MemberTypeKey;
use crate::models::team::TeamMemberKey;
use std::str::FromStr;

#[Scalar]
impl ScalarType for CategoryKey {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            Ok(CategoryKey::new(value.as_str()))
        } else {
            Err(InputValueError::ExpectedType(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.clone().into())
    }
}

#[Scalar]
impl ScalarType for FormatKey {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            Ok(FormatKey::new(value.as_str()))
        } else {
            Err(InputValueError::ExpectedType(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.clone().into())
    }
}

#[Scalar]
impl ScalarType for EventId {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            Ok(EventId(value))
        } else {
            Err(InputValueError::ExpectedType(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}

#[Scalar]
impl ScalarType for Lang {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            let lang = Lang::from_str(value.as_str())?;
            Ok(lang)
        } else {
            Err(InputValueError::ExpectedType(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.clone().into())
    }
}

#[Scalar]
impl ScalarType for SlotKey {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            Ok(SlotKey::new(value.as_str()))
        } else {
            Err(InputValueError::ExpectedType(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.clone().into())
    }
}

#[Scalar]
impl ScalarType for RoomKey {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            Ok(RoomKey::new(value.as_str()))
        } else {
            Err(InputValueError::ExpectedType(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.clone().into())
    }
}

#[Scalar]
impl ScalarType for SessionKey {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            Ok(SessionKey::new(value.as_str()))
        } else {
            Err(InputValueError::ExpectedType(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.clone().into())
    }
}

#[Scalar]
impl ScalarType for SessionId {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            Ok(SessionId::new(value))
        } else {
            Err(InputValueError::ExpectedType(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.clone().into())
    }
}

#[Scalar]
impl ScalarType for SpeakerId {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            Ok(SpeakerId::new(value))
        } else {
            Err(InputValueError::ExpectedType(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.clone().into())
    }
}

#[Scalar]
impl ScalarType for SpeakerKey {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            Ok(SpeakerKey::new(value.as_str()))
        } else {
            Err(InputValueError::ExpectedType(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.clone().into())
    }
}

#[Scalar]
impl ScalarType for SponsorKey {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            Ok(SponsorKey::new(value.as_str()))
        } else {
            Err(InputValueError::ExpectedType(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.clone().into())
    }
}

#[Scalar]
impl ScalarType for SponsorCategoryKey {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            Ok(SponsorCategoryKey::new(value.as_str()))
        } else {
            Err(InputValueError::ExpectedType(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.clone().into())
    }
}

#[Scalar]
impl ScalarType for TeamMemberKey {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            Ok(TeamMemberKey::new(value.as_str()))
        } else {
            Err(InputValueError::ExpectedType(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.clone().into())
    }
}

#[Scalar]
impl ScalarType for MemberTypeKey {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            Ok(MemberTypeKey::new(value.as_str()))
        } else {
            Err(InputValueError::ExpectedType(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.clone().into())
    }
}
