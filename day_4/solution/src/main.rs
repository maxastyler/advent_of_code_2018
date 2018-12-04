extern crate regex;

use std::collections::HashMap;

use regex::{Regex, RegexSet};

fn main() {

    // PARSING STUFF

    let mut input = include_str!("/home/max/git/advent_of_code_2018/day_4/input.txt")
        .lines()
        .collect::<Vec<_>>();
    &input[..].sort();
    let regex_set = RegexSet::new(&[
        r"\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}\] Guard #\d+ begins shift",
        r"\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}\] falls asleep",
        r"\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}\] wakes up",
    ])
    .unwrap();
    let id_regex =
        Regex::new(r"\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}\] Guard #(\d+) begins shift").unwrap();
    let time_regex = Regex::new(r"\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})\] [[:ascii:]]").unwrap();
    let mut elves: HashMap<u16, Vec<u8>> = HashMap::new();
    let mut current: u16 = 0;
    let mut time: u8 = 0;

    input
        .iter()
        .for_each(|x| match regex_set.matches(x).into_iter().next().unwrap() {
            0 => {
                current = id_regex.captures(x).unwrap()[1].parse::<u16>().unwrap();
                if !elves.contains_key(&current) {
                    elves.insert(current, vec![0; 60]);
                }
            }
            1 => {
                time = time_regex.captures(x).unwrap()[1].parse::<u8>().unwrap();
            }
            _ => {
                let current_times = elves.get_mut(&current).unwrap();
                (time..time_regex.captures(x).unwrap()[1].parse::<u8>().unwrap())
                    .for_each(|i| current_times[i as usize] += 1)
            }
        });

    // FINDING THE SLEEPY ELVES LOGIC

    let (elf_id, elf_sleepy_minutes) = elves
        .iter()
        .max_by_key(|(_, x)| x.iter().fold(0, |a, &i| a + (i as u16)))
        .unwrap();

    let sleepiest_minute = elf_sleepy_minutes
        .iter()
        .enumerate()
        .max_by_key(|v| v.1)
        .unwrap()
        .0;
    println!(
        "The sleepiest elf is: {}\nThe minute that the elf is most likely asleep on is: {}\nChecksum is: {}",
        elf_id,
        sleepiest_minute,
        (*elf_id as usize)*sleepiest_minute,
    );

    let (guard, (minute, _)) = elves
        .iter()
        .map(|(a, b)| (a, b.iter().enumerate().max_by_key(|(_, &i)| i).unwrap()))
        .max_by_key(|(_, (_, &i))| i)
        .unwrap();
    println!(
        "Sleepiest minute elf is {}, and the sleepiest minute is {}, the checksum is {}",
        guard,
        minute,
        (*guard as usize) * minute
    );
}
