use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {

    // NOTE: Part 1

    let file = File::open("./binary.txt")?;
    let reader = BufReader::new(file);

    let binary_data: Vec<Vec<u32>> = reader.lines()
        .filter_map(|x| x.unwrap().parse::<String>().ok()
                    .map(|char| char.chars()
                         .map(|dig| char::to_digit(dig, 2).unwrap())
                         .collect()))
        .collect();

    let grid_half = binary_data.len() as u32 / 2;
    let vec_len = binary_data[0].len();
    let mut tally = vec![0; vec_len];

    binary_data.iter().for_each(|line| {
        for i in 0..vec_len {
            tally[i] += line[i]
        }
    });

    let gamma_pre = tally.clone().into_iter().map(|x| if x > grid_half { 1 } else { 0 });
    let gamma_post: String = gamma_pre.map(|x| x.to_string()).collect();
    let gamma = isize::from_str_radix(&gamma_post, 2).unwrap();


    let epsilon_pre = tally.into_iter().map(|x| if x > grid_half { 0 } else { 1 });
    let epsilon_post: String = epsilon_pre.map(|x| x.to_string()).collect();
    let epsilon = isize::from_str_radix(&epsilon_post, 2).unwrap();

    println!("{}", gamma * epsilon);

    //**********************************************************************************

    // NOTE: Part 2



    Ok(())
}
