use crossbeam::channel::{Receiver, Sender};
use rand::distributions::{Distribution, WeightedIndex};
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::solver::surgeon::SurgeonID;
use crate::solver::surgery::{DaysWaiting, Priority, Surgery};
use crate::solver::week::Week;

pub struct AntFindSolutionData {
    pub pheromones: Arc<HashMap<(Surgery, Surgery), f64>>,
    pub round_number: u32,
}

pub struct AntSolution {
    pub objective_function_result: f64,
    pub all_weeks_results: Vec<(Week, f64)>,
    pub followed_path: Vec<(Surgery, Surgery)>,
}

pub struct Ant {
    alpha: f64,
    beta: f64,
    pheromone_evaporation_rate: f64,
    rooms_count: usize,
    surgeries_bin: HashSet<Surgery>,
    surgeons_ids: Arc<Vec<SurgeonID>>,
    max_days_waiting: Arc<HashMap<Priority, DaysWaiting>>,
    priority_penalties: Arc<HashMap<Priority, u32>>,
    random_number_generator: ThreadRng,
    receive_work: Receiver<Option<AntFindSolutionData>>,
    send_solution: Sender<AntSolution>,
}

impl Ant {
    pub fn new(
        alpha: f64,
        beta: f64,
        pheromone_evaporation_rate: f64,
        rooms_count: usize,
        surgeries_bin: HashSet<Surgery>,
        surgeons_ids: Arc<Vec<SurgeonID>>,
        max_days_waiting: Arc<HashMap<Priority, DaysWaiting>>,
        priority_penalties: Arc<HashMap<Priority, u32>>,
        receive_work: Receiver<Option<AntFindSolutionData>>,
        send_solution: Sender<AntSolution>,
    ) -> Self {
        Self {
            alpha,
            beta,
            pheromone_evaporation_rate,
            rooms_count,
            surgeries_bin,
            surgeons_ids: surgeons_ids.clone(),
            max_days_waiting,
            priority_penalties,
            random_number_generator: Default::default(),
            receive_work,
            send_solution,
        }
    }

    fn choose_first_surgery(
        &mut self,
        surgeries_bin: &mut HashSet<Surgery>,
        current_week: &mut Option<Week>,
        current_surgery: &mut Option<Surgery>,
    ) {
        let surgeries_and_weights = surgeries_bin
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

        if let Some(ref mut week) = current_week {
            week.schedule_surgery(chosen.clone());
        }

        surgeries_bin.remove(&chosen);

        *current_surgery = Some(chosen)
    }

    fn choose_next_surgery(
        &mut self,
        round_number: u32,
        pheromones: Arc<HashMap<(Surgery, Surgery), f64>>,
        surgeries_bin: &mut HashSet<Surgery>,
        path: &mut Vec<(Surgery, Surgery)>,
        current_week: &mut Option<Week>,
        past_weeks: &mut Vec<(Week, f64)>,
        visited_surgeries: &mut HashSet<Surgery>,
        current_surgery: &mut Option<Surgery>,
    ) {
        // First surgery for this ant
        if current_surgery.is_none() {
            self.choose_first_surgery(surgeries_bin, current_week, current_surgery)
        } else {
            // All other surgeries
            let mut inner_current_week = current_week.take().unwrap();
            let week_index = past_weeks.len();

            let current_objective_function = inner_current_week.calculate_objective_function(
                &surgeries_bin,
                self.max_days_waiting.clone(),
                self.priority_penalties.clone(),
                week_index,
            );

            let available_surgeries = inner_current_week.filter_available_surgeries(&surgeries_bin);

            let inner_current_surgery = current_surgery.take().unwrap();

            let heuristic_values = available_surgeries
                .iter()
                .map(|surgery| {
                    let key = (inner_current_surgery.clone(), surgery.clone());
                    let pheromone = if pheromones.contains_key(&key) {
                        pheromones[&key]
                    } else {
                        (1.0 - self.pheromone_evaporation_rate).powf((round_number - 1) as f64)
                    };
                    let schedule_token = inner_current_week.schedule_surgery(surgery.clone());
                    let objective_function_with_surgery = inner_current_week
                        .calculate_objective_function(
                            &surgeries_bin,
                            self.max_days_waiting.clone(),
                            self.priority_penalties.clone(),
                            week_index,
                        );
                    inner_current_week.unschedule_surgery(schedule_token, surgery);
                    let heuristic = current_objective_function - objective_function_with_surgery;

                    pheromone.powf(self.alpha) * heuristic.powf(self.beta)
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
                    let key = (inner_current_surgery.clone(), surgery.clone());
                    let pheromone = if pheromones.contains_key(&key) {
                        pheromones[&key]
                    } else {
                        (1.0 - self.pheromone_evaporation_rate).powf((round_number - 1) as f64)
                    };
                    let schedule_token = inner_current_week.schedule_surgery(surgery.clone());
                    let objective_function_with_surgery = inner_current_week
                        .calculate_objective_function(
                            &surgeries_bin,
                            self.max_days_waiting.clone(),
                            self.priority_penalties.clone(),
                            week_index,
                        );
                    inner_current_week.unschedule_surgery(schedule_token, surgery);
                    let heuristic = current_objective_function - objective_function_with_surgery;

                    (
                        surgery,
                        pheromone.powf(self.alpha) * heuristic.powf(self.beta),
                    )
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

            path.push((inner_current_surgery, next_surgery.clone()));
            inner_current_week.schedule_surgery(next_surgery.clone());
            surgeries_bin.remove(&next_surgery);
            *current_surgery = Some(next_surgery);

            // If week is full, self.current_week will be a new week
            if inner_current_week.is_full(&surgeries_bin) {
                let objective_function = inner_current_week.calculate_objective_function(
                    &surgeries_bin,
                    self.max_days_waiting.clone(),
                    self.priority_penalties.clone(),
                    week_index,
                );
                past_weeks.push((inner_current_week, objective_function));
                *current_week = Some(Week::new(self.rooms_count, self.surgeons_ids.clone()))
            } else {
                // Otherwise, self.current_week remais the same current_week
                *current_week = Some(inner_current_week);
            }
        }

        visited_surgeries.insert(current_surgery.clone().unwrap());
    }

    pub fn work(mut self) {
        while let Some(AntFindSolutionData {
            pheromones,
            round_number,
        }) = self
            .receive_work
            .recv()
            .expect("Failed to receive data to find solution")
        {
            let mut current_surgeries_bin = self.surgeries_bin.clone();
            let mut path = Vec::new();
            let mut current_week = Some(Week::new(self.rooms_count, self.surgeons_ids.clone()));
            let mut past_weeks = vec![];
            let mut visited_surgeries = HashSet::new();
            let mut current_surgery: Option<Surgery> = None;

            while !current_surgeries_bin.is_empty() {
                self.choose_next_surgery(
                    round_number,
                    pheromones.clone(),
                    &mut current_surgeries_bin,
                    &mut path,
                    &mut current_week,
                    &mut past_weeks,
                    &mut visited_surgeries,
                    &mut current_surgery,
                );
            }
            let current_week = current_week.take().unwrap();
            let current_week_objective_function = current_week.calculate_objective_function(
                &current_surgeries_bin,
                self.max_days_waiting.clone(),
                self.priority_penalties.clone(),
                past_weeks.len(),
            );
            past_weeks.push((current_week, current_week_objective_function));

            self.send_solution
                .send(AntSolution {
                    objective_function_result: past_weeks[0].1,
                    all_weeks_results: past_weeks,
                    followed_path: path,
                })
                .expect("Failed to send ant solution");
        }
    }
}
