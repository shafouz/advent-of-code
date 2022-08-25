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

    println!("{}", drawn_numbers.len());
    // let a = &rows.into();
    // check_results(&rows, &drawn_numbers);
    gen_permutations(&rows);

    Ok(())
}

fn gen_permutations(rows: &[i32]){
    let filter_group_size = 1;
    let keep_track = &rows;
    let owned = keep_track.to_vec();

    // println!("{:?}", b);
    
    // let test: HashSet<&[i32]> = HashSet::from(keep_track);
    // let test: Vec<HashSet<_>> = iproduct!(keep_track,keep_track,keep_track,keep_track,keep_track).unique().collect();
    // let mut myhash: HashSet<_> = HashSet::new();
    let mut test: BTreeSet<BTreeSet<&i32>> = BTreeSet::new();

    let init_permutations = iproduct!(
        owned[0..8].iter(),
        owned[0..8].iter(),
        owned[0..8].iter(),
        owned[0..8].iter(),
        owned[0..8].iter()
    )
        .for_each(|(a,b,c,d,e)| { 
            let mut tmp: BTreeSet<&i32> = BTreeSet::new();
            tmp.insert(a);
            tmp.insert(b);
            tmp.insert(c);
            tmp.insert(d);
            tmp.insert(e);
            test.insert(tmp);
        });

    test = test.into_iter().filter(|el| el.len() == 5).collect();
    println!("{:?}",test.len());
    // let init_permutations = iproduct!(
    //     owned[0..10].iter(),
    //     owned[0..10].iter()
    // )
    //     .for_each(|(a,b)| { 
    //         let mut tmp: HashSet<_> = HashSet::new();
    //         tmp.insert(a);
    //         tmp.insert(b);
    //         test.push(tmp);
    //     });

}

fn check_results(rows: &Vec<i32>, drawn_numbers: &Vec<i32>) {
    let mut high_bound = 5;

    while high_bound < 100 {
        for index in 0..99 {
            let bingo_card: HashSet<&i32> = HashSet::from_iter(get_bingo_card(rows, index));
            // let n_drawn: HashSet<&i32> = HashSet::from_iter(drawn_numbers.index(0..high_bound));
            // .into();

            let is_bingo = true;// n_drawn.is_subset(&bingo_card);

            if is_bingo == true {
                println!("bingo_card: index = {}", index);
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
