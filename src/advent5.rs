use std::{fs::read_to_string, io};

use itertools::Itertools;

pub fn advent5() -> io::Result<()>{
    let file = read_to_string("inputs/advent5.txt")?;

    let flattened_arr: Vec<_> = file
        .trim()
        .split("\n")
        .map(|str| str.split(" -> ").collect_vec())
        .flatten()
        .map(|str| str.split(",").collect_vec())
        .flatten()
        .map(|el| el.parse::<i32>().unwrap())
        .collect_vec();

    let mut tmp = flattened_arr.clone();
    tmp.sort();
    let max = tmp.last().unwrap().to_owned();

    let mut imprint = vec![0; max as usize];

    Ok(())
}
