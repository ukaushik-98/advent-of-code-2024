use std::env;

use day1::{calculate_dist, calculate_sim_score, read_file};

pub mod day1;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1].parse().unwrap();
    println!("AOC DAY {day}");
    match day {
        1 => tokio::spawn(day1()).await.unwrap(),
        _ => panic!("invalid day"),
    }
}

async fn day1() {
    let mut locations = read_file().await.unwrap();
    let distance = calculate_dist(&mut locations).await.unwrap();
    println!("Distance: {}", distance);
    let sim_score = calculate_sim_score(&mut locations).await.unwrap();
    println!("Sim Score: {}", sim_score)
}
