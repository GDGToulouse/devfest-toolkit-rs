use std::ops::Range;

use async_graphql::{Context, FieldResult, Object, SimpleObject};
use chrono::{DateTime, Utc};

use dftk_common::models::schedule::{
    Room, RoomKey, ScheduleDay, ScheduleRoom, ScheduleRoomSlot, Slot, SlotKey,
};
use dftk_common::models::session::SessionKey;
use dftk_database::Repositories;

pub struct ScheduleOutputType;

#[Object]
impl ScheduleOutputType {
    async fn rooms(&self, ctx: &Context<'_>) -> FieldResult<Vec<RoomOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.room().find_all().await?;
        let result = result.iter().map(|it| it.into()).collect();

        Ok(result)
    }

    async fn slots(&self, ctx: &Context<'_>) -> FieldResult<Vec<SlotOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.slot().find_all().await?;
        let result = result.iter().map(|it| it.into()).collect();

        Ok(result)
    }

    async fn schedule(&self, ctx: &Context<'_>) -> FieldResult<Vec<ScheduleDayOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.schedule().find_all().await?;
        let result = result.iter().map(|it| it.into()).collect();

        Ok(result)
    }
}

#[SimpleObject]
pub struct ScheduleDayOutputType {
    start: DateTime<Utc>,
    rooms: Vec<ScheduleRoomOutputType>,
}

impl From<&ScheduleDay> for ScheduleDayOutputType {
    fn from(sd: &ScheduleDay) -> Self {
        let sd = sd.clone();

        Self {
            start: sd.start(),
            rooms: sd.rooms().iter().map(|it| it.into()).collect(),
        }
    }
}

#[SimpleObject]
pub struct ScheduleRoomOutputType {
    room: RoomKey,
    slots: Vec<ScheduleRoomSlotOutputType>,
}

impl From<&ScheduleRoom> for ScheduleRoomOutputType {
    fn from(sr: &ScheduleRoom) -> Self {
        let sr = sr.clone();

        Self {
            room: sr.room(),
            slots: sr.slots().iter().map(|it| it.into()).collect(),
        }
    }
}

#[SimpleObject]
pub struct ScheduleRoomSlotOutputType {
    slot: SlotKey,
    session: SessionKey,
}

impl From<&ScheduleRoomSlot> for ScheduleRoomSlotOutputType {
    fn from(rs: &ScheduleRoomSlot) -> Self {
        let rs = rs.clone();

        Self {
            slot: rs.slot(),
            session: rs.session(),
        }
    }
}

#[SimpleObject]
pub struct SlotOutputType {
    key: SlotKey,
    start: DateTime<Utc>,
    duration: u8,
    row: SlotRangeOutputType,
}

impl From<&Slot> for SlotOutputType {
    fn from(slot: &Slot) -> Self {
        let slot = slot.clone();

        Self {
            key: slot.key(),
            start: slot.start(),
            duration: slot.duration(),
            row: slot.row().into(),
        }
    }
}

#[SimpleObject]
pub struct SlotRangeOutputType {
    start: u32,
    end: u32,
}

impl From<Range<u32>> for SlotRangeOutputType {
    fn from(range: Range<u32>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

#[SimpleObject]
pub struct RoomOutputType {
    key: RoomKey,
    label: String,
    description: Option<String>,
    skip: bool,
}

impl From<&Room> for RoomOutputType {
    fn from(room: &Room) -> Self {
        let room = room.clone();

        Self {
            key: room.key(),
            label: room.label(),
            description: room.description(),
            skip: room.skip(),
        }
    }
}
