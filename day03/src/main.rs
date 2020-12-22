use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

// We use BufReader since we won't go back in file and we don't need to store it in memory.
#[allow(unused_must_use)]
fn main() {
    part_one();
    part_two();
}

fn part_two() {
    println!("{}", [(1,1), (3,1), (5,1), (7,1), (1,2)].iter()
             .map(|&(x,y)| include_str!("../assets/input.txt")
                  .lines()
                  .step_by(y)
                  .enumerate()
                  .filter(|(i,row)| row.as_bytes()[(i *x)%row.len()]==b'#')
                  .count()
             ).product::<usize>());
}

fn part_one() -> std::io::Result<()> {
    let f = File::open("assets/input.txt")?;
    let mut reader = BufReader::new(f);
    let mut i: usize = 0;
    let mut num_tree: usize = 0;
    loop {
        let mut buf: [u8; 32] = [0; 32];
        let len = reader.read(&mut buf[..])?;
        if is_tree(buf[i]) { num_tree += 1; }
        if len == 0 { break } // When file over, leave the infinite loop
        i += 3;
        if i >= 31 {
            i %= 31;
        }
    }
    println!("Santa hits {} trees.", num_tree);
    Ok(())
}

fn is_tree(c: u8) -> bool {
    if c as char == '#' {
        return true
    }
    false
}