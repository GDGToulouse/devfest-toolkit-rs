use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};

use dftk_common::models::site::{Address, DateRange, EventId, Geolocation, Name, SiteInfo};

use crate::graphql::languages::LanguagesOutputType;

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
