use std::env;

use day1::day1;
use day2::day2;
use day3::day3;

pub mod day1;
pub mod day2;
pub mod day3;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1].parse().unwrap();
    println!("AOC DAY {day}");
    match day {
        1 => tokio::spawn(day1()).await.unwrap(),
        2 => tokio::spawn(day2()).await.unwrap(),
        3 => tokio::spawn(day3()).await.unwrap(),
        _ => panic!("invalid day"),
    }
}
