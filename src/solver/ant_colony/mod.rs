mod ant;

use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, SystemTime};

use super::surgery::{DaysWaiting, Priority, Surgery};
use crate::solver::surgeon::SurgeonID;
use crate::solver::week::Week;
use ant::Ant;
use std::sync::Arc;

pub struct AntColony {
    ants_count: usize,
    rooms_count: usize,
    pheromones: HashMap<(Surgery, Surgery), f64>,
    pheromone_evaporation_rate: f64,
    alpha: f64,
    beta: f64,
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
            pheromone_evaporation_rate,
            alpha,
            beta,
            surgeons_ids,
            surgeries_bin,
            max_days_waiting,
            priority_penalties,
        }
    }

    pub fn round(&mut self, round_number: u32) -> (f64, Vec<(Week, f64)>, Duration) {
        let now = SystemTime::now();

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

        // println!("{:#?}", objective_function_results);

        let (mut best_objective_function, mut best_ant) = (f64::INFINITY, Default::default());
        objective_function_results.into_iter().for_each(|result| {
            if result.0 < best_objective_function {
                best_objective_function = result.0;
                best_ant = result.1;
            }
        });

        // ToDo Atualizar feromônios seguindo a melhor FO
        // ToDo Retornar melhor FO

        (best_objective_function, best_ant, now.elapsed().unwrap())
    }
}
