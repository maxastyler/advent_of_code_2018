extern crate regex;
#[macro_use] extern crate bitvec;

use regex::Regex;
use std::collections::HashMap;
use bitvec::{BitVec, BigEndian};

fn main() {
    let re = Regex::new(r"#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();
    let input = include_str!("/home/max/git/advent_of_code_2018/day_3/input.txt")
        .lines()
        .map(|l| {
            let c = re.captures(l).unwrap();
            (c[1].parse::<usize>().unwrap(),
             (c[2].parse::<u16>().unwrap(), c[3].parse::<u16>().unwrap()),
             (c[4].parse::<u16>().unwrap(), c[5].parse::<u16>().unwrap()))
        })
        .collect::<Vec<_>>();

    let mut fabric: HashMap<(u16, u16), BitVec> = HashMap::with_capacity(1000*1000);
    (0..1000).for_each(|x| (0..1000).for_each(|y| {fabric.insert((x, y), bitvec![0; 1233]); }));
    input.iter().for_each( |(i, (x, y), (w, h))| {
        (*x..(x+w)).for_each( |x| (*y..(y+h)).for_each( |y| {fabric.get_mut(&(x, y)).unwrap().set(*i-1, true);}));
    });

    println!("Squares covered by 2 or more: {}", fabric.values().filter(|v| v.count_ones()>=2).count());

    let mut id_vec = bitvec![1; 1233];

    fabric.values().filter_map(|v| if v.count_ones()>=2 {
        Some(v.iter().enumerate().filter_map(|(i, v)| if v==true { Some(i) } else { None }))
    } else { None })
        .flatten().for_each(|x| id_vec.set(x, false));
    println!("Non-overlapping indices: {:?}", id_vec.iter().enumerate().filter_map(|(i, v)| if v==true { Some(i+1) } else { None }).collect::<Vec<_>>());
}
