use std::collections::HashMap;

const NUM_WORKERS: usize = 5;
const ADDITIONAL_TIME: i32 = 60;

fn time(c: char) -> i32 {
    c as i32 - ('A' as i32) + 1 + ADDITIONAL_TIME
}
struct TaskManager {
    open_tasks: HashMap<char, i32>,
    dependencies: HashMap<char, Vec<char>>,
    completed_order: String,
    time_taken: i32,
    scheduled: HashMap<char, i32>,
}

impl TaskManager {
    fn new(open: HashMap<char, i32>, depend: HashMap<char, Vec<char>>) -> TaskManager {
        TaskManager {
            open_tasks: open,
            scheduled: HashMap::with_capacity(NUM_WORKERS),
            dependencies: depend,
            completed_order: String::new(),
            time_taken: 0,
        }
    }

    fn iterate(&mut self) -> Option<i32> {
        self.time_taken += 1;

        // Check if there's any spots to schedule in
        let workers_open = NUM_WORKERS - self.scheduled.len();
        if workers_open > 0 {
            self.open_tasks
                .clone()
                .iter()
                .filter_map(|(k, &b)| if b == 0 { Some((k, time(*k))) } else { None })
                .take(workers_open)
                .for_each(|(k, t)| {
                    self.scheduled.insert(*k, t);
                    self.open_tasks.remove(k);
                });
        }

        let mut finished: Vec<char> = vec![];
        // Take time from scheduled
        for (c, t) in self.scheduled.iter_mut() {
            *t -= 1;
            if *t <= 0 {
                finished.push(*c);
            }
        }

        // Remove finished tasks from schedule
        for c in finished {
            for unblocked in self.dependencies.get(&c).unwrap() {
                *self.open_tasks.get_mut(unblocked).unwrap() -= 1;
            }
            self.scheduled.remove(&c);
            self.completed_order.push(c);
        }

        if (self.open_tasks.len() == 0) & (self.scheduled.len() == 0) {
            return Some(self.time_taken);
        } else {
            return None;
        }
    }
}

fn main() {
    let mut depends: HashMap<char, Vec<char>> = HashMap::new();
    include_str!("./input.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .for_each(|c| {
            depends.entry(c[5]).or_insert(vec![]).push(c[36]);
            depends.entry(c[36]).or_insert(vec![]);
        });
    let mut num_blocking: HashMap<char, i32> = HashMap::new();
    depends.keys().for_each(|&k| {
        num_blocking.insert(
            k,
            depends
                .values()
                .filter(|a| a.contains(&k))
                .collect::<Vec<_>>()
                .len() as i32,
        );
    });

    let mut task_manager = TaskManager::new(num_blocking.clone(), depends.clone());

    let mut ordered: String = String::new();

    while let Some(&c) = num_blocking
        .iter()
        .filter_map(|(k, &v)| if v == 0 { Some(k) } else { None })
        .min()
    {
        ordered.push(c);
        if let Some(b) = num_blocking.get_mut(&c) {
            *b -= 1;
        }
        depends.get_mut(&c).unwrap().iter().for_each(|blocked| {
            if let Some(b) = num_blocking.get_mut(blocked) {
                *b -= 1
            }
        });
    }

    loop {
        println!("{:?}", task_manager.scheduled);
        if let Some(t) = task_manager.iterate() {
            println!("Time taken: {}", t);
            break;
        }
    }

    println!("{}", task_manager.completed_order);
    println!("{}", ordered);
}
