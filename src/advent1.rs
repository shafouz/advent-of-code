use std::{fs::File, io::Read, u32};

#[derive(Debug)]
struct Scan {
    n: u32,
    start_index: u32,
    count: Vec<u32>,
}

pub fn advent1() -> std::io::Result<()> {
    let mut input = File::open("advent1.txt")?;
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;

    let list: Vec<(usize,u32)> = buf
        .trim()
        .split("\n")
        .map(|str| u32::from_str_radix(str, 10).unwrap())
        .enumerate()
        .collect();

    let mut scan = Scan {
        n: 0,
        start_index: 1,
        count: vec![]
    };

    scan.n = 173;
    // need to count each index only once
    // 173 < 0
    // 175 < 1 << 0
    // 171 < 2 << 1
    // 177     << 2
    // 179
    let mut temp;
    for (index, item) in &list[..list.len() - 2] {
        temp = 0;
        for (ay, n) in &list[*index..*index+3] {
            temp += n;
            // println!("{}",ay);
        }
        // break;
        scan.count.push(temp);
        // println!("index: {}, item: {}, scan.count: {:?}", index, item, scan.count);
    }

    scan.n = *scan.count.first().unwrap();
    for item in &scan.count[1..] {
        if *item < scan.n {
         println!("item {}, scan {}, i {}",item,scan.n,scan.start_index);
            scan.start_index += 1;
            scan.n = *item;
        } else {
         // println!("item {}, scan {}, i {}",item,scan.n,scan.start_index);
            scan.n = *item;
        }
    }

    // for i in &scan.count { println!("{}",i) }
    // println!("{}", scan.start_index);

    Ok(())
}
