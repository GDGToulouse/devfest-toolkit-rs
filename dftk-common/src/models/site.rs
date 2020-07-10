use core::fmt;
use std::fmt::Display;

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::export::Formatter;
use serde::{Deserialize, Serialize};

use crate::models::language::Languages;
use crate::models::schedule::{Room, ScheduleDay, Slot};
use crate::models::session::category::SessionCategory;
use crate::models::session::format::SessionFormat;
use crate::models::session::Session;
use crate::models::speaker::Speaker;
use crate::models::sponsor::category::SponsorCategory;
use crate::models::sponsor::Sponsor;
use crate::models::team::member_type::MemberType;
use crate::models::team::TeamMember;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct EventId(pub(crate) String);

impl EventId {
    pub fn new(s: String) -> Self {
        Self(s)
    }
}

impl Display for EventId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Into<String> for EventId {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SiteInfo {
    _id: EventId,
    name: String,
    address: Address,
    languages: Languages,
    dates: DateRange,
}

impl SiteInfo {
    pub fn new(
        id: EventId,
        name: String,
        address: Address,
        languages: Languages,
        dates: DateRange,
    ) -> Self {
        SiteInfo {
            _id: id,
            name,
            address,
            languages,
            dates,
        }
    }

    pub fn id(&self) -> EventId {
        self._id.clone()
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn address(&self) -> Address {
        self.address.clone()
    }
    pub fn languages(&self) -> Languages {
        self.languages.clone()
    }
    pub fn dates(&self) -> DateRange {
        self.dates.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DateRange {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl DateRange {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        if start > end {
            panic!(
                "Invalid date range, expected start <= end, got start = {} and end = {}",
                start, end
            );
        }

        Self { start, end }
    }

    pub fn form_string(start: &str, end: &str) -> Result<Self> {
        let start = start.parse()?;
        let end = end.parse()?;

        Ok(DateRange::new(start, end))
    }

    pub fn start(&self) -> DateTime<Utc> {
        self.start
    }
    pub fn end(&self) -> DateTime<Utc> {
        self.end
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Name {
    long_name: String,
    short_name: String,
}

impl From<String> for Name {
    fn from(s: String) -> Self {
        let long_name = s.clone();
        let short_name = s;

        Self {
            long_name,
            short_name,
        }
    }
}

impl Name {
    pub fn new(long_name: String, short_name: String) -> Self {
        Self {
            long_name,
            short_name,
        }
    }

    pub fn long_name(&self) -> String {
        self.long_name.clone()
    }
    pub fn short_name(&self) -> String {
        self.short_name.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Geolocation {
    lat: f64,
    lng: f64,
}

impl Geolocation {
    pub fn new(lat: f64, lng: f64) -> Self {
        Geolocation { lat, lng }
    }

    pub fn lat(&self) -> f64 {
        self.lat
    }
    pub fn lng(&self) -> f64 {
        self.lng
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    locality: Name,
    country: Name,
    lat_lng: Geolocation,
}

impl Address {
    pub fn new(locality: Name, country: Name, lat_lng: Geolocation) -> Self {
        Self {
            locality,
            country,
            lat_lng,
        }
    }

    pub fn locality(&self) -> Name {
        self.locality.clone()
    }
    pub fn country(&self) -> Name {
        self.country.clone()
    }
    pub fn lat_lng(&self) -> Geolocation {
        self.lat_lng.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Site {
    info: SiteInfo,
    sessions: Vec<Session>,
    speakers: Vec<Speaker>,
    categories: Vec<SessionCategory>,
    formats: Vec<SessionFormat>,
    rooms: Vec<Room>,
    slots: Vec<Slot>,
    schedule: Vec<ScheduleDay>,
    team: Vec<TeamMember>,
    member_types: Vec<MemberType>,
    sponsors: Vec<Sponsor>,
    sponsor_categories: Vec<SponsorCategory>,
}

impl Site {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        info: SiteInfo,
        sessions: Vec<Session>,
        speakers: Vec<Speaker>,
        categories: Vec<SessionCategory>,
        formats: Vec<SessionFormat>,
        rooms: Vec<Room>,
        slots: Vec<Slot>,
        schedule: Vec<ScheduleDay>,
        team: Vec<TeamMember>,
        member_types: Vec<MemberType>,
        sponsors: Vec<Sponsor>,
        sponsor_categories: Vec<SponsorCategory>,
    ) -> Self {
        Site {
            info,
            sessions,
            speakers,
            categories,
            formats,
            rooms,
            slots,
            schedule,
            team,
            member_types,
            sponsors,
            sponsor_categories,
        }
    }

    pub fn info(&self) -> SiteInfo {
        self.info.clone()
    }
    pub fn sessions(&self) -> &[Session] {
        self.sessions.as_slice()
    }
    pub fn speakers(&self) -> &[Speaker] {
        self.speakers.as_slice()
    }
    pub fn categories(&self) -> &[SessionCategory] {
        self.categories.as_slice()
    }
    pub fn formats(&self) -> &[SessionFormat] {
        self.formats.as_slice()
    }
    pub fn rooms(&self) -> &[Room] {
        self.rooms.as_slice()
    }
    pub fn slots(&self) -> &[Slot] {
        self.slots.as_slice()
    }
    pub fn schedule(&self) -> &[ScheduleDay] {
        self.schedule.as_slice()
    }
    pub fn team(&self) -> &[TeamMember] {
        self.team.as_slice()
    }
    pub fn member_types(&self) -> &[MemberType] {
        self.member_types.as_slice()
    }
    pub fn sponsors(&self) -> &[Sponsor] {
        self.sponsors.as_slice()
    }
    pub fn sponsor_categories(&self) -> &[SponsorCategory] {
        self.sponsor_categories.as_slice()
    }
}
