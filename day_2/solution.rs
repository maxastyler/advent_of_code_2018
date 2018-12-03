fn main() {
    let input: Vec<_> = include_str!("./input.txt").lines().collect();
    let letter_counted = input.iter().map(|s| (0..26).map(move |c: u8| s.chars()
                                           .filter(move |&l| l==(c+97) as char)
                                           .collect::<Vec<_>>()
                                           .len())
                              .collect::<Vec<_>>());
    println!("Checksum: {}", 
             letter_counted.clone().filter(|x| x.contains(&2)).collect::<Vec<_>>().len() *
             letter_counted.clone().filter(|x| x.contains(&3)).collect::<Vec<_>>().len());

    let repeated = input.iter()
                         .enumerate()
                         .map(|(i, x)| input.iter()
                                            .skip(i+1)
                                            .enumerate()
                                            .map(move |s| (s.1.chars()
                                                           .zip(x.chars())
                                                           .map(|(a, b)| (a==b) as usize)
                                                           .sum::<usize>(), (i, s.0+i+1)))
                                            .collect::<Vec<_>>())
                         .flatten();

    let line_numbers: (usize, usize) = repeated.filter(|(x, _) | x==&25)
        .map(|(_, x)| x).next().unwrap();

    println!("Line number | Line\n{} | {}\n{} | {}", 
             line_numbers.0, input[line_numbers.0], line_numbers.1, input[line_numbers.1]);

    println!("Common letters: {}", input[line_numbers.0]
             .chars()
             .zip(input[line_numbers.1].chars())
             .filter(|(a, b)| a==b)
             .map(|a| a.0)
             .collect::<String>())
}
