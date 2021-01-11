use crate::solver::surgeon::SurgeonID;
use crate::solver::surgery::{Speciality, Surgery};
use std::ops::Range;

const LAST_SLOT: u8 = 48;

#[derive(Debug, Clone)]
pub struct RoomPerDay {
    surgeries: Vec<Surgery>,
    scheduled_surgeons: Vec<(Range<u8>, SurgeonID)>,
    speciality: Speciality,
    current_used_slots: u8,
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
            current_used_slots: 2 + surgery_duration,
        }
    }

    pub fn new_by_given_schedule(surgery: Surgery, schedule: Range<u8>) -> Self {
        let surgery_duration = surgery.duration;
        let surgery_speciality = surgery.speciality;
        let surgeon_id = surgery.surgeon_id;
        Self {
            surgeries: vec![surgery],
            scheduled_surgeons: vec![(schedule, surgeon_id)],
            speciality: surgery_speciality,
            current_used_slots: 2 + surgery_duration,
        }
    }

    pub fn scheduled_surgeons(&self) -> &Vec<(Range<u8>, SurgeonID)> {
        &self.scheduled_surgeons
    }

    pub fn is_empty(&self) -> bool {
        self.surgeries.is_empty()
    }

    pub fn surgeries(&self) -> &Vec<Surgery> {
        &self.surgeries
    }

    pub fn can_schedule_surgery(&self, surgery: &Surgery) -> bool {
        // We need 2 time slots to clean the room and room must have the desired speciality for today
        self.speciality == surgery.speciality
            && self.current_used_slots + surgery.duration + 2 <= LAST_SLOT
            && self
                .scheduled_surgeons
                .iter()
                .map(|schedule| schedule.0.end)
                .max()
                .unwrap()
                + surgery.duration
                + 2
                <= LAST_SLOT
    }

    pub fn when_will_schedule(&self, surgery: &Surgery) -> Range<u8> {
        let last_time = self.scheduled_surgeons.last().unwrap().0.end;
        last_time..(last_time + 2 + surgery.duration)
    }

    pub fn schedule_surgery(&mut self, surgery: Surgery) -> usize {
        debug_assert!(
            self.can_schedule_surgery(&surgery),
            "Tried to allocate a surgery on a day without sufficient slots or with different speciality"
        );

        let last_time = self
            .scheduled_surgeons
            .iter()
            .map(|schedule| schedule.0.end)
            .max()
            .unwrap();
        let surgeon_id = surgery.surgeon_id;
        let surgery_duration = surgery.duration;

        self.current_used_slots += 2 + surgery.duration;
        self.surgeries.push(surgery);
        self.scheduled_surgeons
            .push((last_time..(last_time + 2 + surgery_duration), surgeon_id));
        debug_assert!(self.surgeries.len() == self.scheduled_surgeons.len());

        self.surgeries.len() - 1
    }

    pub fn unschedule_surgery(&mut self, surgery_index: usize, surgery: &Surgery) {
        self.current_used_slots -= 2 + surgery.duration;
        self.surgeries.remove(surgery_index);
        self.scheduled_surgeons.remove(surgery_index);
    }
}
