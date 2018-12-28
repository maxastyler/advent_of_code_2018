fn main() {
    let lines = include_str!("./input.txt").lines().collect::<Vec<_>>();
    let initial = &lines[0][15..];
    let rules = lines[2..].iter().map(|l| (&l[0..5], &l[9..10])).collect::<Vec<_>>();
    let mut new = initial.to_owned();
    let mut expanded: String;
    let iterations = 100;
    for i in 0..iterations {
        expanded = "....".to_owned() + &new + &"....".to_owned();
        new = "".to_owned();
        for i in 0..(expanded.len()-4) {
            new += rules.iter().filter(|(m, _)| *m == &expanded[i..i+5]).next().unwrap().1;
        }
        new.trim_matches('.');
        println!("{}, {}", i, new.chars().zip(-2*i..).map(|(c, j)| match c {'.' => 0, _ => j}).sum::<i32>());
    }
    // println!("Part one: {}", new.chars().zip(-2*iterations..).map(|(c, i)| match c {'.' => 0, _ => i}).sum::<i32>());

}
