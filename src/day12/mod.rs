use std::collections::{HashMap, HashSet};

pub async fn day12() {
    let _ = part_1().await;
}

async fn part_1() {
    let content = tokio::fs::read("./src/day12/sample_3.txt").await.unwrap();
    let content = String::from_utf8_lossy(&content);
    let content = content.split("\n").collect::<Vec<&str>>();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut zones: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let directions: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];

    for (i, row) in content.iter().enumerate() {
        for (j, r) in row.chars().enumerate() {
            let str_r = String::from(r);
            if !zones.contains_key(&str_r as &str) {
                stack.push((i, j));
                while let Some((x, y)) = stack.pop() {
                    if seen.contains(&(x, y)) {
                        continue;
                    }
                    println!("x: {} y: {}", x, y);
                    let test = str_r.clone();
                    zones
                        .entry(test)
                        .or_insert(Vec::new())
                        .push((x.clone(), y.clone()));

                    seen.insert((x.clone(), y.clone()));

                    let x = x as i32;
                    let y = y as i32;

                    for (mx, my) in directions.iter() {
                        let x = x + mx;
                        let y = y + my;

                        if 0 <= x
                            && x < content.len() as i32
                            && 0 <= y
                            && y < content[0].len() as i32
                        {
                            let x = x as usize;
                            let y = y as usize;
                            let new_r = content[x].chars().nth(y).unwrap();

                            if new_r == r {
                                stack.push((x, y));
                            }
                        }
                    }
                }
            }
        }
    }
    println!("final map: {:?}", zones);
}
