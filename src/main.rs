use std::{fs::{File, read_to_string}, io::{Read, self}};

mod advent3_sol;
mod advent1;
mod advent2;
mod advent3;
mod advent4;
// mod advent1_sol;

// use std::io::prelude::*;


fn main() -> io::Result<()>{
    advent4::advent4()?;
    Ok(())
}

