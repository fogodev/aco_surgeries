pub mod day;
pub mod room_per_day;

use day::Day;

use crate::solver::surgeon::{SurgeonID, SurgeonWeekly};
use crate::solver::surgery::Surgery;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Default)]
pub struct Week {
    days: Vec<Day>,
    rooms_count: usize,
    weekly_surgeons: HashMap<SurgeonID, SurgeonWeekly>,
}

impl Week {
    pub fn new(rooms_count: usize, surgeon_ids: Arc<Vec<SurgeonID>>) -> Self {
        Self {
            days: Vec::with_capacity(5),
            rooms_count,
            weekly_surgeons: SurgeonWeekly::many_from_ids(&*surgeon_ids),
        }
    }

    pub fn can_schedule_surgery(&self, surgery: &Surgery) -> bool {
        self.weekly_surgeons[&surgery.surgeon_id].has_availability(surgery)
            && (self.days.len() <= self.days.capacity()
                || self
                    .days
                    .iter()
                    .any(|day| day.can_schedule_surgery(surgery)))
    }

    pub fn schedule_surgery(&mut self, surgery: Surgery) {
        if !self.can_schedule_surgery(&surgery) {
            panic!("Tried to schedule a surgery on a full week!");
        }

        self.weekly_surgeons
            .get_mut(&surgery.surgeon_id)
            .unwrap()
            .allocate(&surgery);

        // We already tested that we can schedule a surgery,
        // so if we have no day slots available, its because we can create a new day and schedule
        // surgery in this day
        match self
            .days
            .iter_mut()
            .filter(|day| day.can_schedule_surgery(&surgery))
            .next()
        {
            Some(day) => day.schedule_surgery(surgery),
            None => {
                let mut day = Day::new(
                    self.rooms_count,
                    &self.weekly_surgeons.keys().cloned().collect::<Vec<_>>(),
                );
                day.schedule_surgery(surgery);
                self.days.push(day)
            }
        }
    }

    pub fn filter_available_surgeries(&self, surgeries: &HashSet<Surgery>) -> HashSet<Surgery> {
        surgeries
            .iter()
            .filter(|&surgery| self.can_schedule_surgery(surgery))
            .cloned()
            .collect()
    }

    pub fn is_full(&self, surgeries: &HashSet<Surgery>) -> bool {
        !surgeries.is_empty() && self.filter_available_surgeries(surgeries).is_empty()
    }
}
