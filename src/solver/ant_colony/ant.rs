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
    past_weeks: Vec<(Week, f64)>,
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
            week.schedule_surgery(chosen.clone());
        }

        self.surgeries_bin.remove(&chosen);

        self.current_surgery = Some(chosen)
    }

    fn choose_next_surgery(
        &mut self,
        alpha: f64,
        beta: f64,
        pheromones: &HashMap<(Surgery, Surgery), f64>,
        pheromone_evaporation_rate: f64,
        round_number: u32,
    ) {
        // First surgery for this ant
        if self.current_surgery.is_none() {
            self.choose_first_surgery()
        } else {
            // All other surgeries
            let mut current_week = self.current_week.take().unwrap();
            let week_index = self.past_weeks.len();

            let current_objective_function = current_week.calculate_objective_function(
                &self.surgeries_bin,
                self.max_days_waiting.clone(),
                self.priority_penalties.clone(),
                week_index,
            );

            let available_surgeries = current_week.filter_available_surgeries(&self.surgeries_bin);

            let current_surgery = self.current_surgery.take().unwrap();

            let heuristic_values = available_surgeries
                .iter()
                .map(|surgery| {
                    let key = (current_surgery.clone(), surgery.clone());
                    let pheromone = if pheromones.contains_key(&key) {
                        pheromones[&key]
                    } else {
                        (1.0 - pheromone_evaporation_rate).powf((round_number - 1) as f64)
                    };
                    let scheduled_day = current_week.schedule_surgery(surgery.clone());
                    let objective_function_with_surgery = current_week
                        .calculate_objective_function(
                            &self.surgeries_bin,
                            self.max_days_waiting.clone(),
                            self.priority_penalties.clone(),
                            week_index,
                        );
                    current_week.unschedule_surgery(scheduled_day, surgery);
                    let heuristic = current_objective_function - objective_function_with_surgery;

                    pheromone.powf(alpha) * heuristic.powf(beta)
                })
                .collect::<Vec<f64>>();

            let smallest_value = heuristic_values
                .iter()
                .fold(f64::INFINITY, |a, &b| a.min(b));
            let summation = heuristic_values
                .into_iter()
                .map(|value| value - smallest_value + 0.1)
                .sum::<f64>();

            let mut surgeries_probability = available_surgeries
                .iter()
                .map(|surgery| {
                    let key = (current_surgery.clone(), surgery.clone());
                    let pheromone = if pheromones.contains_key(&key) {
                        pheromones[&key]
                    } else {
                        (1.0 - pheromone_evaporation_rate).powf((round_number - 1) as f64)
                    };
                    let scheduled_day = current_week.schedule_surgery(surgery.clone());
                    let objective_function_with_surgery = current_week
                        .calculate_objective_function(
                            &self.surgeries_bin,
                            self.max_days_waiting.clone(),
                            self.priority_penalties.clone(),
                            week_index,
                        );
                    current_week.unschedule_surgery(scheduled_day, surgery);
                    let heuristic = current_objective_function - objective_function_with_surgery;

                    (surgery, pheromone.powf(alpha) * heuristic.powf(beta))
                })
                .collect::<Vec<_>>();

            let smallest_value = surgeries_probability
                .iter()
                .map(|value| value.1)
                .fold(f64::INFINITY, |a, b| a.min(b));
            surgeries_probability.iter_mut().for_each(|value| {
                value.1 = (value.1 - smallest_value + 0.1) / summation;
            });

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
                let objective_function = current_week.calculate_objective_function(
                    &self.surgeries_bin,
                    self.max_days_waiting.clone(),
                    self.priority_penalties.clone(),
                    week_index,
                );
                self.past_weeks.push((current_week, objective_function));
                self.current_week = Some(Week::new(self.rooms_count, self.surgeons_ids.clone()))
            } else {
                // Otherwise, self.current_week remais the same current_week
                self.current_week = Some(current_week);
            }
        }

        self.visited_surgeries
            .insert(self.current_surgery.clone().unwrap().clone());
    }

    pub fn find_solution(
        mut self,
        alpha: f64,
        beta: f64,
        pheromones: &HashMap<(Surgery, Surgery), f64>,
        pheromone_evaporation_rate: f64,
        round_number: u32,
    ) -> (f64, Vec<(Week, f64)>) {
        while !self.surgeries_bin.is_empty() {
            self.choose_next_surgery(
                alpha,
                beta,
                pheromones,
                pheromone_evaporation_rate,
                round_number,
            );
        }
        let current_week = self.current_week.take().unwrap();
        let current_week_objective_function = current_week.calculate_objective_function(
            &self.surgeries_bin,
            self.max_days_waiting.clone(),
            self.priority_penalties.clone(),
            self.past_weeks.len(),
        );
        self.past_weeks
            .push((current_week, current_week_objective_function));

        (self.past_weeks[0].1, self.past_weeks)
    }
}
