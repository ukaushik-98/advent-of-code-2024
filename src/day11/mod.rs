pub async fn day11() {
    let _ = part_1("0 37551 469 63 1 791606 2065 9983586", 75);
}

fn part_1(input: &str, i: usize) {
    println!("input: {}, i: {}", input, i);
    let input = input
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    println!("input: {:?}", input);
    let mut curr: Option<Vec<usize>> = Some(input);
    for j in 0..i {
        println!("curr loop: {}", j);
        let mut next_state: Vec<usize> = Vec::new();
        for i in curr.unwrap().iter_mut() {
            let i_str = i.to_string();
            let length = i_str.len();
            match length % 2 {
                0 => {
                    let left = &i_str[..length / 2];
                    let right = &i_str[length / 2..];
                    // println!("i_sr: {}, left: {}, right: {}", i_str, left, right);
                    let left = left.parse::<usize>().unwrap();
                    let right = right.parse::<usize>().unwrap();
                    next_state.push(left);
                    next_state.push(right);
                }
                _ => {
                    if *i == 0 {
                        next_state.push(1);
                    } else {
                        next_state.push(*i * 2024);
                    }
                }
            }
        }
        curr = Some(next_state);
    }
    println!("curr: {:?}", curr.unwrap().len());
}
