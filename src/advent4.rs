use std::{fs::read_to_string, io, ops::Index};

pub fn advent4() -> io::Result<()>{
    let file = read_to_string("inputs/advent4.txt")?;

    let drawn_numbers: Vec<i32> = file
        .split("\n")
        .nth(0)
        .and_then(|line| Some(line.split(",").map(|el| el.parse::<i32>().unwrap()).collect::<Vec<i32>>()))
        .unwrap();

    let rows: Vec<i32> = file.split_once("\n").unwrap().1
        .split("\n")
        .map(|str| str.split(" ")
            .filter(|el| !el.is_empty())
            .map(|el| el.parse::<i32>().unwrap())
            .collect::<Vec<i32>>())
        .flatten()
        .collect::<Vec<i32>>();

    // let a = &rows.into();
    // check_results(&rows, &drawn_numbers);

    Ok(())
}

fn check_results(rows: &Vec<i32>, drawn_numbers: &Vec<i32>) {
    let mut high_bound = 5;

    while high_bound < 100 {
        for index in 0..99 {
            let bingo_card = get_bingo_card(rows, index);

            let bingo = drawn_numbers.index(0..high_bound)
                .iter()
                .all(|number|
                    bingo_card.contains(&number)
                );

            if bingo == true {
                println!("{}", index);
                break
            }
        }
        high_bound += 1;
    }
}

fn get_bingo_card(arr: &Vec<i32>, index: usize) -> Vec<&i32> {
    if index >= 100 { 
        return arr.iter().collect()
    }

    let low = index * 25;
    let high = low + 25;

    let tmp = arr.index(low..high)
        .iter()
        .collect();

    return tmp;
}

fn get_col(arr: &Vec<i32>, index: usize) -> Vec<&i32> {
    let reindex = index % 5;
    let low = reindex + ((index / 5) * 25);
    let mut high = low + 25;

    if high >= 2500 { 
        high = 2500
    };

    let tmp: Vec<&i32> = arr.index(low..high)
        .iter()
        .step_by(5)
        .collect::<Vec<&i32>>();

    return tmp;
}

fn get_row(arr: &Vec<i32>, index: usize) -> Vec<&i32> {
    let low = index * 5;
    let high = low + 5;


    let tmp = arr.index(low..high)
        .iter()
        .collect();
    return tmp;
}
