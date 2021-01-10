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

    let cpus = num_cpus::get_physical();

    println!("Running with {} ants in parallel;", cpus);

    let (mut results, mut durations) = (Vec::with_capacity(5), Vec::with_capacity(5));
    for _ in 0..5 {
        let (result, round, elapsed_time) = Solver::solve(
            "./sample_data/Indefinidas - i1.csv",
            cpus,
            3,
            max_days_waiting.clone(),
            priority_penalties.clone(),
            2.0,
            1.0,
            1.0,
            10000.0,
            0.2,
            1000,
            500,
            false,
        );
        println!(
            "Best objective function result: {}; Round: {}; Elapsed time: {:#?}",
            result, round, elapsed_time
        );
        results.push(result);
        durations.push(elapsed_time)
    }
    results.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let minimum_result = results[0];
    let median_result = results[2];
    let maximum_result = results[4];

    let results_mean = results.iter().sum::<f64>() / 5.0;
    let durations_mean = durations.iter().sum::<Duration>() / 5;

    println!(
        "Minimum Result: {}; Maximum Result: {}; Median: {};\nMean Objective Function: {} ± {}; Mean Elapsed Time: {:#?} ± {:#?}s;",
        minimum_result,
        maximum_result,
        median_result,
        results_mean,
        (results
            .iter()
            .fold(0.0, |sum, &value| sum + (value - results_mean).powi(2))
            / 4.0)
            .sqrt(),
        durations_mean,
        (durations
            .iter()
            .map(|duration| duration.as_secs_f64())
            .fold(0.0, |sum, value| sum
                + (value - durations_mean.as_secs_f64()).powi(2))
            / 4.0)
            .sqrt()
    )
}
