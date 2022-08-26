use std::{fs::read_to_string, io, ops::Index, collections::{HashSet, BTreeSet}, iter::Inspect};

use itertools::{Itertools, iproduct};

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

    // check_results(&rows, &drawn_numbers);
    // get_bingo_card(&rows, 48);
    // get_row(&rows, 1);
    let pm6 = gen_permutations(&rows, 6);
    for item in &pm6 {
        for item2 in &pm6  {
            if item == item2 {
                println!("{:?} => {:?}", item, item2);
            }
        }
    }
    // println!("{:?}", pm6);
    // println!("len: {:?}", pm6.len());
    // let a : Vec<_> = vec![1,2,3,4,5];
    // let b : Vec<_> = vec![2,1,3,4,5];
    // println!("{}", a == b);
//    let pm7: Vec<Vec<_>> = pm6
//        .into_iter()
//        .filter(|el| el.)
//
//    println!("{:?}", pm7);
//    println!("len: {:?}", pm7.len());

    Ok(())
}

fn check_results(rows: &Vec<i32>, drawn_numbers: &Vec<i32>) {
    'outer: for choose_n in 5..drawn_numbers.len() {
        let permutations = gen_permutations(&drawn_numbers, choose_n);

        for card in 0..99 {
            let bingo_card: HashSet<&i32> = get_bingo_card(&rows, card).into_iter().collect();
            // let is_bingo_index = permutations.iter().rposition(|el| el.is_subset(&bingo_card));

            // match is_bingo_index {
                // Some(row_index) => { 
                    // let choosen_5 = permutations.iter().nth(row_index).unwrap();
                    // check_cols(rows, choosen_5);
                    // check_rows(&bingo_card, choosen_5)
                    // check if cols match returning an option with the row number, same for col
                // }
                // None => { continue; }
            // }
        }
    }
}

fn check_cols(rows: &Vec<i32>, choosen_5: &HashSet<&i32>) {
    let i: usize = 1;
    let card = get_bingo_card(&rows, i);
}

fn get_col(arr: Vec<&i32>, index: usize) {
    let reindex = index % 5;
    let low = reindex + ((index / 5) * 25);
    let mut high = low + 25;

    if high >= 2500 { 
        high = 2500
    };

}

fn get_row(arr: &Vec<i32>, index: usize) -> Vec<&i32> {
    let low = index * 5;
    let high = low + 5;


    let tmp = arr.index(low..high)
        .iter()
        .collect();

    println!("get_row -> {} -> {:?}", index, tmp);
    return tmp;
}

fn check_rows(bingo_card: &HashSet<&i32>, choosen_5: &HashSet<&i32>) {
    for i in 0..5 {
        // get_row(bingo_card, i)
    }
}

                    // println!("bingo_card_index: {:?}, bingo_row: {:?}, last_number_index: {}", card, bingo_index, choose_n);
                    // card_coefficient(
                        // &bingo_card,
                        // permutations.iter().nth(bingo_index).unwrap(),
                        // drawn_numbers.iter().nth(choose_n).unwrap() 
                    // );
                    // break 'outer;
fn card_coefficient(card: &HashSet<&i32>, bingo_5: &HashSet<&i32>, last_number: &i32) {
    println!("card: {:?}", card);
    println!("bingo_row: {:?}", bingo_5);
    println!("last_number: {:?}", last_number);

    let unmarked_numbers: _ = card
        .difference(bingo_5)
        .into_iter()
        .map(|&&el| el)
        .reduce(|acc, el| acc + el)
        .unwrap();

    let card_coefficient = unmarked_numbers * last_number;
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


fn gen_permutations(arr: &[i32], choose_n: usize) -> Vec<Vec<&i32>> {
    let keep_track = &arr;

    let mut permutation_tree: Vec<Vec<&i32>> = Vec::new();

    let init_permutations = iproduct!(
        keep_track[0..choose_n].iter(),
        keep_track[0..choose_n].iter(),
        keep_track[0..choose_n].iter(),
        keep_track[0..choose_n].iter(),
        keep_track[0..choose_n].iter()
    )
        .for_each(|(a,b,c,d,e)| { 
            let mut tmp: HashSet<&i32> = HashSet::new();
            tmp.insert(a);
            tmp.insert(b);
            tmp.insert(c);
            tmp.insert(d);
            tmp.insert(e);
            permutation_tree.push(tmp.into_iter().collect::<Vec<&i32>>());
        });

    let tmp = permutation_tree.into_iter().collect::<HashSet<_>>();

    permutation_tree = tmp
        .into_iter()
        .filter(|el| el.len() == 5)
        .collect();
    permutation_tree
}

