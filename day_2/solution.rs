fn main() {
    let input = include_str!("./input.txt")
        .lines()
        .map(|s| (0..26).map(move |c: u8| s.chars()
                                           .filter(move |&l| l==(c+97) as char)
                                           .collect::<Vec<_>>()
                                           .len())
                        .collect::<Vec<_>>());
    println!("{:?}", input.clone().filter(|x| x.contains(&2)).collect::<Vec<_>>().len() *
                     input.clone().filter(|x| x.contains(&3)).collect::<Vec<_>>().len());
}
