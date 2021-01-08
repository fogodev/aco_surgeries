use rand::distributions::{Distribution, WeightedIndex};
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::solver::surgeon::SurgeonID;
use crate::solver::surgery::{DaysWaiting, Priority, Surgery};
use crate::solver::week::Week;

pub struct Ant {
    path: Vec<(Surgery, Surgery)>,
    rooms_count: usize,
    surgeries_bin: HashSet<Surgery>,
    surgeons_ids: Arc<Vec<SurgeonID>>,
    current_week: Option<Week>,
    past_weeks: Vec<Week>,
    visited_surgeries: HashSet<Surgery>,
    current_surgery: Option<Surgery>,
    max_days_waiting: Arc<HashMap<Priority, DaysWaiting>>,
    priority_penalties: Arc<HashMap<Priority, u32>>,
    random_number_generator: ThreadRng,
}

impl Ant {
    pub fn new(
        rooms_count: usize,
        surgeries_bin: HashSet<Surgery>,
        surgeons_ids: Arc<Vec<SurgeonID>>,
        max_days_waiting: Arc<HashMap<Priority, DaysWaiting>>,
        priority_penalties: Arc<HashMap<Priority, u32>>,
    ) -> Self {
        Self {
            path: Default::default(),
            rooms_count,
            surgeries_bin,
            surgeons_ids: surgeons_ids.clone(),
            current_week: Some(Week::new(rooms_count, surgeons_ids.clone())),
            past_weeks: vec![],
            visited_surgeries: Default::default(),
            current_surgery: None,
            max_days_waiting,
            priority_penalties,
            random_number_generator: Default::default(),
        }
    }

    fn choose_first_surgery(&mut self) {
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

        if let Some(ref mut week) = self.current_week {
            week.schedule_surgery(chosen.clone())
        }

        self.surgeries_bin.remove(&chosen);

        self.current_surgery = Some(chosen)
    }

    fn choose_next_surgery(
        &mut self,
        alpha: f64,
        beta: f64,
        pheromones: &HashMap<(Surgery, Surgery), f64>,
        round_number: u32,
    ) {
        // First surgery for this ant
        if self.current_surgery.is_none() {
            self.choose_first_surgery()
        } else {
            // All other surgeries
            let mut current_week = self.current_week.take().unwrap();

            let available_surgeries = current_week.filter_available_surgeries(&self.surgeries_bin);

            let current_surgery = self.current_surgery.take().unwrap();

            let summation = available_surgeries
                .iter()
                .filter(|&surgery| current_surgery != *surgery)
                .map(|surgery| {
                    let key = (current_surgery.clone(), surgery.clone());
                    let pheromone = if pheromones.contains_key(&key) {
                        pheromones[&key]
                    } else {
                        // ToDo default value must decay by given round_number, check if this approach is ok
                        1.0 / round_number as f64
                    };
                    pheromone.powf(alpha) // ToDo use heuristic function here
                                          // * heuristic.powf(beta)
                })
                .sum::<f64>();

            let surgeries_probability = available_surgeries
                .iter()
                .filter(|&surgery| current_surgery != *surgery)
                .map(|surgery| {
                    let key = (current_surgery.clone(), surgery.clone());
                    let pheromone = if pheromones.contains_key(&key) {
                        pheromones[&key]
                    } else {
                        // ToDo default value must decay by given round_number, check if this approach is ok
                        1.0 / round_number as f64
                    };
                    // ToDo use heuristic funcion here
                    // let heuristic = heuristic_function(&current_surgery, possible_next_surgery);

                    (
                        surgery,
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
            current_week.schedule_surgery(next_surgery.clone());
            self.surgeries_bin.remove(&next_surgery);
            self.current_surgery = Some(next_surgery);

            // If week is full, self.current_week will be a new week
            if current_week.is_full(&self.surgeries_bin) {
                self.past_weeks.push(current_week);
                self.current_week = Some(Week::new(self.rooms_count, self.surgeons_ids.clone()))
            } else {
                // Otherwise, self.current_week remais the same current_week
                self.current_week = Some(current_week);
            }
        }

        self.visited_surgeries
            .insert(self.current_surgery.clone().unwrap().clone());
    }

    fn calculate_objective_function(
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

    pub fn find_solution(
        &mut self,
        alpha: f64,
        beta: f64,
        pheromones: &HashMap<(Surgery, Surgery), f64>,
        round_number: u32,
    ) -> f64 {
        while !self.surgeries_bin.is_empty() {
            self.choose_next_surgery(alpha, beta, pheromones, round_number);
        }

        self.past_weeks.push(self.current_week.take().unwrap());

        // ToDo calculate objective function
        // self.past_weeks.iter().map(|week| week.calculate_objective_function()).sum::<f64>()
        0.0
    }
}
