use crate::solver::surgery::{Speciality, Surgery};
use std::collections::HashSet;

pub struct RoomPerDay {
    surgeries: Vec<Surgery>,
    speciality: Speciality,
    remaining_slots: u8,
}

impl RoomPerDay {
    pub fn new(first_surgery: Surgery) -> Self {
        let surgery_duration = first_surgery.duration;
        let surgery_speciality = first_surgery.speciality;
        Self {
            surgeries: vec![first_surgery],
            speciality: surgery_speciality,
            remaining_slots: 46 - surgery_duration,
        }
    }

    pub fn can_schedule_surgery(&self, surgery: &Surgery) -> bool {
        // We need 2 time slots to clean the room and room must have the desired speciality for today
        self.speciality == surgery.speciality && surgery.duration <= self.remaining_slots - 2
    }

    pub fn schedule_surgery(&mut self, surgery: Surgery) {
        if surgery.duration > self.remaining_slots - 2 {
            panic!("Tried to allocate a surgery on a week without sufficient slots")
        }
        if surgery.speciality != self.speciality {
            panic!(format!(
                "This week have speciality \"{}\" today and surgery has speciality \"{}\"",
                self.speciality, surgery.speciality
            ))
        }

        self.remaining_slots -= 2 + surgery.duration;
        self.surgeries.push(surgery);
    }

    pub fn have_available_slot(&self, surgery: &Surgery) -> bool {
        surgery.duration <= self.remaining_slots - 2
    }

    pub fn have_speciality_today(&self, surgery: &Surgery) -> bool {
        self.speciality == surgery.speciality
    }

    pub fn filter_already_scheduled_surgeries(
        &self,
        surgeries: &HashSet<Surgery>,
    ) -> HashSet<Surgery> {
        let mut filtered = surgeries.clone();
        self.surgeries.iter().for_each(|surgery| {
            filtered.remove(surgery);
        });

        filtered
    }
}
