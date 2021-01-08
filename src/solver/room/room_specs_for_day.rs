use crate::solver::surgeon::{Surgeon, SurgeonID};
use crate::solver::surgery::{Speciality, Surgery};
use std::collections::{HashMap, HashSet};

pub struct RoomSpecsForDay {
    day: usize,
    surgeries: Vec<Surgery>,
    speciality: Speciality,
    remaining_slots: u8,
}

impl RoomSpecsForDay {
    pub fn new(day: usize, surgery: Surgery) -> Self {
        let surgery_duration = surgery.duration;
        let surgery_speciality = surgery.speciality;
        Self {
            day,
            surgeries: vec![surgery],
            speciality: surgery_speciality,
            remaining_slots: 46 - surgery_duration,
        }
    }

    pub fn add_surgery(&mut self, surgery: Surgery) {
        if surgery.duration > self.remaining_slots - 2 {
            panic!("Tried to allocate a surgery on a room without sufficient slots")
        }
        if surgery.speciality != self.speciality {
            panic!(format!(
                "This room have speciality \"{}\" today and surgery has speciality \"{}\"",
                self.speciality, surgery.speciality
            ))
        }

        self.remaining_slots -= 2 + surgery.duration;
        self.surgeries.push(surgery);
    }
    pub fn have_available_slot(&self, surgery: &Surgery) -> bool {
        surgery.duration <= self.remaining_slots - 2 // We need 2 time slots to clean the room
    }

    pub fn have_speciality_today(&self, surgery: &Surgery) -> bool {
        self.speciality == surgery.speciality
    }

    pub fn available_surgeries(
        &self,
        surgeries: &HashSet<Surgery>,
        surgeons: &HashMap<SurgeonID, Surgeon>,
    ) -> HashSet<Surgery> {
        self.filter_already_scheduled_surgeries(surgeries)
            .into_iter()
            .filter(|surgery| {
                let surgeon = &surgeons[&surgery.surgeon_id];
                self.have_available_slot(surgery) && surgeon.is_available(surgery, self.day)
            })
            .collect()
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
