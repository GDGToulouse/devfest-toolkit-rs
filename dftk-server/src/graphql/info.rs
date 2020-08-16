use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};

use dftk_common::models::language::{Lang, Languages};
use dftk_common::models::site::{Address, DateRange, EventId, Geolocation, Name, SiteInfo};

use crate::graphql::languages::LanguagesOutputType;

#[InputObject]
pub struct SiteInfoInputType {
    name: String,
    address: AddressInputType,
    languages: LanguagesInputType,
    dates: DateRangeInputType,
}

impl SiteInfoInputType {
    pub fn to_site_info(&self, event_id: &EventId) -> SiteInfo {
        let name = self.name.clone();
        let address = (&self.address).into();
        let languages = (&self.languages).into();
        let dates = (&self.dates).into();

        SiteInfo::new(event_id.clone(), name, address, languages, dates)
    }
}

#[InputObject]
struct AddressInputType {
    locality: NameInputType,
    country: NameInputType,
    lat_lng: GeolocationInputType,
}

impl Into<Address> for &AddressInputType {
    fn into(self) -> Address {
        Address::new(
            (&self.locality).into(),
            (&self.country).into(),
            (&self.lat_lng).into(),
        )
    }
}

#[InputObject]
struct NameInputType {
    long_name: String,
    short_name: String,
}

impl Into<Name> for &NameInputType {
    fn into(self) -> Name {
        Name::new(self.long_name.clone(), self.short_name.clone())
    }
}

#[InputObject]
struct GeolocationInputType {
    lat: f64,
    lng: f64,
}
impl Into<Geolocation> for &GeolocationInputType {
    fn into(self) -> Geolocation {
        Geolocation::new(self.lat, self.lng)
    }
}

#[InputObject]
pub struct LanguagesInputType {
    main: Lang,
    others: Vec<Lang>,
}

impl Into<Languages> for &LanguagesInputType {
    fn into(self) -> Languages {
        Languages::new(self.main.clone(), self.others.clone())
    }
}

#[InputObject]
struct DateRangeInputType {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl Into<DateRange> for &DateRangeInputType {
    fn into(self) -> DateRange {
        DateRange::new(self.start, self.end)
    }
}

#[SimpleObject]
pub struct SiteInfoOutputType {
    id: EventId,
    name: String,
    address: AddressOutputType,
    languages: LanguagesOutputType,
    dates: DateRangeOutputType,
}

impl From<SiteInfo> for SiteInfoOutputType {
    fn from(info: SiteInfo) -> Self {
        let id = info.id();
        let name = info.name();
        let address = info.address().into();
        let languages = info.languages().into();
        let dates = info.dates().into();

        Self {
            id,
            name,
            address,
            languages,
            dates,
        }
    }
}

#[SimpleObject]
pub struct AddressOutputType {
    locality: NameOutputType,
    country: NameOutputType,
    lat_lng: GeolocationOutputType,
}

impl From<Address> for AddressOutputType {
    fn from(addr: Address) -> Self {
        Self {
            locality: addr.locality().into(),
            country: addr.country().into(),
            lat_lng: addr.lat_lng().into(),
        }
    }
}

#[SimpleObject]
pub struct NameOutputType {
    long_name: String,
    short_name: String,
}

impl From<Name> for NameOutputType {
    fn from(name: Name) -> Self {
        let long_name = name.long_name();
        let short_name = name.short_name();

        Self {
            long_name,
            short_name,
        }
    }
}

#[SimpleObject]
pub struct GeolocationOutputType {
    lat: f64,
    lng: f64,
}

impl From<Geolocation> for GeolocationOutputType {
    fn from(geoloc: Geolocation) -> Self {
        let lat = geoloc.lat();
        let lng = geoloc.lng();

        Self { lat, lng }
    }
}

#[SimpleObject]
pub struct DateRangeOutputType {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl From<DateRange> for DateRangeOutputType {
    fn from(range: DateRange) -> Self {
        let start = range.start();
        let end = range.end();

        Self { start, end }
    }
}
