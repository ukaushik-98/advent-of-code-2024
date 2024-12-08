use std::collections::HashSet;

use tokio::fs;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub async fn day6() {
    part_1().await;
    part_2().await;
}

async fn part_1() {
    let buf = fs::read("./src/day6/sample.txt").await.unwrap();

    let content: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buf);
    let content: Vec<&str> = content.split("\n").collect();

    let mut matrix = Vec::new();
    let mut start: Option<(usize, usize, Directions)> = None;

    for (i, row) in content.iter().enumerate() {
        let row = row.chars().collect::<Vec<char>>();
        let guard = row.iter().position(|x| *x == '^');
        match guard {
            Some(p) => {
                start = Some((i, p, Directions::UP));
            }
            _ => (),
        };
        matrix.push(row);
    }

    let result = traverse(&matrix, start);

    println!("AOC Day 6 - Part 1: {}", result.len() + 1)
}

fn traverse(
    matrix: &Vec<Vec<char>>,
    start: Option<(usize, usize, Directions)>,
) -> HashSet<(usize, usize)> {
    // println!("matrix: ");
    // matrix.iter().for_each(|row| println!("{:?}", row));

    let mut position = start.unwrap();
    let mut result = HashSet::new();

    loop {
        let (x, y, d) = position;
        println!("x: {}, y: {}, d: {:?}", x, y, d);
        let (dx, dy): (i32, i32) = match d {
            Directions::UP => (-1, 0),
            Directions::DOWN => (1, 0),
            Directions::LEFT => (0, -1),
            Directions::RIGHT => (0, 1),
        };
        let ix = x as i32 + dx;
        let iy = y as i32 + dy;

        if !(0 <= ix && ix < matrix.len() as i32 && 0 <= iy && iy < matrix[0].len() as i32) {
            break;
        }
        match matrix.get(ix as usize).unwrap().get(iy as usize) {
            Some(p) => {
                let (x, y, d) = match p {
                    '#' => {
                        let new_d = match d {
                            Directions::UP => Directions::RIGHT,
                            Directions::RIGHT => Directions::DOWN,
                            Directions::DOWN => Directions::LEFT,
                            Directions::LEFT => Directions::UP,
                        };
                        // println!("DIRECTION MOVE OCCURRED: {:?}", new_d);

                        (x, y, new_d)
                    }
                    _ => {
                        result.insert((x as usize, y as usize));
                        (ix as usize, iy as usize, d)
                    }
                };

                position = (x as usize, y as usize, d.clone());
                // println!("result: {:?}", result);
            }
            None => break,
        }
    }
    result
}

async fn part_2() {
    let buf = fs::read("./src/day6/real.txt").await.unwrap();

    let content: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buf);
    let content: Vec<&str> = content.split("\n").collect();

    let mut matrix = Vec::new();
    let mut start: Option<(usize, usize, Directions)> = None;

    for (i, row) in content.iter().enumerate() {
        let row = row.chars().collect::<Vec<char>>();
        let guard = row.iter().position(|x| *x == '^');
        match guard {
            Some(p) => {
                start = Some((i, p, Directions::UP));
            }
            _ => (),
        };
        matrix.push(row);
    }

    let mut count = 0;

    for (i, row) in matrix.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if matrix[i][j] == '#' {
                continue;
            }
            let mut matrix = matrix.clone();
            matrix[i][j] = '#';
            count += if !traverse2(&matrix, start.clone()) {
                println!("blocked: {i}, {j}");
                1
            } else {
                0
            };
        }
    }

    println!("AOC Day 6 - Part 2: {}", count)
}

fn traverse2(matrix: &Vec<Vec<char>>, start: Option<(usize, usize, Directions)>) -> bool {
    // println!("matrix: ");
    // matrix.iter().for_each(|row| println!("{:?}", row));

    let mut position = start.unwrap();
    let mut result = HashSet::new();

    loop {
        let (x, y, d) = position;
        if result.contains(&(x, y, d)) {
            return false;
        }
        // println!("x: {}, y: {}, d: {:?}", x, y, d);
        let (dx, dy): (i32, i32) = match d {
            Directions::UP => (-1, 0),
            Directions::DOWN => (1, 0),
            Directions::LEFT => (0, -1),
            Directions::RIGHT => (0, 1),
        };
        let ix = x as i32 + dx;
        let iy = y as i32 + dy;

        if !(0 <= ix && ix < matrix.len() as i32 && 0 <= iy && iy < matrix[0].len() as i32) {
            break;
        }
        match matrix.get(ix as usize).unwrap().get(iy as usize) {
            Some(p) => {
                let (x, y, d) = match p {
                    '#' => {
                        let new_d = match d {
                            Directions::UP => Directions::RIGHT,
                            Directions::RIGHT => Directions::DOWN,
                            Directions::DOWN => Directions::LEFT,
                            Directions::LEFT => Directions::UP,
                        };
                        // println!("DIRECTION MOVE OCCURRED: {:?}", new_d);

                        (x, y, new_d)
                    }
                    _ => {
                        result.insert((x as usize, y as usize, d.clone()));
                        (ix as usize, iy as usize, d)
                    }
                };

                position = (x as usize, y as usize, d.clone());
                // println!("result: {:?}", result);
            }
            None => break,
        }
    }
    true
}
