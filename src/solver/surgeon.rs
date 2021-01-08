use indoc::formatdoc;
use std::collections::HashMap;

use super::surgery::Surgery;

pub type SurgeonID = usize;

pub struct Surgeon {
    id: SurgeonID,
    max_time_day: u8,
    max_time_week: u8,
    weeks: Vec<[u8; 5]>,
}

impl Surgeon {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            max_time_day: 24,
            max_time_week: 100,
            weeks: vec![[0, 0, 0, 0, 0]],
        }
    }

    pub fn open_new_week(&mut self) {
        self.weeks.push([0, 0, 0, 0, 0])
    }

    pub fn is_available(&self, surgery: &Surgery, day: usize) -> bool {
        let current_week = self.weeks.last().unwrap();

        current_week[day] + surgery.duration <= self.max_time_day
            && current_week.iter().sum::<u8>() + surgery.duration <= self.max_time_week
    }

    pub fn allocate_to_surgery(&mut self, surgery: &Surgery, day: usize) {
        let current_week = self.weeks.last_mut().unwrap();

        if current_week[day] + surgery.duration > self.max_time_day
            || current_week.iter().sum::<u8>() + surgery.duration > self.max_time_week
        {
            panic!(formatdoc!(
                "Unable to allocate a surgery with duration bigger than available time for surgeon;
            Surgeon: {};
            Week: {};
            Day: {};",
                self.id,
                self.weeks.len(),
                day,
            ))
        }
        current_week[day] += surgery.duration;
    }

    pub fn from_ids(ids: &[SurgeonID]) -> HashMap<SurgeonID, Surgeon> {
        ids.iter().map(|id| (*id, Self::new(*id))).collect()
    }
}
