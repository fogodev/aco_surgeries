use crate::solver::room::Room;
use crate::solver::surgeon::{Surgeon, SurgeonID};
use crate::solver::surgery::{DaysWaiting, Priority, Surgery};
use rand::distributions::{Distribution, WeightedIndex};
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::solver::room::room_specs_for_day::RoomSpecsForDay;

pub struct Ant {
    path: Vec<(Surgery, Surgery)>,
    surgeries_bin: HashSet<Surgery>,
    surgeons: HashMap<SurgeonID, Surgeon>,
    rooms: Vec<Room>,
    visited_surgeries: HashSet<Surgery>,
    current_surgery: Option<Surgery>,
    max_days_waiting: Rc<HashMap<Priority, DaysWaiting>>,
    priority_penalties: Rc<HashMap<Priority, u32>>,
    random_number_generator: ThreadRng,
}

impl Ant {
    pub fn new(
        rooms_count: usize,
        surgeries_bin: HashSet<Surgery>,
        surgeons_ids: &[SurgeonID],
        max_days_waiting: Rc<HashMap<Priority, DaysWaiting>>,
        priority_penalties: Rc<HashMap<Priority, u32>>,
    ) -> Self {
        Self {
            path: Default::default(),
            surgeries_bin,
            surgeons: Surgeon::from_ids(surgeons_ids),
            rooms: (0..rooms_count).map(|_| Room::new()).collect(),
            visited_surgeries: Default::default(),
            current_surgery: None,
            max_days_waiting,
            priority_penalties,
            random_number_generator: Default::default(),
        }
    }

    pub fn choose_next_surgery(
        &mut self,
        alpha: f64,
        beta: f64,
        pheromones: &HashMap<(Surgery, Surgery), f64>,
        round_number: u32,
    ) {
        // First surgery for this ant
        if self.current_surgery.is_none() {
            let surgeries_and_weights = self
                .surgeries_bin
                .iter()
                .map(|surgery| (surgery, if surgery.priority == 1 { 2.0 } else { 1.0 }))
                .collect::<Vec<(&Surgery, f64)>>();

            let dist = WeightedIndex::new(
                surgeries_and_weights
                    .iter()
                    .map(|surgery_weight| surgery_weight.1),
            )
            .unwrap();

            let chosen = surgeries_and_weights[dist.sample(&mut self.random_number_generator)]
                .0
                .clone();

            self.rooms[0].add_day(RoomSpecsForDay::new(1, chosen.clone()));
            // Todo cirurgiões
            self.surgeries_bin.remove(&chosen);

            let surgeon = self.surgeons.get_mut(&chosen.surgeon_id).unwrap();
            surgeon.allocate_to_surgery(&chosen, 1);

            self.current_surgery = Some(chosen)
        } else {
            // All other surgeries

            let current_surgery = self.current_surgery.take().unwrap();
            let not_visited = self
                .surgeries_bin
                .difference(&self.visited_surgeries)
                .collect::<HashSet<&Surgery>>();

            let mut summation = 0.0;
            for possible_next_surgery in not_visited.iter() {
                if current_surgery != **possible_next_surgery {
                    let key = (current_surgery.clone(), (*possible_next_surgery).clone());
                    let pheromone = if pheromones.contains_key(&key) {
                        pheromones[&key]
                    } else {
                        // ToDo default value must decay by given round_number, check if this approach is ok
                        1.0 / round_number as f64
                    };
                    summation += pheromone.powf(alpha); // ToDo use heuristic function here * heuristic.powf(beta);
                }
            }

            let surgeries_probability = not_visited
                .iter()
                .filter(|&&possible_next_surgery| current_surgery != *possible_next_surgery)
                .map(|&possible_next_surgery| {
                    let key = (current_surgery.clone(), possible_next_surgery.clone());
                    let pheromone = if pheromones.contains_key(&key) {
                        pheromones[&key]
                    } else {
                        // ToDo default value must decay by given round_number, check if this approach is ok
                        1.0 / round_number as f64
                    };
                    // ToDo use heuristic funcion here
                    // let heuristic = heuristic_function(&current_surgery, possible_next_surgery);

                    (
                        possible_next_surgery,
                        // ToDo change this, heuristic function can return negative numbers
                        pheromone.powf(alpha) // ToDo use heuristic function result here  
                            // * heuristic.powf(beta) 
                            / summation,
                    )
                })
                .collect::<Vec<_>>();

            let next_surgery = surgeries_probability
                .choose_weighted(&mut self.random_number_generator, |surgery_probability| {
                    surgery_probability.1
                })
                .unwrap()
                .0
                .clone();

            self.path.push((current_surgery, next_surgery.clone()));

            self.current_surgery = Some(next_surgery)
        }

        self.visited_surgeries
            .insert(self.current_surgery.clone().unwrap().clone());
    }

    pub fn calculate_objective_function(
        &self,
        max_days_waiting: &HashMap<Priority, DaysWaiting>,
        priority_penalties: &HashMap<Priority, u32>,
    ) -> f64 {
        let mut total_objective = 0.0;

        // ToDo change this code to comply with current structures
        // for current_day in &self.daily_solutions {
        //     for surgery in &current_day.surgeries {
        //         total_objective +=
        //             surgery.scheduled_objective_function(max_days_waiting, current_day.day);
        //         if current_day.day != 1 && surgery.priority == 1 {
        //             total_objective +=
        //                 surgery.penalty_for_not_scheduling_on_first_day(current_day.day);
        //         }
        //     }
        // }
        //
        // for surgery in &self.surgeries_bin {
        //     total_objective +=
        //         surgery.not_scheduled_objective_function(max_days_waiting, priority_penalties)
        // }

        total_objective += 42.0;

        total_objective
    }

    pub fn has_unallocated_surgeries(&self) -> bool {
        !self.surgeries_bin.is_empty()
    }
}
