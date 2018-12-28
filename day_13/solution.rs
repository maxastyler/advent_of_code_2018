use std::fmt;

#[derive(Debug, Clone)]
struct Car {
    dir: i8,
    turn: i8,
    x: usize,
    y: usize,
}

impl Car {
    fn new(_dir: i8, _x: usize, _y: usize) -> Car {
        Car {
            dir: _dir,
            turn: 0,
            x: _x,
            y: _y,
        }
    }
}

#[derive(Debug, Clone)]
enum Track {
    NE,
    NW,
    SE,
    SW,
    V,
    H,
    IN,
    B,
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Track::NE => "\\",
                Track::NW => "/",
                Track::SE => "/",
                Track::SW => "\\",
                Track::V => "|",
                Track::H => "-",
                Track::IN => "+",
                Track::B => " ",
            }
        )
    }
}

fn tick(track: &Vec<Vec<Track>>, cars: &mut Vec<Car>) {
    for car in cars {
        match car.dir {
            0 => car.x += 1,
            1 => car.y += 1,
            2 => car.x -= 1,
            _ => car.y -= 1,
        }
        match track[car.y][car.x] {
            Track::NE => match car.dir {
                3 => car.dir = 0,
                2 => car.dir = 1,
                _ => panic!("Wrong direction at turn")
            },
            Track::NW => match car.dir {
                3 => car.dir = 2,
                0 => car.dir = 1,
                _ => panic!("Wrong direction at turn")
            },
            Track::SE => match car.dir {
                1 => car.dir = 0,
                0 => car.dir = 3,
                _ => panic!("Wrong direction at turn")
            },
            Track::SW => match car.dir {
                1 => car.dir = 2,
                0 => car.dir = 3,
                _ => panic!("Wrong direction at turn")
            },
            Track::IN => {
                car.dir = (car.dir - (car.turn - 1) + 4) % 4;
                println!("{}", car.dir);
                car.turn = (car.turn + 1) % 3;
            }
            _ => {}
        }
    }
}

fn main() {
    let track_string: Vec<Vec<_>> = include_str!("./input.txt")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let mut track = vec![vec![Track::B; track_string[0].len()]; track_string.len()];
    let mut cars: Vec<Car> = vec![];
    for (y, l) in track_string.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c == '>' {
                cars.push(Car::new(0, x, y))
            } else if *c == '^' {
                cars.push(Car::new(1, x, y))
            } else if *c == '<' {
                cars.push(Car::new(2, x, y))
            } else if *c == 'v' {
                cars.push(Car::new(3, x, y))
            }
            track[y][x] = match c {
                '/' => {
                    if y == 0 {
                        match track_string[y + 1][x] {
                            '|' => Track::SE,
                            _ => Track::NW,
                        }
                    } else {
                        match track_string[y - 1][x] {
                            '|' => Track::NW,
                            _ => Track::SE,
                        }
                    }
                }
                '\\' => {
                    if y == 0 {
                        match track_string[y + 1][x] {
                            '|' => Track::SW,
                            _ => Track::NE,
                        }
                    } else {
                        match track_string[y - 1][x] {
                            '|' => Track::NE,
                            _ => Track::SW,
                        }
                    }
                }
                '|' | 'v' | '^' => Track::V,
                '-' | '<' | '>' => Track::H,
                '+' => Track::IN,
                _ => Track::B,
            }
        }
    }
    let track = track;
    'sim_loop: for t in 0.. {
        println!("{}", t);
        tick(&track, &mut cars);
        for (i, car_1) in cars.iter().enumerate() {
            for (_, car_2) in cars.iter().skip(i+1).enumerate() {
                if car_1.x == car_2.x && car_1.y == car_2.y {
                    println!("{}, {}, tick: {}", car_1.x, car_2.y, t);
                    break 'sim_loop;
                }
            }
        }
    }
}
