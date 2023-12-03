mod day01;
mod day02;
// Add more modules as needed

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        for day in 1..=25 {
            run_day(day);
        }
    } else {
        if let Ok(day) = args[1].parse::<u32>() {
            run_day(day);
        } else {
            eprintln!("Invalid day: {}", args[1]);
            std::process::exit(1);
        }
    }
}

fn run_day(day: u32) {
    println!("Day {}: {}", day, name_of_day(day));

    match day {
        1 => day01::run(),
        2 => day02::run(),
        // Add more days as needed
        _ => eprintln!("Day {} not implemented", day),
    }
}

fn name_of_day(day: u32) -> &'static str {
    match day {
        1 => day01::name(),
        2 => day02::name(),
        // Add more days as needed
        _ => "Unknown",
    }
}


