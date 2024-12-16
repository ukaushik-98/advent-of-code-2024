use std::collections::HashSet;

use tokio::fs;

pub async fn day8() {
    // let _ = part_1().await;
    let _ = part_2().await;
}

async fn part_1() {
    let buf = fs::read("./src/day8/real.txt").await.unwrap();
    let content = String::from_utf8_lossy(&buf);
    let mut content: Vec<&str> = content.split("\n").collect();

    let mut nodes = Vec::new();
    for (i, cont) in content.iter_mut().enumerate() {
        for (j, c) in cont.char_indices() {
            if c != '.' {
                nodes.push((i, j, c))
            }
        }
    }

    let mut anti_node = HashSet::new();

    for (i, j, c) in nodes.iter() {
        for (x, y, z) in nodes.iter() {
            if c == z && !(i == x && j == y) {
                let dx = *i as i32 - *x as i32;
                let dy = *j as i32 - *y as i32;

                let i = *i as i32 + dx;
                let j = *j as i32 + dy;

                if (0 <= i && i < content.len() as i32) && (0 <= j && j < content[0].len() as i32) {
                    anti_node.insert((i, j));
                }
            }
        }
    }

    println!("AOC Day 8 - Part 1: {}", anti_node.len());
}

async fn part_2() {
    let buf = fs::read("./src/day8/real.txt").await.unwrap();
    let content = String::from_utf8_lossy(&buf);
    let mut content: Vec<&str> = content.split("\n").collect();

    let mut nodes = Vec::new();
    for (i, cont) in content.iter_mut().enumerate() {
        for (j, c) in cont.char_indices() {
            if c != '.' {
                nodes.push((i, j, c));
            }
        }
    }

    let mut anti_node = HashSet::new();
    for (i, j, c) in nodes.iter() {
        for (x, y, z) in nodes.iter() {
            let mut i = *i as i32;
            let mut j = *j as i32;
            anti_node.insert((i, j));

            let x = *x as i32;
            let y = *y as i32;

            if c == z && !(i == x && j == y) {
                let dx = i - x;
                let dy = j - y;

                loop {
                    i += dx;
                    j += dy;

                    if (0 <= i && i < content.len() as i32)
                        && (0 <= j && j < content[0].len() as i32)
                    {
                        anti_node.insert((i, j));
                    } else {
                        break;
                    }
                }
            }
        }
    }

    println!("AOC Day 8 - Part 2: {}", anti_node.len());
}
