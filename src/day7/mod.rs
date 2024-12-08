use std::collections::VecDeque;

use tokio::fs;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
enum Operation {
    Mult,
    Add,
    Combine,
}

pub async fn day7() {
    part_1().await;
    part_2().await;
}

async fn part_1() {
    let buf = fs::read("./src/day7/real.txt").await.unwrap();

    let content: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buf);
    let mut content: Vec<&str> = content.split("\n").collect();

    let mut result = 0;

    for row in content.iter_mut() {
        let total_and_inputs: Vec<&str> = row.split(": ").collect();
        let total: usize = total_and_inputs[0].parse().unwrap();
        let inputs: Vec<usize> = total_and_inputs[1]
            .split(" ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        // println!("total: {:?}, inputs: {:?}", total, inputs);

        let mut dq = VecDeque::new();
        dq.push_back((inputs[0], 1, Operation::Add));
        dq.push_back((inputs[0], 1, Operation::Mult));

        while let Some((curr_total, index, operation)) = dq.pop_front() {
            // println!(
            //     "total: {}, index: {}, o: {:?}",
            //     curr_total, index, operation
            // );

            if curr_total > total {
                continue;
            }

            // check total
            if index == inputs.len() {
                if curr_total == total {
                    result += curr_total;
                    break;
                } else {
                    continue;
                }
            }

            let new_total = match operation {
                Operation::Mult => curr_total * inputs[index],
                Operation::Add => curr_total + inputs[index],
                _ => 0,
            };

            dq.push_back((new_total, index + 1, Operation::Add));
            dq.push_back((new_total, index + 1, Operation::Mult));
        }
    }

    println!("AOC Day 7 - Part 1: {}", result)
}

async fn part_2() {
    let buf = fs::read("./src/day7/real.txt").await.unwrap();

    let content: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buf);
    let mut content: Vec<&str> = content.split("\n").collect();

    let mut result = 0;

    for row in content.iter_mut() {
        let total_and_inputs: Vec<&str> = row.split(": ").collect();
        let total: usize = total_and_inputs[0].parse().unwrap();
        let inputs: Vec<usize> = total_and_inputs[1]
            .split(" ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let mut dq = VecDeque::new();
        dq.push_back((inputs[0], 1, Operation::Add));
        dq.push_back((inputs[0], 1, Operation::Mult));
        dq.push_back((inputs[0], 1, Operation::Combine));

        while let Some((curr_total, index, operation)) = dq.pop_front() {
            // println!(
            //     "total: {}, index: {}, o: {:?}",
            //     curr_total, index, operation
            // );

            if curr_total > total {
                continue;
            }

            // check total
            if index == inputs.len() {
                if curr_total == total {
                    result += curr_total;
                    break;
                } else {
                    continue;
                }
            }

            let new_total = match operation {
                Operation::Mult => curr_total * inputs[index],
                Operation::Add => curr_total + inputs[index],
                Operation::Combine => {
                    let left = curr_total.to_string();
                    let right = inputs[index].to_string();
                    let t = format!("{left}{right}").parse::<usize>().unwrap();
                    t
                }
            };

            dq.push_back((new_total, index + 1, Operation::Add));
            dq.push_back((new_total, index + 1, Operation::Mult));
            dq.push_back((new_total, index + 1, Operation::Combine));
        }
    }

    println!("AOC Day 7 - Part 2: {}", result)
}
