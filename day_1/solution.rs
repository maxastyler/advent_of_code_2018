use std::collections::HashSet;

fn main() {
    let f = include_str!("./input.txt")
        .lines()
        .map(|x| x.parse::<i64>().unwrap());

    println!("First part: {}", f.clone().fold(0, |a, x| a + x));

    let mut visited = HashSet::new();
    let mut freq = 0;

    for i in f.cycle() {
        freq += i;
        if !visited.insert(freq) { break }
    }
    println!("{}", freq);
}
