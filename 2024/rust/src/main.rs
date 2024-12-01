use std::iter::zip;
use std::path::absolute;
use std::usize;

use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let distance = read_file().await.unwrap();
    println!("Distance: {}", distance)
}

async fn read_file() -> Result<usize, Box<dyn std::error::Error + 'static>> {
    let mut f = File::open("./day1/real.txt").await?;
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

    loc1.sort();
    loc2.sort();

    println!("list1: {:?} \nlist2: {:?}", loc1, loc2);

    let mut total = 0;
    for (x, y) in zip(loc1, loc2) {
        total += (x).abs_diff(y);
    }
    Ok(total)
}
