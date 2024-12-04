use std::{collections::HashSet, usize};

use tokio::fs;

pub async fn day4() {
    let _ = read_file().await;
    let _ = read_file_part_2().await;
}

async fn read_file() {
    let buf = fs::read("./src/day4/real.txt").await.unwrap();
    let content = String::from_utf8_lossy(&buf);
    let mut content: Vec<_> = content.split("\n").collect();

    let mut matrix = Vec::new();
    for row in content.iter() {
        let row: Vec<char> = row.chars().collect();
        matrix.push(row);
    }

    let mut count = 0;
    let direction = vec![
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    for (i, row) in matrix.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            for (x, y) in direction.iter() {
                let result = dfs(&matrix, &(i, j), (*x, *y));
                count += result;
            }
        }
    }

    println!("Day 4 Part 1 Count: {}", count);
}

fn dfs(matrix: &Vec<Vec<char>>, (i, j): &(usize, usize), (x, y): (i32, i32)) -> usize {
    let pattern: Vec<char> = "XMAS".chars().collect();
    let mut count = 0;
    let mut stack = vec![(*i as i32, *j as i32, 0)];
    let mut seen = HashSet::new();

    while let Some((m, n, a)) = stack.pop() {
        if seen.contains(&(m.clone(), n.clone())) {
            continue;
        }
        seen.insert((m, n));

        if matrix[m as usize][n as usize] == pattern[a] {
            if a == pattern.len() - 1 {
                count += 1;
                break;
            }
            let m = m + x;
            let n = n + y;
            if 0 <= m && m < matrix.len() as i32 && 0 <= n && n < matrix[0].len() as i32 {
                stack.push((m, n, a + 1))
            }
        }
    }
    count
}

async fn read_file_part_2() {
    let buf = fs::read("./src/day4/real.txt").await.unwrap();
    let content = String::from_utf8_lossy(&buf);
    let content: Vec<_> = content.split("\n").collect();

    let mut matrix = Vec::new();
    for row in content.iter() {
        let row: Vec<char> = row.chars().collect();
        matrix.push(row);
    }

    let mut count = 0;

    for (i, row) in matrix.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if matrix[i as usize][j as usize] == 'A' {
                let result = dfs_part_2(&matrix, &(i, j));
                count += result;
            }
        }
    }

    println!("Day 4 Part 2 Count: {}", count);
}

fn dfs_part_2(matrix: &Vec<Vec<char>>, (i, j): &(usize, usize)) -> usize {
    let directions: Vec<(i32, i32)> = vec![(-1, 1), (1, 1), (-1, -1), (1, -1)];

    let mut diags = Vec::new();
    for (x, y) in directions.iter() {
        let m = *i as i32 + x;
        let n = *j as i32 + y;
        if 0 <= m && m < matrix.len() as i32 && 0 <= n && n < matrix[0].len() as i32 {
            diags.push(matrix[m as usize][n as usize]);
        }
    }

    let res: Vec<[char; 4]> = vec![
        ['S', 'S', 'M', 'M'],
        ['M', 'M', 'S', 'S'],
        ['M', 'S', 'M', 'S'],
        ['S', 'M', 'S', 'M'],
    ];

    // Check if any of the patterns in res are in diags
    for pattern in res.iter() {
        if diags == pattern {
            return 1;
        }
    }
    0
}
