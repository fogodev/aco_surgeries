mod solver;

use crate::solver::week::Week;
use solver::surgery::{DaysWaiting, Priority};
use solver::Solver;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use clap::{Arg, App};

const INSTANCE_NAME: &str = "./sample_data/Indefinidas - i1.csv";
const ROOMS_COUNT: usize = 2;

fn main() {
    let inputs = App::new("Ant colony optimization for surgery scheduling.")
        .version("0.0.1")
        .author("Ericson Soares")
        .about("Solves surgery scheduling problem using ant colony optimization algorithm.")
        .arg(Arg::with_name("instance_file")
                 .short("f")
                 .long("file")
                 .takes_value(true)
                 .help("An instance csv file."))
        .arg(Arg::with_name("elitism_factor")
                 .short("el")
                 .long("elitism")
                 .takes_value(true)
                 .help("Elitism factor on pheromones, change to 0 to not use it."))
        .arg(Arg::with_name("deposit")
                 .short("d")
                 .long("deposit")
                 .takes_value(true)
                 .help("Pheromones deposit rate."))
        .arg(Arg::with_name("evaporation")
                 .short("ev")
                 .long("evaporation")
                 .takes_value(true)
                 .help("Pheromones evaporation rate."))
        .arg(Arg::with_name("rooms")
                 .short("r")
                 .long("rooms")
                 .takes_value(true)
                 .help("Number of surgery rooms."))
        .arg(Arg::with_name("alpha")
                 .short("a")
                 .long("alpha")
                 .takes_value(true)
                 .help("Alpha parameter to control pheromones intensity."))
        .arg(Arg::with_name("beta")
                 .short("b")
                 .long("beta")
                 .takes_value(true)
                 .help("Beta parameter to control heuristic intensity."))
        .arg(Arg::with_name("max_rounds")
                 .short("max_r")
                 .long("max_rounds")
                 .takes_value(true)
                 .help("Max number of rounds to execute the algorithm."))
        .arg(Arg::with_name("max_rounds_improv")
                 .short("max_r_imp")
                 .long("max_rounds_improv")
                 .takes_value(true)
                 .help("Max number of rounds without improvement to execute the algorithm."))
        .arg(Arg::with_name("in_parallel")
                 .short("p")
                 .long("in_parallel")
                 .takes_value(true)
                 .help("Choose true or false to execute it in parallel."))
        .get_matches();

    let instance_file = inputs.value_of("instance_file").unwrap_or("./sample_data/Indefinidas - i1.csv");
    let in_parallel: bool = inputs.value_of("in_parallel").unwrap_or(false);
    let max_rounds = inputs.value_of("max_rounds").unwrap_or(1000);
    let max_rounds_improv = inputs.value_of("max_rounds").unwrap_or(1000);
    let elitism_factor = inputs.value_of("elitism_factor").unwrap_or(1.0);
    let deposit = inputs.value_of("deposit").unwrap_or(10000.0);
    let evaporation = inputs.values_of("evaporation").unwrap_or(0.2);
    let rooms = inputs.values_of("rooms").unwrap_or(1);
    let alpha = inputs.values_of("alpha").unwrap_or(1.0);
    let beta = inputs.values_of("beta").unwrap_or(1.0);

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
            alpha,
            beta,
            elitism_factor,
            deposit,
            evaporation,
            max_rounds,
            max_rounds_improv,
            in_parallel,
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
