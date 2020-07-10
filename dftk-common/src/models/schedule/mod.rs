use std::ops::Range;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::session::SessionKey;
use crate::models::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScheduleDay {
    start: DateTime<Utc>,
    rooms: Vec<ScheduleRoom>,
}

impl ScheduleDay {
    pub fn start(&self) -> DateTime<Utc> {
        self.start
    }
    pub fn rooms(&self) -> &[ScheduleRoom] {
        self.rooms.as_slice()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct RoomKey(String);

impl RoomKey {
    pub fn new(s: &str) -> Self {
        Self(s.into())
    }
}

impl Into<String> for RoomKey {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScheduleRoom {
    room: RoomKey,
    slots: Vec<ScheduleRoomSlot>,
}

impl ScheduleRoom {
    pub fn room(&self) -> RoomKey {
        self.room.clone()
    }
    pub fn slots(&self) -> &[ScheduleRoomSlot] {
        self.slots.as_slice()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScheduleRoomSlot {
    slot: SlotKey,
    session: SessionKey,
}

impl ScheduleRoomSlot {
    pub fn slot(&self) -> SlotKey {
        self.slot.clone()
    }
    pub fn session(&self) -> SessionKey {
        self.session.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Slot {
    key: SlotKey,
    start: DateTime<Utc>,
    duration: Duration,
    row: Range<u32>,
}

impl Slot {
    pub fn key(&self) -> SlotKey {
        self.key.clone()
    }
    pub fn start(&self) -> DateTime<Utc> {
        self.start
    }
    pub fn duration(&self) -> Duration {
        self.duration
    }
    pub fn row(&self) -> Range<u32> {
        self.row.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct SlotKey(String);

impl SlotKey {
    pub fn new(s: &str) -> Self {
        Self(s.into())
    }
}

impl Into<String> for SlotKey {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Room {
    key: RoomKey,
    label: String,
    description: Option<String>,
    skip: bool,
}

impl Room {
    pub fn key(&self) -> RoomKey {
        self.key.clone()
    }
    pub fn label(&self) -> String {
        self.label.clone()
    }
    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }
    pub fn skip(&self) -> bool {
        self.skip
    }
}
