use crate::solver::surgeon::SurgeonID;
use crate::solver::surgery::{Speciality, Surgery};
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct RoomPerDay {
    surgeries: Vec<Surgery>,
    scheduled_surgeons: Vec<(Range<u8>, SurgeonID)>,
    speciality: Speciality,
    remaining_slots: u8,
}

impl RoomPerDay {
    pub fn new(first_surgery: Surgery) -> Self {
        let surgery_duration = first_surgery.duration;
        let surgery_speciality = first_surgery.speciality;
        let surgeon_id = first_surgery.surgeon_id;
        Self {
            surgeries: vec![first_surgery],
            scheduled_surgeons: vec![(1..(2 + surgery_duration), surgeon_id)],
            speciality: surgery_speciality,
            remaining_slots: 46 - surgery_duration,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.surgeries.is_empty()
    }

    pub fn can_schedule_surgeon_in_another_room(
        &self,
        other_room_schedule: &Range<u8>,
        surgeon_id: SurgeonID,
    ) -> bool {
        self.scheduled_surgeons
            .iter()
            .filter(|range_surgeon| range_surgeon.1 == surgeon_id)
            .all(|range| {
                !range.0.contains(&other_room_schedule.start)
                    && !range.0.contains(&other_room_schedule.end)
            })
    }

    pub fn surgeries(&self) -> &Vec<Surgery> {
        &self.surgeries
    }

    pub fn can_schedule_surgery(&self, surgery: &Surgery) -> bool {
        // We need 2 time slots to clean the room and room must have the desired speciality for today
        self.speciality == surgery.speciality && surgery.duration + 2 <= self.remaining_slots
    }

    pub fn when_will_schedule(&self, surgery: &Surgery) -> Range<u8> {
        let last_time = self.scheduled_surgeons.last().unwrap().0.end;
        last_time..(last_time + 2 + surgery.duration)
    }

    pub fn schedule_surgery(&mut self, surgery: Surgery) -> usize {
        if surgery.duration + 2 > self.remaining_slots {
            panic!("Tried to allocate a surgery on a week without sufficient slots")
        }
        if surgery.speciality != self.speciality {
            panic!(
                "This week have speciality \"{}\" today and surgery has speciality \"{}\"",
                self.speciality, surgery.speciality
            )
        }
        let last_time = self.scheduled_surgeons.last().unwrap().0.end;
        let surgeon_id = surgery.surgeon_id;
        let surgery_duration = surgery.duration;
        self.remaining_slots -= 2 + surgery.duration;
        self.surgeries.push(surgery);
        self.scheduled_surgeons
            .push((last_time..(last_time + 2 + surgery_duration), surgeon_id));
        debug_assert!(self.surgeries.len() == self.scheduled_surgeons.len());

        self.surgeries.len() - 1
    }

    pub fn unschedule_surgery(&mut self, surgery_index: usize, surgery: &Surgery) {
        self.remaining_slots += 2 + surgery.duration;
        self.surgeries.remove(surgery_index);
        self.scheduled_surgeons.remove(surgery_index);
    }
}
