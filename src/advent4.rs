use std::{fs::read_to_string, io, ops::Index, collections::{HashSet, BTreeSet}, iter::Inspect};

use itertools::Itertools;

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

    let cards = init_cards(&rows);
    let mut index = 5;

    'outer: loop {
        if index > 99 { break; }
        let drawn_slice: BTreeSet<&i32> = drawn_numbers[0..index].into_iter().collect();

        for (inner_index, card) in cards.iter().enumerate() {
            let tmp_card: BTreeSet<&i32> = card
                .clone()
                .into_iter()
                .collect();                        

            let check: Vec<&i32> = drawn_slice
                .intersection(&tmp_card)
                .cloned()
                .collect::<Vec<&i32>>();

            if check.len() >= 5 {
                /* println!("intersection: {:?}", check); */
                /* println!("more than 5 matches: {:?}", card); */
                let cols = check_cols(card, &check);
                let rows = check_rows(card, &check);

                if cols.len() > 0 || rows.len() > 0 {
                    println!("card: {:?},\ncard_index: {},\nmatch: {:?},\nslice_outer_index: {},\nrows: {:?},\ncols: {:?}", card, inner_index, check, index, rows, cols);
                    ppcard(card);
                    card_coefficient(
                        card,
                        &drawn_numbers[index..].iter().collect_vec(),
                        &drawn_numbers[index-1]
                    );
                    break 'outer;
                }

            } else {
                continue;
            }
        }
        index += 1;
    }

    Ok(())
}

fn ppcard(card: &Vec<&i32>) {
   for i in 0..5 {
       println!("{:?}", get_row(card, i));
   } 
}

fn check_cols<'a>(card: &'a Vec<&i32>, intersection: &'a Vec<&i32>) -> Vec<&'a i32> {
    let mut found = Vec::new();

    for col in 0..5  {
        let tmp_col: BTreeSet<_> = get_col(card, col).into_iter().collect();
        let tmp_inter: BTreeSet<_> = intersection.clone().into_iter().collect();

        /* println!("intersection: {:?} -> col: {:?}", tmp_inter, tmp_col); */
        if tmp_inter.is_superset(&tmp_col) {
            found = tmp_col.into_iter().collect();
            break;
        } 
    }
    found
}

fn check_rows<'a>(card: &'a Vec<&i32>, intersection: &'a Vec<&i32>) -> Vec<&'a i32> {
    let mut found = Vec::new();

    for row in 0..5  {
        let tmp_row: BTreeSet<_> = get_row(card, row).into_iter().collect();
        let tmp_inter: BTreeSet<_> = intersection.clone().into_iter().collect();

        if tmp_inter.is_superset(&tmp_row) {
            found = tmp_row.into_iter().collect();
            break;
        } 
    }
    found
}

fn get_col<'a>(arr: &'a Vec<&i32>, index: usize) -> Vec<&'a i32> {
    let mut iter = arr.into_iter();

    for _ in 0..index {
        iter.next();
    }

    let tmp = iter
        .step_by(5)
        .map(|&el| el)
        .collect();

    tmp
}

fn get_row<'a>(arr: &'a Vec<&i32>, index: usize) -> Vec<&'a i32> {
    let low = index * 5;
    let high = low + 5;


    let tmp = arr
        .index(low..high)
        .to_vec();

    // println!("get_row -> {} -> {:?}", index, tmp);
    return tmp;
}

fn card_coefficient(card: &Vec<&i32>, slice_complement: &Vec<&i32>, last_number: &i32) {
    println!("card: {:?}", card);
    println!("bingo_row: {:?}", slice_complement);
    println!("last_number: {:?}", last_number);

    let card: BTreeSet<&i32> = card.clone().into_iter().collect();
    let slice_complement: BTreeSet<&i32> = slice_complement.clone().into_iter().collect();

    let intersection: Vec<_> = card
        .difference(&slice_complement)
        .cloned()
        .collect();

    println!("complement: {:?}", intersection);
    println!("complement len: {:?}", intersection.len());

    let sum_of_unmarked = intersection
        .into_iter()
        .map(|&el| el)
        .reduce(|acc, el| acc + el)
        .unwrap();


    println!("sum of unmarked: {}", sum_of_unmarked);
    let card_coefficient = sum_of_unmarked * last_number;
    println!("card_coefficient: {}", card_coefficient);
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

fn init_cards(rows: &Vec<i32>) -> Vec<Vec<&i32>> {
    let mut cards: Vec<Vec<&i32>> = vec![];

    for index in 0..99 {
        cards.push(get_bingo_card(&rows, index).into_iter().collect::<Vec<&i32>>());
    }

    cards
}
