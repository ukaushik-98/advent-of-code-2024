use tokio::fs;

pub async fn day9() {
    // let _ = part_1().await;
    let _ = part_2().await;
}

async fn part_1() {
    let buf = fs::read("./src/day9/real.txt").await.unwrap();
    let content = String::from_utf8_lossy(&buf);
    let mut res = Vec::new();
    let mut count = 0;

    let mut d = -1;
    for (i, c) in content.chars().enumerate() {
        let x = if i % 2 == 0 {
            d += 1;
            count += c.to_digit(10).unwrap();
            d
        } else {
            -1
        };

        let c = c.to_digit(10).unwrap();
        for _ in 0..c {
            res.push(x);
        }
    }

    let mut res2 = res.clone();
    let mut disk: Vec<i32> = Vec::new();
    for r in res {
        if count == disk.len() as u32 {
            break;
        }
        let x = if r == -1 {
            let mut a = r;
            while let Some(b) = res2.pop() {
                if b != -1 {
                    a = b;
                    break;
                }
            }
            a
        } else {
            r
        };
        disk.push(x);
    }

    let mut result: usize = 0;
    for (i, d) in disk.iter().enumerate() {
        result += i * (*d as usize);
    }

    println!("AOC Day 9 - Part 1: {}", result);
}

async fn part_2() {
    let buf = fs::read("./src/day9/real.txt").await.unwrap();
    let content = String::from_utf8_lossy(&buf);
    let mut res = Vec::new();
    let mut count = 0;

    let mut d = -1;
    for (i, c) in content.chars().enumerate() {
        let x = if i % 2 == 0 {
            d += 1;
            count += c.to_digit(10).unwrap();
            d
        } else {
            -1
        };

        let c = c.to_digit(10).unwrap();
        for _ in 0..c {
            res.push(x);
        }
    }

    let mut res2 = res.clone();
    let mut disk: Vec<i32> = Vec::new();
    for r in res {
        if count == disk.len() as u32 {
            break;
        }
        let x = if r == -1 {
            let mut a = r;
            while let Some(b) = res2.pop() {
                if b != -1 {
                    a = b;
                    break;
                }
            }
            a
        } else {
            r
        };
        disk.push(x);
    }

    let mut result: usize = 0;
    for (i, d) in disk.iter().enumerate() {
        result += i * (*d as usize);
    }

    println!("AOC Day 9 - Part 2: {}", result);
}
