fn main() {
    let mut input: Vec<u32> = include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    input.sort();
    input.reverse();
    for a in &input {
        for b in &input {
            for c in &input {
                if a + b + c == 2020 {
                    println!("{}", a * b * c);
                    return ;
                }
            }
        }
    }
}
