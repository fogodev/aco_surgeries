mod solver;

use solver::surgery::{DaysWaiting, Priority};
use solver::Solver;
use std::collections::HashMap;
use std::time::Duration;

fn main() {
    let max_days_waiting = [(1, 3), (2, 15), (3, 60), (4, 365)]
        .iter()
        .cloned()
        .collect::<HashMap<Priority, DaysWaiting>>();
    let priority_penalties = [(1, 90), (2, 20), (3, 5), (4, 1)]
        .iter()
        .cloned()
        .collect::<HashMap<Priority, DaysWaiting>>();

    let mut total_duration = Duration::new(0, 0);
    for _ in 0..5 {
        let (result, round, elapsed_time) = Solver::solve(
            // "./sample_data/10_inst.csv",
            "./sample_data/Indefinidas - i8.csv",
            16,
            2,
            max_days_waiting.clone(),
            priority_penalties.clone(),
            1.0,
            1.0,
            1.0,
            10.0,
            0.2,
            1000,
            500,
        );
        total_duration += elapsed_time;
        println!(
            "Best objective function result: {}; Round: {}; Elapsed time: {:#?}",
            result, round, elapsed_time
        );
    }

    println!("Mean Elapsed Time: {:#?}", total_duration / 5)
}
