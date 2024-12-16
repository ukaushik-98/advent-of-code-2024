use std::collections::{HashMap, HashSet};

use tokio::fs;

pub async fn day10() {
    // let _ = part_1().await;
    let _ = part_2().await;
}

async fn part_1() {
    let buf = fs::read("./src/day10/real.txt").await.unwrap();
    let content = String::from_utf8_lossy(&buf);
    let content = content.split("\n").collect::<Vec<&str>>();
    let mut matrix = Vec::new();
    for c in content.iter() {
        matrix.push(c.chars().collect::<Vec<char>>());
    }

    let mut head = Vec::new();
    for (i, row) in content.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c == '0' {
                head.push((i, j, c));
            }
        }
    }

    let directions: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];
    let mut count = HashMap::new();

    for (i, j, c) in head.iter() {
        let mut seen = HashSet::new();
        let mut stack = vec![(i.clone(), j.clone(), c.clone())];

        while let Some((i, j, c)) = stack.pop() {
            if seen.contains(&(i.clone(), j.clone())) {
                continue;
            }
            seen.insert((i.clone(), j.clone()));

            if c == '9' {
                *count.entry((i, j)).or_insert(0) += 1;
            }

            let c = c.to_digit(10).unwrap();
            for (x, y) in directions.iter() {
                let m = i as i32 + x;
                let n = j as i32 + y;
                if 0 <= m && m < matrix.len() as i32 && 0 <= n && n < matrix[0].len() as i32 {
                    let r = matrix[m as usize][n as usize];
                    if r == '.' {
                        continue;
                    }
                    let r2 = r.to_digit(10).unwrap();
                    if c + 1 == r2 {
                        stack.push((m as usize, n as usize, r));
                    }
                }
            }
        }
    }

    println!("count: {:?}", count);

    let mut score = 0;
    for c in count.values() {
        score += c;
    }

    println!("AOC Day 10 - Part 1: {:?}", score);
}

async fn part_2() {
    let buf = fs::read("./src/day10/real.txt").await.unwrap();
    let content = String::from_utf8_lossy(&buf);
    let content = content.split("\n").collect::<Vec<&str>>();
    let mut matrix = Vec::new();
    for c in content.iter() {
        matrix.push(c.chars().collect::<Vec<char>>());
    }

    let mut head = Vec::new();
    for (i, row) in content.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c == '0' {
                head.push((i, j, c));
            }
        }
    }

    let directions: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];
    let mut count = HashMap::new();

    for (i, j, c) in head.iter() {
        // let mut seen = HashSet::new();
        let mut stack = vec![(i.clone(), j.clone(), c.clone())];

        while let Some((i, j, c)) = stack.pop() {
            // if seen.contains(&(i.clone(), j.clone())) {
            //     continue;
            // }
            // seen.insert((i.clone(), j.clone()));

            if c == '9' {
                *count.entry((i, j)).or_insert(0) += 1;
            }

            let c = c.to_digit(10).unwrap();
            for (x, y) in directions.iter() {
                let m = i as i32 + x;
                let n = j as i32 + y;
                if 0 <= m && m < matrix.len() as i32 && 0 <= n && n < matrix[0].len() as i32 {
                    let r = matrix[m as usize][n as usize];
                    if r == '.' {
                        continue;
                    }
                    let r2 = r.to_digit(10).unwrap();
                    if c + 1 == r2 {
                        stack.push((m as usize, n as usize, r));
                    }
                }
            }
        }
    }

    println!("count: {:?}", count);

    let mut score = 0;
    for c in count.values() {
        score += c;
    }

    println!("AOC Day 10 - Part 1: {:?}", score);
}
