fn distance_sq(p1: &Vec<i64>, p2: &Vec<i64>) -> i64 {
    (p1[0] - p2[0]).pow(2) + (p1[1] - p2[1]).pow(2)
}

fn average_distance_sq(p: &Vec<Vec<i64>>) -> i64 {
    let mut total: i64 = 0;
    for i in 0..p.len() {
        let mut i_total = 0;
        let mut others = (0..p.len()).collect::<Vec<_>>();
        others.remove(i);
        for j in others {
            i_total += distance_sq(&p[i], &p[j]);
        }
        total += i_total / (p.len() as i64 - 1);
    }
    total / (p.len() as i64)
}

fn extract_pos(pv: &Vec<Vec<Vec<i64>>>) -> Vec<Vec<i64>> {
    pv.iter().map(|l| l[0].clone()).collect()
}

fn iterate(pv: &mut Vec<Vec<Vec<i64>>>) {
    for i in pv.iter_mut() {
        i[0][0] += i[1][0];
        i[0][1] += i[1][1];
    }
}

fn display(p: &Vec<Vec<i64>>) {
    // . = 0x2e, # = 0x23
    let min_x = p.iter().min_by_key(|v| v[0]).unwrap()[0];
    let max_x = p.iter().max_by_key(|v| v[0]).unwrap()[0];
    let min_y = p.iter().min_by_key(|v| v[1]).unwrap()[1];
    let max_y = p.iter().max_by_key(|v| v[1]).unwrap()[1];
    let n_lines = (max_y - min_y) as usize;
    let width = (max_x - min_x) as usize;
    let mut str: Vec<Vec<u8>> = vec![vec![0x2e; width + 1]; n_lines + 1];
    for point in p {
        str[(point[1] - min_y) as usize][(point[0] - min_x) as usize] = 0x23;
    }
    let f = str.iter().map(|x| String::from_utf8(x.clone()).unwrap());
    for l in f {
        println!("{}", l);
    }
}

fn main() {
    let mut pos_vel = include_str!("./input.txt")
        .lines()
        .map(|x| &x[10..])
        .map(|x| x.trim_matches(|c| c == '>'))
        .map(|x| {
            x.split("> velocity=<")
                .map(|c| {
                    c.split(", ")
                        .map(|i| i.trim().parse::<i64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut old_p: (Vec<Vec<i64>>, i64) = (
        extract_pos(&pos_vel),
        average_distance_sq(&extract_pos(&pos_vel)),
    );
    let mut new_p;
    let mut time = 0;
    loop {
        time += 1;
        iterate(&mut pos_vel);
        new_p = (
            extract_pos(&pos_vel),
            average_distance_sq(&extract_pos(&pos_vel)),
        );
        if new_p.1 > old_p.1 {
            println!("New positions are further away than the old ones");
            break;
        }
        println!("{}, {}", new_p.1, old_p.1);
        old_p = new_p.clone();
    }
    display(&old_p.0);
    println!("Took: {} seconds", time - 1);
}
