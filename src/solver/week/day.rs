use std::collections::HashMap;

use super::room_per_day::RoomPerDay;
use crate::solver::surgeon::{SurgeonDaily, SurgeonID};
use crate::solver::surgery::Surgery;

pub struct Day {
    rooms: Vec<RoomPerDay>,
    daily_surgeons: HashMap<SurgeonID, SurgeonDaily>,
}

impl Day {
    pub fn new(rooms_count: usize, surgeon_ids: &[SurgeonID]) -> Self {
        Self {
            rooms: Vec::with_capacity(rooms_count),
            daily_surgeons: SurgeonDaily::many_from_ids(surgeon_ids),
        }
    }

    pub fn can_schedule_surgery(&self, surgery: &Surgery) -> bool {
        self.daily_surgeons[&surgery.surgeon_id].has_availability(surgery)
            && (self.rooms.len() < self.rooms.capacity()
                || self
                    .rooms
                    .iter()
                    .any(|room| room.can_schedule_surgery(surgery)))
    }

    pub fn schedule_surgery(&mut self, surgery: Surgery) {
        if !self.can_schedule_surgery(&surgery) {
            panic!("Tried to allocate a surgery on a full day");
        }

        self.daily_surgeons
            .get_mut(&surgery.surgeon_id)
            .unwrap()
            .allocate(&surgery);

        // We already tested that we can schedule a surgery,
        // so if we have no room available, its because we can create a new room and schedule
        // surgery in this room
        match self
            .rooms
            .iter_mut()
            .filter(|room| room.can_schedule_surgery(&surgery))
            .next()
        {
            Some(room) => room.schedule_surgery(surgery),
            None => self.rooms.push(RoomPerDay::new(surgery)),
        }
    }
}