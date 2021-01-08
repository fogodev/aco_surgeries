use std::collections::{HashMap, HashSet};

pub mod ant_colony;
pub mod room;
pub mod surgeon;
pub mod surgery;

use crate::solver::surgery::Speciality;
use ant_colony::AntColony;
use std::fmt::Debug;
use std::path::Path;
use surgeon::SurgeonID;
use surgery::{DaysWaiting, Priority, Surgery};

pub struct Solver {
    ant_colony: AntColony,
}

impl Solver {
    pub fn solve<P: AsRef<Path> + Debug + Copy>(
        instance_filename: P,
        ants_count: usize,
        rooms_count: usize,
        max_days_waiting: HashMap<Priority, DaysWaiting>,
        priority_penalties: HashMap<Priority, u32>,
        max_rounds_count: u32,
        max_rounds_without_improvement: u32,
    ) -> (f64, u32) {
        let (surgeries, surgeons_ids) = Self::load_from_csv(instance_filename);

        let mut solver = Self {
            ant_colony: AntColony::new(
                ants_count,
                rooms_count,
                1.0,
                1.0,
                surgeons_ids,
                surgeries,
                max_days_waiting,
                priority_penalties,
            ),
        };

        let mut best_objective_function_result = 0.0;
        let mut best_objective_function_round = 0;
        for round in 0..max_rounds_count {
            let (objective_function_result, elapsed_time) = solver.ant_colony.round(round);
            println!(
                "Round: {}; Objective Function: {}; Elapsed Time: {:#?}",
                round, objective_function_result, elapsed_time
            );
            if objective_function_result > best_objective_function_result {
                best_objective_function_result = objective_function_result;
                best_objective_function_round = round
            }
            if round - best_objective_function_round > max_rounds_without_improvement {
                break;
            }
        }

        (
            best_objective_function_result,
            best_objective_function_round,
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
            .expect(&format!("Unable to read from {:#?}", &filename))
            .deserialize()
            .enumerate()
        {
            let record: Record =
                row.expect(&format!("Malformed line error at line: {}", line_number));
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
