use std::collections::HashMap;

use super::room_per_day::RoomPerDay;
use crate::solver::surgeon::{SurgeonDaily, SurgeonID};
use crate::solver::surgery::Surgery;

#[derive(Debug, Clone)]
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

    pub fn rooms(&self) -> &Vec<RoomPerDay> {
        &self.rooms
    }

    pub fn surgeries(&self) -> Vec<Surgery> {
        self.rooms
            .iter()
            .map(|room| room.surgeries())
            .flatten()
            .cloned()
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.rooms.iter().all(|room| room.surgeries().is_empty())
    }

    pub fn can_schedule_surgery(&self, surgery: &Surgery) -> bool {
        self.daily_surgeons[&surgery.surgeon_id].has_availability(surgery)
            && (self.rooms.len() < self.rooms.capacity()
                || self
                    .rooms
                    .iter()
                    .any(|room| room.can_schedule_surgery(surgery)))
    }

    pub fn schedule_surgery(&mut self, surgery: Surgery) -> (usize, usize) {
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
            .enumerate()
            .find(|index_room| index_room.1.can_schedule_surgery(&surgery))
        {
            Some(index_room) => (index_room.0, index_room.1.schedule_surgery(surgery)),
            None => {
                self.rooms.push(RoomPerDay::new(surgery));
                (self.rooms.len() - 1, 0)
            }
        }
    }

    pub fn unschedule_surgery(
        &mut self,
        room_index: usize,
        surgery_index: usize,
        surgery: &Surgery,
    ) {
        self.daily_surgeons
            .get_mut(&surgery.surgeon_id)
            .unwrap()
            .deallocate(&surgery);

        self.rooms[room_index].unschedule_surgery(surgery_index, surgery);
    }
}
