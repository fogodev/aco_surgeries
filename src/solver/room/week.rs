use super::room_specs_for_day::RoomSpecsForDay;
use crate::solver::surgery::Surgery;

#[derive(Default)]
pub struct Week {
    days: Vec<RoomSpecsForDay>,
}

impl Week {
    pub fn new() -> Self {
        Self { days: vec![] }
    }

    pub fn add_day(&mut self, room_specs_for_day: RoomSpecsForDay) {
        if self.days.len() >= 5 {
            panic!("Tried to add a new day on a full week");
        }

        self.days.push(room_specs_for_day);
    }

    pub fn is_full(&self) -> bool {
        self.days.len() == 5
    }

    pub fn have_available_slots_for_day(&self, surgery: &Surgery) -> Option<usize> {
        self.days
            .iter()
            .enumerate()
            .filter(|index_day| index_day.1.have_available_slot(surgery))
            .map(|index_day| index_day.0)
            .next()
    }
}
