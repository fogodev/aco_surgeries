pub mod room_specs_for_day;
pub mod week;

use crate::solver::room::room_specs_for_day::RoomSpecsForDay;
use week::Week;

pub struct Room {
    weeks: Vec<Week>,
}

impl Room {
    pub fn new() -> Self {
        Self { weeks: vec![] }
    }

    pub fn open_new_week(&mut self) {
        self.weeks.push(Week::new())
    }

    pub fn add_day(&mut self, room_specs_for_day: RoomSpecsForDay) {
        if self.weeks.is_empty() {
            self.open_new_week()
        }
        let current_week = self.weeks.last_mut().unwrap();
        if current_week.is_full() {
            panic!("Tried to add a new day on an already full week");
        }

        current_week.add_day(room_specs_for_day)
    }
}
