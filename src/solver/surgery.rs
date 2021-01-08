use std::hash::{Hash, Hasher};

pub type Speciality = u32;
pub type Priority = usize;
pub type DaysWaiting = u32;

use std::collections::HashMap;

#[derive(Clone)]
pub struct Surgery {
    id: usize,
    pub duration: u8,
    pub days_waiting: DaysWaiting,
    pub priority: Priority,
    pub speciality: Speciality,
    pub surgeon_id: usize,
}

impl Hash for Surgery {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq for Surgery {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Surgery {}

impl Surgery {
    pub fn new(
        id: usize,
        duration: u8,
        days_waiting: DaysWaiting,
        priority: Priority,
        speciality: Speciality,
        surgeon_id: usize,
    ) -> Self {
        Self {
            id,
            duration,
            days_waiting,
            priority,
            speciality,
            surgeon_id,
        }
    }

    pub fn scheduled_objective_function(
        &self,
        max_days_waiting: &HashMap<Priority, DaysWaiting>,
        day: u32,
    ) -> f64 {
        let days_waited = self.days_waiting + 2 + day;
        let my_max_days_waiting = max_days_waiting[&self.priority];

        if my_max_days_waiting > days_waited {
            days_waited.pow(2).into()
        } else {
            (days_waited.pow(2) + (days_waited - my_max_days_waiting).pow(2)).into()
        }
    }

    pub fn not_scheduled_objective_function(
        &self,
        max_days_waiting: &HashMap<Priority, DaysWaiting>,
        priority_penalties: &HashMap<Priority, u32>,
    ) -> f64 {
        let my_max_days_waiting = max_days_waiting[&self.priority];
        if my_max_days_waiting > self.days_waiting + 9 {
            (self.days_waiting + 7).pow(2).into()
        } else {
            (((self.days_waiting + 7).pow(2)
                + (self.days_waiting + 9 - my_max_days_waiting).pow(2))
                * priority_penalties[&self.priority])
                .into()
        }
    }

    pub fn penalty_for_not_scheduling_on_first_day(&self, day: u32) -> f64 {
        if self.priority == 1 {
            (10 * (self.days_waiting + 2)).pow(day).into()
        } else {
            0.0
        }
    }
}
