use std::{
    collections::{HashMap, HashSet},
    usize,
};

use tokio::{fs, join};

pub async fn day5() {
    let (map, updates) = join!(build_map(), build_updates());

    let mut count = 0;
    let mut count2 = 0;
    for update in updates.iter() {
        if check(&map, &update) {
            let mid = update.len() / 2;
            count += update[mid].parse::<usize>().unwrap();
        } else {
            let update = fix(&map, &update);
            // let mid = update.len() / 2;
            // count2 += update[mid].parse::<usize>().unwrap();
        }
    }
    println!("AOC DAY 5 Part 1: {count}");
    println!("AOC DAY 5 Part 1: {count2}");
}

async fn build_map() -> HashMap<String, HashSet<String>> {
    let buf = fs::read("./src/day5/sample_map.txt").await.unwrap();

    let content: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buf);
    let mut content: Vec<&str> = content.split("\n").collect();

    let mut rules: HashMap<String, HashSet<String>> = HashMap::new();

    for rule in content.iter_mut() {
        let pages: Vec<&str> = rule.split("|").collect();
        let deps = rules.entry(pages[1].to_string()).or_insert(HashSet::new());
        deps.insert(pages[0].to_string());
    }

    rules
}

async fn build_updates() -> Vec<Vec<String>> {
    let buf = fs::read("./src/day5/sample_updates.txt").await.unwrap();

    let content: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buf);
    let mut content: Vec<&str> = content.split("\n").collect();

    let mut updates = Vec::new();

    for update in content.iter_mut() {
        let pages: Vec<&str> = update.split(",").collect();
        let pages = pages.iter().map(|x| x.to_string()).collect();
        updates.push(pages);
    }

    updates
}

fn check(map: &HashMap<String, HashSet<String>>, updates: &Vec<String>) -> bool {
    for (i, u) in updates.iter().enumerate() {
        let deps = map.get(u);
        match deps {
            None => continue,
            Some(deps) => {
                for d in deps.iter() {
                    let p = updates.iter().position(|x| *x == d.to_string());
                    match p {
                        Some(p) => {
                            if p > i {
                                return false;
                            }
                        }
                        None => continue,
                    }
                }
            }
        }
    }
    true
}

fn fix(map: &HashMap<String, HashSet<String>>, updates: &Vec<String>) -> Vec<String> {
    println!("updates: {:?}", updates);
    Vec::new()
}
