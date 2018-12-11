use std::iter;

const INPUT: usize = 5177;

fn fuel(xy: (usize, usize)) -> i64 {
    let rack_id = xy.0 + 10;
    (((rack_id * xy.1 + INPUT) * rack_id / 100) % 10) as i64 - 5
}

fn main() {
    let grid: Vec<Vec<i64>> = (1..=300)
        .map(|x| (1..=300).map(|y| fuel((x, y))).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let max_coords = (1..=20).map(|w| {
        (
            w,
            (0..300 - w)
                .map(|x| iter::repeat(x).zip(0..300 - w))
                .flatten()
                .map(|(x, y)| {
                    (
                        (x, y),
                        (0..w)
                            .map(|i| {
                                (0..w)
                                    .map(|j| grid[(x + i) as usize][(y + j) as usize])
                                    .sum::<i64>()
                            })
                            .sum::<i64>(),
                    )
                })
                .max_by_key(|&(_, f)| f)
                .unwrap(),
        )
    }).max_by_key(|&(_, (_, f))| f).unwrap();
    println!("{},{},{}", ((max_coords.1).0).0 + 1, ((max_coords.1).0).1 + 1, max_coords.0);
}
