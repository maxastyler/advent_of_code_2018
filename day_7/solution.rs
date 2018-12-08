use std::collections::HashMap;

fn main() {
    let mut depends: HashMap<char, Vec<char>> = HashMap::new();
    include_str!("./input.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .for_each(|c| {
            depends.entry(c[5]).or_insert(vec![]).push(c[36]);
            depends.entry(c[36]).or_insert(vec![]);
        });
    let mut num_blocking: HashMap<char, i32> = HashMap::new();
    depends.keys().for_each(|&k| {
        num_blocking.insert(
            k,
            depends
                .values()
                .filter(|a| a.contains(&k))
                .collect::<Vec<_>>()
                .len() as i32,
        );
    });

    let mut ordered: String = String::new();

    while let Some(&c) = num_blocking
        .iter()
        .filter_map(|(k, &v)| if v == 0 { Some(k) } else { None })
        .min()
    {
        ordered.push(c);
        if let Some(b) = num_blocking.get_mut(&c) {
            *b -= 1;
        }
        depends.get_mut(&c).unwrap().iter().for_each(|blocked| {
            if let Some(b) = num_blocking.get_mut(blocked) {
                *b -= 1
            }
        });
    }

    println!("{}", ordered);
}
