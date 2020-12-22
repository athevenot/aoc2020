fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let passwords = include_str!("../assets/input.txt").lines();
    let mut i = 0;

    for e in passwords {
        let raw: Vec<&str> = e.split(|c: char| c == '.' || c == '-' || c == ':' || c == ' ').collect();
        let lo :usize = raw[0].parse().unwrap();
        let hi :usize = raw[1].parse().unwrap();
        let c:char = raw[2].parse().unwrap();
        let data: &str = raw[4];

        let count = count_char(data, c);
        if count >= lo && count <= hi {
            i += 1;
        }
    }
    println!("{}", i);
}

fn part_two() {
    let passwords = include_str!("../assets/input.txt").lines();
    let mut i = 0;

    for e in passwords {
        let raw: Vec<&str> = e.split(|c: char| c == '.' || c == '-' || c == ':' || c == ' ').collect();
        let pos :usize = raw[0].parse().unwrap();
        let pos2 :usize = raw[1].parse().unwrap();
        let c:char = raw[2].parse().unwrap();
        let data: &str = raw[4];
        
        if (data.as_bytes()[pos - 1] == c as u8 && data.as_bytes()[pos2 - 1] != c as u8) || (data.as_bytes()[pos2 - 1] == c as u8 && data.as_bytes()[pos - 1] != c as u8) {
            i += 1;
        }
    }
    println!("{}", i);
}

fn count_char(s: &str, c: char) -> usize {
    let mut i = 0;
    for e in s.chars() {
        if e == c {
            i += 1;
        }
    }
    i
}