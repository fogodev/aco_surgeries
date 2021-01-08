mod ant;

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::time::{Duration, SystemTime};

use super::surgery::{DaysWaiting, Priority, Surgery};
use crate::solver::surgeon::SurgeonID;
use ant::Ant;

enum Status {
    Finished,
    Unfinished,
}

pub struct AntColony {
    ants_count: usize,
    rooms_count: usize,
    pheromones: HashMap<(Surgery, Surgery), f64>,
    alpha: f64,
    beta: f64,
    surgeons_ids: Vec<SurgeonID>,
    surgeries_bin: HashSet<Surgery>,
    max_days_waiting: Rc<HashMap<Priority, DaysWaiting>>,
    priority_penalties: Rc<HashMap<Priority, u32>>,
}

impl AntColony {
    pub fn new(
        ants_count: usize,
        rooms_count: usize,
        alpha: f64,
        beta: f64,
        surgeons_ids: Vec<SurgeonID>,
        surgeries_bin: HashSet<Surgery>,
        max_days_waiting: HashMap<Priority, DaysWaiting>,
        priority_penalties: HashMap<Priority, u32>,
    ) -> Self {
        let max_days_waiting = Rc::new(max_days_waiting);
        let priority_penalties = Rc::new(priority_penalties);

        Self {
            ants_count,
            rooms_count,
            pheromones: HashMap::new(),
            alpha,
            beta,
            surgeons_ids,
            surgeries_bin,
            max_days_waiting,
            priority_penalties,
        }
    }

    pub fn round(&mut self, round_number: u32) -> (f64, Duration) {
        let now = SystemTime::now();

        let mut ants = (0..self.ants_count)
            .map(|_| {
                Ant::new(
                    self.rooms_count,
                    self.surgeries_bin.clone(),
                    &self.surgeons_ids,
                    self.max_days_waiting.clone(),
                    self.priority_penalties.clone(),
                )
            })
            .collect::<Vec<_>>();

        loop {
            if matches!(self.step(round_number, &mut ants), Status::Finished) {
                break;
            }
        }

        // ToDo Calcular FO de cada formiga
        // ToDo Atualizar feromônios seguindo a melhor FO
        // ToDo Retornar melhor FO

        (0.0, now.elapsed().unwrap())
    }

    fn step(&mut self, round_number: u32, ants: &mut Vec<Ant>) -> Status {
        let unfinished_ants = ants
            .iter_mut()
            .filter(|ant| !ant.has_unallocated_surgeries())
            .collect::<Vec<_>>();

        if unfinished_ants.is_empty() {
            Status::Finished
        } else {
            unfinished_ants.into_iter().for_each(|ant| {
                ant.choose_next_surgery(self.alpha, self.beta, &self.pheromones, round_number)
            });

            Status::Unfinished
        }
    }
}
