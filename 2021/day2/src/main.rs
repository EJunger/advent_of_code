use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {

    // NOTE: Part 1

    let file = File::open("./directions.txt")?;
    let reader = BufReader::new(file);

    let directions_vec: Vec<String> = reader.lines()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap().parse::<String>().unwrap())
        .collect();

    let mut directions: Vec<String> = Vec::new();
    let mut values: Vec<i32> = Vec::new();

    for cmd in directions_vec {
        let mut split = cmd.splitn(2, ' ');
        directions.push(split.next().unwrap().to_string());
        values.push(split.next().unwrap().parse::<i32>().unwrap());
    }

    let mut depth = 0;
    let mut horizontal = 0;

    for (cmd, val) in directions.iter().zip(values.iter()) {
        match cmd.as_str() {
            "forward" => horizontal += *val,
            "up" => depth -= *val,
            "down" => depth += *val,
            _ => ()
        }
    }

    println!("{}", depth);
    println!("{}", horizontal);
    println!("The final total is: {}", depth * horizontal);

    //*************************************************************************

    // NOTE: Part 2

    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for (cmd, val) in directions.iter().zip(values.iter()) {
        match cmd.as_str() {
            "forward" => {
                horizontal += *val;
                depth += aim * *val;
            },
            "up" => aim -= *val,
            "down" => aim += *val,
            _ => ()
        }
    }

    println!("{}", depth);
    println!("{}", horizontal);
    println!("The final total is: {}", depth * horizontal);

    Ok(())
}
