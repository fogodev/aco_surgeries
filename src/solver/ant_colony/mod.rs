mod ant;

use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use super::surgery::{DaysWaiting, Priority, Surgery};
use crate::solver::surgeon::SurgeonID;
use crate::solver::week::Week;
use ant::Ant;
use std::sync::Arc;

pub struct AntColony {
    ants_count: usize,
    rooms_count: usize,
    pheromones: HashMap<(Surgery, Surgery), f64>,
    pheromone_deposit_rate: f64,
    pheromone_evaporation_rate: f64,
    alpha: f64,
    beta: f64,
    elitism_factor: f64,
    surgeons_ids: Arc<Vec<SurgeonID>>,
    surgeries_bin: HashSet<Surgery>,
    max_days_waiting: Arc<HashMap<Priority, DaysWaiting>>,
    priority_penalties: Arc<HashMap<Priority, u32>>,
}

impl AntColony {
    pub fn new(
        ants_count: usize,
        rooms_count: usize,
        alpha: f64,
        beta: f64,
        elitism_factor: f64,
        pheromone_deposit_rate: f64,
        pheromone_evaporation_rate: f64,
        surgeons_ids: Vec<SurgeonID>,
        surgeries_bin: HashSet<Surgery>,
        max_days_waiting: HashMap<Priority, DaysWaiting>,
        priority_penalties: HashMap<Priority, u32>,
    ) -> Self {
        if surgeries_bin.is_empty() {
            panic!("Unable to solve for a empty set of surgeries!");
        }

        let max_days_waiting = Arc::new(max_days_waiting);
        let priority_penalties = Arc::new(priority_penalties);
        let surgeons_ids = Arc::new(surgeons_ids);

        Self {
            ants_count,
            rooms_count,
            pheromones: HashMap::new(),
            pheromone_deposit_rate,
            pheromone_evaporation_rate,
            alpha,
            beta,
            elitism_factor,
            surgeons_ids,
            surgeries_bin,
            max_days_waiting,
            priority_penalties,
        }
    }

    pub fn round(&mut self, round_number: u32) -> (f64, Vec<(Week, f64)>, Duration) {
        let now = Instant::now();

        let objective_function_results = (0..self.ants_count)
            .into_par_iter()
            .map(|_| {
                Ant::new(
                    self.rooms_count,
                    self.surgeries_bin.clone(),
                    self.surgeons_ids.clone(),
                    self.max_days_waiting.clone(),
                    self.priority_penalties.clone(),
                )
                .find_solution(
                    self.alpha,
                    self.beta,
                    &self.pheromones,
                    self.pheromone_evaporation_rate,
                    round_number,
                )
            })
            .collect::<Vec<_>>();

        let mut pheromones_by_path = HashMap::<(Surgery, Surgery), f64>::new();
        let (mut best_objective_function, mut best_scheduling, mut best_paths) =
            (f64::INFINITY, Default::default(), Default::default());

        objective_function_results.iter().for_each(|result| {
            if result.0 < best_objective_function {
                best_objective_function = result.0;
                best_scheduling = result.1.clone();
                best_paths = result.2.clone();
            }
        });

        let best_paths_set = best_paths
            .into_iter()
            .collect::<HashSet<(Surgery, Surgery)>>();

        let elitism_factor = self.elitism_factor;

        objective_function_results.into_iter().for_each(|result| {
            for path in result.2 {
                let delta = self.pheromone_deposit_rate / result.0;
                let elitism_delta = if best_paths_set.contains(&path) {
                    elitism_factor * delta
                } else {
                    0.0
                };
                *pheromones_by_path.entry(path).or_default() += delta + elitism_delta;
            }
        });

        let pheromone_evaporation_rate = self.pheromone_evaporation_rate;

        let mut updated_paths = HashSet::new();
        for (path, new_pheromone) in pheromones_by_path {
            let total_pheromone = self
                .pheromones
                .entry(path.clone())
                .or_insert((1.0 - self.pheromone_evaporation_rate).powf((round_number - 1) as f64));

            *total_pheromone =
                *total_pheromone * (1.0 - pheromone_evaporation_rate) + new_pheromone;
            updated_paths.insert(path);
        }

        self.pheromones
            .iter_mut()
            .filter(|entry| !updated_paths.contains(&entry.0))
            .for_each(|entry| {
                *entry.1 *= 1.0 - pheromone_evaporation_rate;
            });

        (best_objective_function, best_scheduling, now.elapsed())
    }
}
