use std::{io, fs::read_to_string};

// power = gamma * epsilon
// gamma = most common bit of each column
// set 1000

// start at leftmost
// drop from vector if doesnt match
// if there is only one go to left
pub fn advent3() -> io::Result<()> {
    let file: Vec<String> = read_to_string("inputs/advent3.txt")?
        .trim()
        .split("\n")
        .map(String::from)
        .collect();

    let file1 = file.clone();
    let mut arr: Vec<&String> = file.iter().collect();
    let mut arr1: Vec<&String> = arr.clone();
    let mut a: Vec<&String> = arr.clone();
    let mut b: Vec<&String> = arr.clone();

    for item in 0..12 {
        let most_common = get_most_common(&arr, item, '0');
        // println!("len2: {}", arr.len());
        arr.retain(|n| n.chars().nth(item).unwrap() == most_common);
        if arr.len() == 1 { a = arr; break; }
    }
    for item in 0..12 {
        let most_common = get_most_common(&arr1, item, '1');
        // println!("len2: {}", arr.len());
        arr1.retain(|n| n.chars().nth(item).unwrap() == most_common);
        if arr1.len() == 1 { b = arr1; break; }
    }
    // 001110011011
    // 101011011110
    let c = u32::from_str_radix(a[0], 2).unwrap();
    let d = u32::from_str_radix(b[0], 2).unwrap();

    println!("{}", c * d);


    Ok(())
}

fn get_most_common(arr: &Vec<&String>, index: usize, bit: char) -> char {
    let count: i32 = arr.iter().filter(|n| n.chars().nth(index).unwrap() == '1').count() as i32;
    println!("len: {}", arr.len());
    let lenx: i32 = (arr.len() as i32 + 1) / 2;

    if bit == '1' {
        return if count >= lenx { '0' }
        else { '1' }
    } else {
        return if count < lenx { '0' }
        else { '1' }
    }
    // if bit = 0
    // 
}
