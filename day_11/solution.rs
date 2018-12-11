const INPUT: usize = 5177;

fn fuel(xy: (usize, usize)) -> i64 {
    let rack_id = xy.0 + 10;
    (((rack_id * xy.1 + INPUT) * rack_id / 100) % 10) as i64 - 5
}

fn sum_fuel(fuel: &Vec<Vec<i64>>, width: usize) -> Vec<Vec<i64>> {
    (0..300 - width)
        .map(|x| {
            (0..300 - width)
                .map(|y| {
                    (0..width)
                        .map(|i| {
                            (0..width)
                                .map(|j| fuel[(x + i) as usize][(y + j) as usize])
                                .sum::<i64>()
                        })
                        .sum::<i64>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn sum_fuel_using_previous(
    fuel: &Vec<Vec<i64>>,
    previous: &Vec<Vec<i64>>,
    width: usize,
) -> Vec<Vec<i64>> {
    (0..300 - width)
        .map(|x| {
            (0..300 - width)
                .map(|y| {
                    previous[x][y]
                        + (0..width).map(|i| fuel[x + i][y + width - 1]).sum::<i64>()
                        + (0..width - 1)
                            .map(|j| fuel[x + width - 1][y + j])
                            .sum::<i64>()
                })
                .collect()
        })
        .collect()
}

fn main() {
    let grid: Vec<Vec<i64>> = (1..=300)
        .map(|x| (1..=300).map(|y| fuel((x, y))).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let summed = sum_fuel(&grid, 3);
    let (p1x, (p1y, p1f)) = summed
        .iter()
        .map(move |v| v.iter().enumerate().max_by_key(move |&(_, f)| f).unwrap())
        .enumerate()
        .max_by_key(move |&(_, (_, f))| f)
        .unwrap();
    let mut maximum_by_width: Vec<(usize, usize, usize, i64)> = Vec::with_capacity(300);
    let mut current_sum = sum_fuel(&grid, 1);
    for w in 2..300 {
        current_sum = sum_fuel_using_previous(&grid, &current_sum, w);
        let (x, (y, f)) = current_sum
            .iter()
            .map(move |v| v.iter().enumerate().max_by_key(move |&(_, f)| f).unwrap())
            .enumerate()
            .max_by_key(move |&(_, (_, f))| f)
            .unwrap();
        maximum_by_width.push((w, x, y, *f));
    }
    let (p2w, p2x, p2y, p2f) = maximum_by_width
        .iter()
        .max_by_key(|(_, _, _, f)| f)
        .unwrap();
    println!("Part one x, y: {},{} with fuel: {}", p1x + 1, p1y + 1, p1f);
    println!(
        "Part two x, y: {},{} with width: {} and fuel: {}",
        p2x + 1,
        p2y + 1,
        p2w,
        p2f
    );
}
