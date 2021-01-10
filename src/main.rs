mod solver;

use crate::solver::week::Week;
use solver::surgery::{DaysWaiting, Priority};
use solver::Solver;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

const INSTANCE_NAME: &str = "./sample_data/Indefinidas - i7.csv";
const ROOMS_COUNT: usize = 3;

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
    let mut best_result = f64::INFINITY;
    let mut best_scheduling = Vec::new();

    let (mut results, mut durations) = (Vec::with_capacity(5), Vec::with_capacity(5));
    for _ in 0..5 {
        let (result, round, schedule, elapsed_time) = Solver::solve(
            INSTANCE_NAME,
            cpus,
            ROOMS_COUNT,
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
        if result < best_result {
            best_result = result;
            best_scheduling = schedule;
        }
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
    );

    schedule_to_csv(INSTANCE_NAME, best_scheduling);
}

fn schedule_to_csv(instance_name: &str, schedule: Vec<(Week, f64)>) {
    let name = instance_name.split(".csv").next().unwrap();
    let solution_name = format!("{}_sol.csv", name);
    let mut file = File::create(solution_name).expect("Unable to create csv file");
    write!(file, "Cirurgia (c),Sala (r),Dia (d),Horário (t)\n").expect("Failed to write header");

    let mut results = Vec::new();

    for (week_index, (week, _)) in schedule.into_iter().enumerate() {
        for (day_index, day) in week.days().iter().enumerate() {
            for (room_index, room) in day.rooms().iter().enumerate() {
                let mut scheduled_time = 1;
                for surgery in room.surgeries() {
                    results.push((
                        surgery.id,
                        room_index + 1,
                        (day_index + 1) * (week_index + 1),
                        scheduled_time,
                    ));
                    scheduled_time += surgery.duration + 2;
                }
            }
        }
    }

    results.sort_by(|element1, element2| element1.0.cmp(&element2.0));
    for row in results {
        write!(file, "{},{},{},{}\n", row.0, row.1, row.2, row.3,).expect("Failed to write row");
    }
}
