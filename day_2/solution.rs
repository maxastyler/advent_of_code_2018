fn main() {
    let input: Vec<_> = include_str!("./input.txt").lines().collect();
    let letter_counted = input.iter().map(|s| (0..26).map(move |c: u8| s.chars()
                                           .filter(move |&l| l==(c+97) as char)
                                           .count())
                              .collect::<Vec<_>>());
    println!("Checksum: {}", 
             letter_counted.clone().filter(|x| x.contains(&2)).count() *
             letter_counted.clone().filter(|x| x.contains(&3)).count());

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

    let line_numbers: (usize, usize) = repeated.filter_map(|(x, y)| if x==25 { Some(y) } else { None } ).next().unwrap();

    println!("Line number | Line\n{} | {}\n{} | {}", 
             line_numbers.0, input[line_numbers.0], line_numbers.1, input[line_numbers.1]);

    println!("Common letters: {}", input[line_numbers.0]
             .chars()
             .zip(input[line_numbers.1].chars())
             .filter_map(|(a, b)| if a==b { Some(a) } else { None })
             .collect::<String>())
}
