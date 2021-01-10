pub mod day;
pub mod room_per_day;

use day::Day;

use crate::solver::surgeon::{SurgeonID, SurgeonWeekly};
use crate::solver::surgery::{DaysWaiting, Priority, Surgery};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub struct ScheduleToken {
    day_index: usize,
    room_index: usize,
    surgery_index: usize,
}

#[derive(Default, Debug, Clone)]
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

    pub fn schedule_surgery(&mut self, surgery: Surgery) -> ScheduleToken {
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
            .enumerate()
            .find(|index_day| index_day.1.can_schedule_surgery(&surgery))
        {
            Some(index_day) => {
                let (room_index, surgery_index) = index_day.1.schedule_surgery(surgery);
                ScheduleToken {
                    day_index: index_day.0,
                    room_index,
                    surgery_index,
                }
            }
            None => {
                let mut day = Day::new(
                    self.rooms_count,
                    &self.weekly_surgeons.keys().cloned().collect::<Vec<_>>(),
                );
                let (room_index, surgery_index) = day.schedule_surgery(surgery);
                self.days.push(day);
                ScheduleToken {
                    day_index: self.days.len() - 1,
                    room_index,
                    surgery_index,
                }
            }
        }
    }

    pub fn unschedule_surgery(&mut self, schedule_token: ScheduleToken, surgery: &Surgery) {
        let ScheduleToken{day_index, room_index, surgery_index} = schedule_token;
        self.weekly_surgeons
            .get_mut(&surgery.surgeon_id)
            .unwrap()
            .deallocate(&surgery);

        self.days[day_index].unschedule_surgery(room_index, surgery_index, surgery);

        if self.days[day_index].is_empty() {
            self.days.remove(day_index);
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

    pub fn calculate_objective_function(
        &self,
        surgeries_bin: &HashSet<Surgery>,
        max_days_waiting: Arc<HashMap<Priority, DaysWaiting>>,
        priority_penalties: Arc<HashMap<Priority, u32>>,
        week_index: usize,
    ) -> f64 {
        let mut total_objective = 0.0;

        for (index, current_day) in self.days.iter().enumerate() {
            for surgery in current_day.surgeries() {
                let day = index as u32 + 1 + (7 * week_index as u32);
                total_objective +=
                    surgery.scheduled_objective_function(max_days_waiting.clone(), day);
                if day != 1 && surgery.priority == 1 {
                    total_objective += surgery.penalty_for_not_scheduling_on_first_day(day);
                }
            }
        }

        for surgery in surgeries_bin {
            total_objective += surgery.not_scheduled_objective_function(
                max_days_waiting.clone(),
                priority_penalties.clone(),
            )
        }

        total_objective
    }
}
