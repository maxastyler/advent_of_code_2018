fn main() {
    let coords = include_str!("./input.txt")
        .lines()
        .map(|x| {
            let mut split = x.split(", ");
            (
                split.next().unwrap().parse::<i16>().unwrap(),
                split.next().unwrap().parse::<i16>().unwrap(),
            )
        })
        .enumerate()
        .collect::<Vec<_>>();
    let min_x = coords.iter().map(|(_, x)| x.0).min().unwrap();
    let min_y = coords.iter().map(|(_, x)| x.1).min().unwrap();
    let max_x = coords.iter().map(|(_, x)| x.0).max().unwrap();
    let max_y = coords.iter().map(|(_, x)| x.1).max().unwrap();

    let mut areas: Vec<u16> = vec![0; coords.len()];
    let mut safe_region_size: u32 = 0;
    for x in (min_x - 1)..(max_x + 1) {
        for y in (min_y - 1)..(max_y + 1) {
            let mut distances = coords
                .iter()
                .map(|(i, (px, py))| (i, (x - px).abs() + (y - py).abs()))
                .collect::<Vec<_>>();
            distances.sort_by_key(|&(_, d)| d);
            match distances
                .iter()
                .take_while(|&(_, d)| *d == distances[0].1)
                .collect::<Vec<_>>()
            {
                ref distances if distances.len() == 1 => {
                    if ((x < min_x) | (x > max_x)) | ((y < min_y) | (y > min_y)) {
                        if areas[*distances[0].0] != std::u16::MAX {
                            areas[*distances[0].0] += 1;
                        }
                    } else {
                        areas[*distances[0].0] = std::u16::MAX;
                    }
                }
                _ => (),
            };

            if distances.iter().map(|(_, d)| *d as u32).sum::<u32>() < 10000 {
                safe_region_size += 1;
            }
        }
    }
    println!("Part 1: {}", areas.iter().filter(|&x| *x < std::u16::MAX).max().unwrap());
    println!("Part 2: {}", safe_region_size);
}
