use std::collections::HashMap;

use super::surgery::Surgery;
use std::ops::Range;

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
    scheduled_times: Vec<(Range<u8>, Surgery)>,
}

impl SurgeonDaily {
    pub fn new() -> Self {
        Self {
            max_day_time: 24,
            current_day_time: 0,
            scheduled_times: vec![],
        }
    }

    pub fn has_availability(&self, surgery: &Surgery) -> bool {
        self.current_day_time + surgery.duration <= self.max_day_time
    }

    pub fn can_be_allocated(&self, schedule_time: &Range<u8>) -> bool {
        self.scheduled_times.iter().all(|scheduled| {
            scheduled.0.end < schedule_time.start || scheduled.0.start > schedule_time.end
        })
    }

    pub fn last_scheduled_time(&self) -> Range<u8> {
        self.scheduled_times.last().unwrap().0.clone()
    }

    pub fn allocate_next(&mut self, surgery: Surgery) {
        if self.current_day_time + surgery.duration > self.max_day_time {
            panic!("Tried to allocate a surgery that surpasses surgeon max daily time!");
        }

        self.current_day_time += surgery.duration;
        if self.scheduled_times.is_empty() {
            self.scheduled_times
                .push((1..(surgery.duration + 2), surgery));
        } else {
            let last_time = self.scheduled_times.last().unwrap().0.end;
            self.scheduled_times
                .push((last_time..(last_time + 2 + surgery.duration), surgery));
        }
    }

    pub fn allocate_by_schedule(&mut self, schedule_time: Range<u8>, surgery: Surgery) {
        if self.current_day_time + surgery.duration > self.max_day_time {
            panic!("Tried to allocate a surgery that surpasses surgeon max daily time!");
        }
        self.current_day_time += surgery.duration;

        self.scheduled_times.push((schedule_time, surgery));
    }

    pub fn deallocate(&mut self, surgery: &Surgery) {
        if self.current_day_time < surgery.duration {
            panic!("Tried to deallocate a surgery that has not been allocated");
        }

        self.current_day_time -= surgery.duration;
        let to_remove_index = self
            .scheduled_times
            .iter()
            .enumerate()
            .find(|index_tuple| index_tuple.1 .1.id == surgery.id)
            .unwrap()
            .0;

        self.scheduled_times.remove(to_remove_index);
    }

    pub fn many_from_ids(ids: &[SurgeonID]) -> HashMap<SurgeonID, SurgeonDaily> {
        ids.iter().map(|id| (*id, Self::new())).collect()
    }
}
