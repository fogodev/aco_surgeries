mod solver;

use solver::surgery::{DaysWaiting, Priority};
use solver::Solver;
use std::collections::HashMap;

fn main() {
    let max_days_waiting = [(1, 3), (2, 15), (3, 60), (4, 365)]
        .iter()
        .cloned()
        .collect::<HashMap<Priority, DaysWaiting>>();
    let priority_penalties = [(1, 90), (2, 20), (3, 5), (4, 1)]
        .iter()
        .cloned()
        .collect::<HashMap<Priority, DaysWaiting>>();

    Solver::solve(
        "./sample_data/1_inst.csv",
        1,
        1,
        max_days_waiting,
        priority_penalties,
        0.5,
        1000,
        100,
    );
}
