use std::collections::HashMap;

use super::surgery::Surgery;

pub type SurgeonID = usize;

#[derive(Debug, Clone)]
pub struct SurgeonWeekly {
    max_week_time: u8,
    current_week_time: u8,
}

impl SurgeonWeekly {
    pub fn new() -> Self {
        Self {
            max_week_time: 100,
            current_week_time: 0,
        }
    }

    pub fn has_availability(&self, surgery: &Surgery) -> bool {
        self.current_week_time + surgery.duration <= self.max_week_time
    }

    pub fn allocate(&mut self, surgery: &Surgery) {
        if self.current_week_time + surgery.duration > self.max_week_time {
            panic!("Tried to allocate a surgery that surpasses surgeon max weekly time!");
        }

        self.current_week_time += surgery.duration;
    }

    pub fn deallocate(&mut self, surgery: &Surgery) {
        if self.current_week_time < surgery.duration {
            panic!("Tried to deallocate a surgery that has not been allocated");
        }

        self.current_week_time -= surgery.duration;
    }

    pub fn many_from_ids(ids: &[SurgeonID]) -> HashMap<SurgeonID, SurgeonWeekly> {
        ids.iter().map(|id| (*id, Self::new())).collect()
    }
}

#[derive(Debug, Clone)]
pub struct SurgeonDaily {
    max_day_time: u8,
    current_day_time: u8,
}

impl SurgeonDaily {
    pub fn new() -> Self {
        Self {
            max_day_time: 24,
            current_day_time: 0,
        }
    }

    pub fn has_availability(&self, surgery: &Surgery) -> bool {
        self.current_day_time + surgery.duration <= self.max_day_time
    }

    pub fn allocate(&mut self, surgery: &Surgery) {
        if self.current_day_time + surgery.duration > self.max_day_time {
            panic!("Tried to allocate a surgery that surpasses surgeon max daily time!");
        }

        self.current_day_time += surgery.duration;
    }

    pub fn deallocate(&mut self, surgery: &Surgery) {
        if self.current_day_time < surgery.duration {
            panic!("Tried to deallocate a surgery that has not been allocated");
        }

        self.current_day_time -= surgery.duration;
    }

    pub fn many_from_ids(ids: &[SurgeonID]) -> HashMap<SurgeonID, SurgeonDaily> {
        ids.iter().map(|id| (*id, Self::new())).collect()
    }
}
