// forward, down, up
// depth
// horizontal
// forward +horizontal
// down +depth
// up -depth

use std::{fs::{File, read_to_string}, io};

#[derive(Debug)]
struct Cords {
    depth: i32,
    h_position: i32,
}

pub fn advent2() -> io::Result<()>{
    let file = read_to_string("inputs/advent2.txt")?;

    let trimmed: Vec<(&str,&str)> = file
        .trim()
        .split("\n")
        .map(|item| item.split_once(" ").unwrap() )
        .collect();

    let mut cords = Cords {
        depth: 0,
        h_position: 0 
    };

    for (direction, distance) in trimmed {
        match direction {
            "forward" => { cords.h_position += distance.parse::<i32>().unwrap(); }
            "down" => { cords.depth += distance.parse::<i32>().unwrap(); }
            "up" => { cords.depth -= distance.parse::<i32>().unwrap(); }
            _ => {}
        }
    }
    println!("{:?}", cords);
    Ok(())
}
