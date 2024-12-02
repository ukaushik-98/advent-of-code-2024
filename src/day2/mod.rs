use tokio::fs;

pub async fn read_data_part_1() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let buf = fs::read("./src/day2/real.txt").await.unwrap();

    let content: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buf);
    let mut content: Vec<&str> = content.split("\n").collect();

    let mut count = 0;

    for raw_pair in content.iter_mut() {
        let raw_pair: Vec<usize> = raw_pair
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        let mut permute = Vec::new();
        let len = raw_pair.len();
        permute.push(raw_pair.clone());
        let mut valid = false;
        for perm in permute.iter_mut() {
            valid |= compare(perm.to_vec());
            if valid {
                break;
            }
        }
        count += if valid { 1 } else { 0 };
    }
    println!("Day 2 Part 1 Count: {count}");
    Ok(())
}

pub async fn read_data_part_2() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let buf = fs::read("./src/day2/real.txt").await.unwrap();

    let content: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buf);
    let mut content: Vec<&str> = content.split("\n").collect();

    let mut count = 0;

    for raw_pair in content.iter_mut() {
        let raw_pair: Vec<usize> = raw_pair
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        let mut permute = Vec::new();
        let len = raw_pair.len();
        permute.push(raw_pair.clone());
        for i in 0..len {
            let mut new_vec = Vec::new();
            new_vec.extend_from_slice(&raw_pair[..i]);
            new_vec.extend_from_slice(&raw_pair[i + 1..]);
            permute.push(new_vec);
        }
        let mut valid = false;
        for perm in permute.iter_mut() {
            valid |= compare(perm.to_vec());
            if valid {
                break;
            }
        }
        count += if valid { 1 } else { 0 };
    }
    println!("Day 2 Part 2 Count: {count}");
    Ok(())
}

fn compare(mut data: Vec<usize>) -> bool {
    let desc = data.is_sorted_by(|a, b| a < b);
    let asc = data.is_sorted_by(|a, b| a > b);
    if !(asc || desc) {
        return false;
    }

    let mut data = data.iter_mut();
    let mut curr = data.next().unwrap();

    loop {
        let nxt = data.next();
        match nxt {
            Some(nxt) => {
                let dif = curr.abs_diff(*nxt);
                // println!("cur: {curr}, nxt: {nxt}, dif: {dif}");
                if dif == 0 || dif > 3 {
                    return false;
                }
                curr = nxt;
            }
            _ => break,
        }
    }
    true
}
