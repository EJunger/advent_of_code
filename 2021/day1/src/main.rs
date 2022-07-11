use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> std::io::Result<()> {

    // NOTE: Part 1

    let file = File::open("./depth_readings.txt")?;
    let reader = BufReader::new(file);

    let depths: Vec<i32> = reader.lines()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect();

    let mut count = 0;
    let mut temp = &depths[0];

    for curr in &depths {
        if curr > temp {
            count += 1;
        }
        temp = curr
    }

    println!("The final count is: {}", count);

    //*************************************************************************

    // NOTE: Part 2

    let windows = depths.windows(3);
    let mut vals = Vec::new();

    for window in windows {
        let summed_window: i32 = window.iter().sum();
        //println!("{:?}", summed_window);
        vals.push(summed_window)
    }

    let mut summed_count = 0;
    let mut temp = &vals[0];

    for curr in &vals {
        if curr > temp {
            summed_count += 1;
        }
        temp = curr
    }

    println!("The final summed count is: {}", summed_count);

    Ok(())
}

