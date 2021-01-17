mod ant;

use crossbeam::channel::{unbounded, Receiver, Sender};
use std::collections::{HashMap, HashSet};
use std::thread;
use std::time::{Duration, Instant};

use super::surgery::{DaysWaiting, Priority, Surgery};
use crate::solver::surgeon::SurgeonID;
use crate::solver::week::Week;
use ant::{Ant, AntFindSolutionData, AntSolution};
use std::sync::Arc;
use std::thread::JoinHandle;

struct AntManager {
    ant_thread: JoinHandle<()>,
    send_to_ant: Sender<Option<AntFindSolutionData>>,
    receive_ant_response: Receiver<AntSolution>,
}

pub struct AntColony {
    ants_threads: Vec<AntManager>,
    ants_count: usize,
    pheromones: HashMap<(Surgery, Surgery), f64>,
    pheromone_deposit_rate: f64,
    pheromone_evaporation_rate: f64,
    elitism_factor: f64,
}

impl AntColony {
    pub fn new(
        treads_count: usize,
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

        let ants = (0..treads_count)
            .map(|_| {
                let (send_to_ant, receive_in_ant) = unbounded();
                let (send_ant_response, receive_ant_response) = unbounded();
                let surgeries_bin = surgeries_bin.clone();
                let surgeons_ids = surgeons_ids.clone();
                let max_days_waiting = max_days_waiting.clone();
                let priority_penalties = priority_penalties.clone();

                AntManager {
                    ant_thread: thread::spawn(move || {
                        Ant::new(
                            alpha,
                            beta,
                            pheromone_evaporation_rate,
                            rooms_count,
                            surgeries_bin,
                            surgeons_ids,
                            max_days_waiting,
                            priority_penalties,
                            receive_in_ant,
                            send_ant_response,
                        )
                        .work()
                    }),
                    send_to_ant,
                    receive_ant_response,
                }
            })
            .collect::<Vec<AntManager>>();

        Self {
            ants_threads: ants,
            ants_count,
            pheromones: HashMap::new(),
            pheromone_deposit_rate,
            pheromone_evaporation_rate,
            elitism_factor,
        }
    }

    pub fn round(&mut self, round_number: u32) -> (f64, Vec<(Week, f64)>, Duration) {
        let now = Instant::now();

        let mut pheromones = HashMap::new();
        std::mem::swap(&mut pheromones, &mut self.pheromones);

        let pheromones = Arc::new(pheromones);

        self.ants_threads
            .iter()
            .cycle()
            .take(self.ants_count)
            .for_each(|ant_manager| {
                ant_manager
                    .send_to_ant
                    .send(Some(AntFindSolutionData {
                        pheromones: Arc::downgrade(&pheromones),
                        round_number,
                    }))
                    .expect("Failed to sent data to ant");
            });

        let responses = self
            .ants_threads
            .iter()
            .cycle()
            .take(self.ants_count)
            .map(|ant_manager| {
                ant_manager
                    .receive_ant_response
                    .recv()
                    .expect("Failed to receive ant response")
            })
            .collect::<Vec<_>>();

        self.pheromones = Arc::try_unwrap(pheromones).unwrap();

        let mut pheromones_by_path = HashMap::<(Surgery, Surgery), f64>::new();
        let mut best_objective_function = f64::INFINITY;
        let mut best_index = 0;

        responses.iter().enumerate().for_each(|(index, result)| {
            if result.objective_function_result < best_objective_function {
                best_objective_function = result.objective_function_result;
                best_index = index;
            }
        });

        let best_scheduling = responses[best_index].all_weeks_results.clone();

        let best_paths_set = responses[best_index]
            .followed_path
            .iter()
            .cloned()
            .collect::<HashSet<(Surgery, Surgery)>>();

        let elitism_factor = self.elitism_factor;

        responses.into_iter().for_each(|result| {
            for path in result.followed_path {
                let delta = self.pheromone_deposit_rate / result.objective_function_result;
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
            let total_pheromone = self.pheromones.entry(path.clone()).or_insert_with(|| {
                (1.0 - pheromone_evaporation_rate).powf((round_number - 1) as f64)
            });

            *total_pheromone =
                *total_pheromone * (1.0 - pheromone_evaporation_rate) + new_pheromone;
            updated_paths.insert(path);
        }

        self.pheromones
            .iter_mut()
            .filter(|(key, _value)| !updated_paths.contains(key))
            .for_each(|(_key, value)| {
                *value *= 1.0 - pheromone_evaporation_rate;
            });

        (best_objective_function, best_scheduling, now.elapsed())
    }

    pub fn kill_ants(&mut self) {
        let mut ants_to_kill = Vec::new();
        std::mem::swap(&mut ants_to_kill, &mut self.ants_threads);

        ants_to_kill.into_iter().for_each(|ant_manager| {
            ant_manager.send_to_ant.send(None).unwrap();
            ant_manager.ant_thread.join().unwrap();
        })
    }
}
