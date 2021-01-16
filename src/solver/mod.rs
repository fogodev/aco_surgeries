use std::collections::{HashMap, HashSet};

pub mod ant_colony;
pub mod surgeon;
pub mod surgery;
pub mod week;

use crate::solver::surgery::Speciality;
use crate::solver::week::Week;
use ant_colony::AntColony;
use std::fmt::Debug;
use std::path::Path;
use std::time::{Duration, Instant};
use surgeon::SurgeonID;
use surgery::{DaysWaiting, Priority, Surgery};

pub struct Solver {
    ant_colony: AntColony,
}

impl Solver {
    pub fn solve<P: AsRef<Path> + Debug + Copy>(
        instance_filename: P,
        threads_count: usize,
        ants_count: usize,
        rooms_count: usize,
        max_days_waiting: HashMap<Priority, DaysWaiting>,
        priority_penalties: HashMap<Priority, u32>,
        alpha: f64,
        beta: f64,
        elitism_factor: f64,
        pheromone_deposit_rate: f64,
        pheromone_evaporation_rate: f64,
        max_rounds_count: u32,
        max_rounds_without_improvement: u32,
    ) -> (f64, u32, Vec<(Week, f64)>, Duration) {
        let (surgeries, surgeons_ids) = Self::load_from_csv(instance_filename);

        let mut solver = Self {
            ant_colony: AntColony::new(
                threads_count,
                ants_count,
                rooms_count,
                alpha,
                beta,
                elitism_factor,
                pheromone_deposit_rate,
                pheromone_evaporation_rate,
                surgeons_ids,
                surgeries,
                max_days_waiting,
                priority_penalties,
            ),
        };

        let now = Instant::now();

        let mut best_objective_function_result = f64::INFINITY;
        let mut best_objective_function_round = 0;
        let mut best_scheduling = Vec::new();
        for round in 1..(max_rounds_count + 1) {
            let (objective_function_result, scheduling, elapsed_time) =
                solver.ant_colony.round(round);

            if round % 100 == 0 {
                println!(
                    "Round:\t{:5};\tObjective Function:\t{:15};\tElapsed Time:\t{:#?}",
                    round, objective_function_result, elapsed_time
                );
            }
            if objective_function_result < best_objective_function_result {
                best_objective_function_result = objective_function_result;
                best_objective_function_round = round;
                best_scheduling = scheduling;
            }
            if round - best_objective_function_round > max_rounds_without_improvement {
                break;
            }
        }

        solver.ant_colony.kill_ants();

        (
            best_objective_function_result,
            best_objective_function_round,
            best_scheduling,
            now.elapsed(),
        )
    }

    pub fn load_from_csv<P: AsRef<Path> + Debug + Copy>(
        filename: P,
    ) -> (HashSet<Surgery>, Vec<SurgeonID>) {
        // Cirurgia (c),Prioridade (p),Dias_espera (w),Especialidade (e),Cirurgião (h),Duração (tc)
        type Record = (usize, Priority, DaysWaiting, Speciality, SurgeonID, u8);

        let mut surgeries = HashSet::new();
        let mut surgeons_ids = Vec::new();

        for (line_number, row) in csv::Reader::from_path(filename)
            .unwrap_or_else(|_| panic!("Unable to read from {:#?}", &filename))
            .deserialize()
            .enumerate()
        {
            let record: Record =
                row.unwrap_or_else(|_| panic!("Malformed line error at line: {}", line_number));
            let (id, priority, days_waiting, speciality, surgeon_id, duration) = record;

            surgeries.insert(Surgery::new(
                id,
                duration,
                days_waiting,
                priority,
                speciality,
                surgeon_id,
            ));

            surgeons_ids.push(surgeon_id);
        }

        (surgeries, surgeons_ids)
    }
}
