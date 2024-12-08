use std::collections::HashMap;
use std::iter::zip;
use std::usize;

use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn day1() {
    let mut locations = read_file().await.unwrap();
    let distance = calculate_dist(&mut locations).await.unwrap();
    println!("Distance: {}", distance);
    let sim_score = calculate_sim_score(&mut locations).await.unwrap();
    println!("Sim Score: {}", sim_score)
}

async fn read_file() -> Result<(Vec<usize>, Vec<usize>), Box<dyn std::error::Error + 'static>> {
    let mut f = File::open("./src/day1/real.txt").await?;
    let mut buf = Vec::new();

    let _ = f.read_to_end(&mut buf).await;

    let content: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buf);
    let mut content: Vec<&str> = content.split("\n").collect();
    let mut loc1 = Vec::new();
    let mut loc2 = Vec::new();
    for raw_pair in content.iter_mut() {
        let mut raw_pair = raw_pair.split_whitespace();
        loc1.push(raw_pair.next().unwrap().parse::<usize>().unwrap());
        loc2.push(raw_pair.next().unwrap().parse::<usize>().unwrap());
    }

    Ok((loc1, loc2))
}

async fn calculate_dist(
    locations: &mut (Vec<usize>, Vec<usize>),
) -> Result<usize, Box<dyn std::error::Error + 'static>> {
    let (loc1, loc2) = locations;
    loc1.sort();
    loc2.sort();

    let mut total = 0;
    for (x, y) in zip(loc1, loc2) {
        total += (*x).abs_diff(*y);
    }
    Ok(total)
}

async fn calculate_sim_score(
    locations: &mut (Vec<usize>, Vec<usize>),
) -> Result<usize, Box<dyn std::error::Error + 'static>> {
    let (loc1, loc2) = locations;
    let mut loc2_freq_map: HashMap<usize, usize> = HashMap::new();
    for x in loc2.iter_mut() {
        loc2_freq_map.insert(*x, loc2_freq_map.get(x).unwrap_or(&0) + 1);
    }
    let mut total = 0;
    for x in loc1 {
        total += *x * loc2_freq_map.get(x).unwrap_or(&0);
    }
    Ok(total)
}
