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

    check_results(&rows, &drawn_numbers);

    Ok(())
}

fn check_results(rows: &Vec<i32>, drawn_numbers: &Vec<i32>) {
    'outer: for choose_n in 5..drawn_numbers.len() {
        println!("choosing n: {}", choose_n);

        let permutations = gen_permutations(&drawn_numbers, choose_n);

        for card in 0..99 {
            println!("card: {}", card);

            let bingo_card: HashSet<&i32> = get_bingo_card(&rows, card)
                .into_iter()
                .collect();

            let drawn_set_index = permutations
                .iter()
                .rposition(|el| {
                    el.is_subset(&bingo_card)
                });

            match drawn_set_index {
                Some(drawn_set_index) => { 
                    let drawn_set = permutations[drawn_set_index].clone();
                    let bingo_card = get_bingo_card(&rows, card);
                    let found = check_card(bingo_card, drawn_set);
                    if found { break 'outer };
                    // matched on bingo card, now check the bingo card rows and columns
                }
                None => { continue; }
            }
        }
    }
}

fn check_card(card: Vec<&i32>, choosen_5: HashSet<&i32>) -> bool {
    let found: BTreeSet<&i32> = choosen_5.into_iter().collect();
    let mut z = false;

    for index in 0..5 {
        let col: BTreeSet<&i32> = get_col(&card, index)
            .into_iter()
            .collect();

        let row: BTreeSet<&i32> = get_row(&card, index)
            .into_iter()
            .collect();

        let col_intersection = col.is_superset(&found);
        let row_intersection = col.is_superset(&found);

        if col_intersection {
            println!("found: col {}, card {:?}", index, card);
            z = true;
            return z
        }

        if row_intersection {
            println!("found: row {}, card {:?}", index, card);
            z = true;
            return z
        }
    }

    z
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

fn gen_permutations(arr: &[i32], choose_n: usize) -> Vec<HashSet<&i32>> {
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
            let mut tmp: BTreeSet<&i32> = BTreeSet::new();
            tmp.insert(a);
            tmp.insert(b);
            tmp.insert(c);
            tmp.insert(d);
            tmp.insert(e);
            permutation_tree.push(tmp.into_iter().collect::<Vec<&i32>>());
        });

    let tmp = permutation_tree.into_iter().collect::<BTreeSet<_>>();

    tmp
        .into_iter()
        .map(|el| HashSet::from_iter(el))
        .filter(|el| el.len() == 5)
        .collect::<Vec<HashSet<&i32>>>()
}

