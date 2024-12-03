use regex::Regex;
use tokio::fs;

pub async fn day3() {
    let _ = read_data().await;
    let _ = read_data_part_2().await;
}

pub async fn read_data() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let buf = fs::read("./src/day3/real.txt").await.unwrap();

    let content: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buf);

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mul_candidates: Vec<(usize, &str)> = re
        .find_iter(&content)
        .map(|mat| (mat.start(), mat.as_str()))
        .collect();

    let mut count = 0;

    for (_, x) in mul_candidates {
        let nums: Vec<usize> = x[4..x.len() - 1]
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        count += nums[0] * nums[1];
    }

    println!("Day 3 Part 1 Count: {count}");
    Ok(())
}

pub async fn read_data_part_2() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let buf = fs::read("./src/day3/real.txt").await.unwrap();

    let content: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buf);

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mul_candidates: Vec<(usize, &str)> = re
        .find_iter(&content)
        .map(|mat| (mat.start(), mat.as_str()))
        .collect();

    let dos: Vec<(usize, &str)> = content.match_indices("do()").collect();
    let donts: Vec<(usize, &str)> = content.match_indices("don't()").collect();

    let mut execs = [dos, donts, mul_candidates].concat();
    execs.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut count = 0;
    let mut state = true;

    for (_, x) in execs.iter() {
        if *x == "do()" {
            state = true;
        } else if *x == "don't()" {
            state = false
        } else {
            count += if state { compute(*x) } else { 0 }
        }
    }

    println!("Day 3 Part 2 Count: {count}");
    Ok(())
}

fn compute(x: &str) -> usize {
    let nums: Vec<usize> = x[4..x.len() - 1]
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    nums[0] * nums[1]
}
