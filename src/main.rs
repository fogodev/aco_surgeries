mod solver;

use crate::solver::week::Week;
use solver::surgery::{DaysWaiting, Priority};
use solver::Solver;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Ant Colony Optimization for surgery scheduling",
    about = "An ACO implementation to solve a surgery scheduling problem."
)]
struct Opt {
    /// An instance csv file.
    #[structopt(
        short = "f",
        long = "file",
        default_value = "./sample_data/Indefinidas - i1.csv"
    )]
    instance_file: String,

    /// Elitism factor on pheromones, change to 0 to not use it.
    #[structopt(short = "el", long = "elitism", default_value = "1.0")]
    elitism_factor: f64,

    /// Ants count, default = 8.
    #[structopt(short = "a", long = "ants_count", default_value = "8")]
    ants_count: usize,

    /// Pheromones deposit rate.
    #[structopt(short = "d", long = "deposit", default_value = "10000.0")]
    deposit: f64,

    /// Pheromones evaporation rate.
    #[structopt(long = "evaporation", default_value = "0.2")]
    evaporation: f64,

    /// Number of surgery rooms.
    #[structopt(short = "r", long = "rooms", default_value = "1")]
    rooms: usize,

    /// Alpha parameter to control pheromones intensity.
    #[structopt(short = "a", long = "alpha", default_value = "1.0")]
    alpha: f64,

    /// Beta parameter to control heuristic intensity.
    #[structopt(short = "b", long = "beta", default_value = "1.0")]
    beta: f64,

    /// Max number of rounds to run.
    #[structopt(long = "max_rounds", default_value = "1000")]
    max_rounds: u32,

    /// Max number of rounds to run without improvement.
    #[structopt(long = "max_rounds_improv", default_value = "500")]
    max_rounds_improv: u32,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let instance_file = &opt.instance_file;
    let max_rounds = opt.max_rounds;
    let max_rounds_improv = opt.max_rounds_improv;
    let elitism_factor = opt.elitism_factor;
    let deposit = opt.deposit;
    let evaporation = opt.evaporation;
    let rooms = opt.rooms;
    let alpha = opt.alpha;
    let beta = opt.beta;
    let ants_count = opt.ants_count;

    let max_days_waiting = [(1, 3), (2, 15), (3, 60), (4, 365)]
        .iter()
        .cloned()
        .collect::<HashMap<Priority, DaysWaiting>>();
    let priority_penalties = [(1, 90), (2, 20), (3, 5), (4, 1)]
        .iter()
        .cloned()
        .collect::<HashMap<Priority, DaysWaiting>>();

    println!("Running with {} ants", ants_count);
    let mut best_result = f64::INFINITY;
    let mut best_scheduling = Vec::new();

    let (mut results, mut durations) = (Vec::with_capacity(5), Vec::with_capacity(5));
    for _ in 0..5 {
        let (result, round, schedule, elapsed_time) = Solver::solve(
            instance_file,
            ants_count,
            rooms,
            max_days_waiting.clone(),
            priority_penalties.clone(),
            alpha,
            beta,
            elitism_factor,
            deposit,
            evaporation,
            max_rounds,
            max_rounds_improv,
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
        "Minimum Result: {}; Median: {}; Maximum Result: {};\nMean Objective Function: {} ± {}; Mean Elapsed Time: {:#?} ± {:#?}s;",
        minimum_result,
        median_result,
        maximum_result,
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

    save_durations(instance_file, durations, ants_count);

    schedule_to_csv(instance_file, best_scheduling);
}

fn save_durations(instance_name: &str, durations: Vec<Duration>, ants_count: usize) {
    let name = instance_name.split(".csv").next().unwrap();
    let solution_name = format!("{}_durations_{}_ants.txt", name, ants_count);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(solution_name)
        .expect("Unable to create txt file");
    durations.into_iter().for_each(|duration| {
        write!(file, "{:#?}\n", duration.as_secs_f64()).expect("Failed to write duration");
    })
}

fn schedule_to_csv(instance_name: &str, schedule: Vec<(Week, f64)>) {
    let name = instance_name.split(".csv").next().unwrap();
    let solution_name = format!("{}_sol.csv", name);
    let mut file = File::create(solution_name).expect("Unable to create csv file");
    write!(file, "Cirurgia (c);Sala (r);Dia (d);Horário (t)\n").expect("Failed to write header");

    let mut results = Vec::new();

    for (week_index, (week, _)) in schedule.into_iter().enumerate() {
        for (day_index, day) in week.days().iter().enumerate() {
            for (room_index, room) in day.rooms().iter().enumerate() {
                for (surgery, (schedule, _)) in
                    room.surgeries().iter().zip(room.scheduled_surgeons())
                {
                    results.push((
                        surgery.id,
                        room_index + 1,
                        (day_index + 1) * (week_index + 1),
                        schedule.start,
                    ));
                }
            }
        }
    }

    results.sort_by(|element1, element2| element1.0.cmp(&element2.0));
    for row in results {
        write!(file, "{};{};{};{}\n", row.0, row.1, row.2, row.3,).expect("Failed to write row");
    }
}
