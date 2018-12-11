use std::ops::Range;
use std::collections::VecDeque;

struct MarbleGame {
    player_count: usize,
    current_player: usize,
    highest_marble: usize,
    current_marbles: Range<usize>,
    current_circle: Vec<usize>,
    current_index: usize,
    scores: Vec<usize>,
}

impl MarbleGame {
    fn new(player_count: usize, highest_marble: usize) -> MarbleGame {
        MarbleGame {
            player_count: player_count,
            current_player: 0,
            highest_marble: highest_marble,
            current_marbles: (0..(highest_marble + 1)),
            current_circle: Vec::with_capacity(highest_marble + 1),
            current_index: 0,
            scores: vec![0; player_count],
        }
    }

    fn iterate(&mut self) -> Option<usize> {
        match self.current_marbles.next() {
            None => Some(*self.scores.iter().max().unwrap()),
            Some(i) => {
                match i {
                    0 => {
                        self.current_circle.insert(0, i);
                    }
                    j if i % 23 == 0 => {
                        let other_index = (((self.current_index as i64 - 7) % self.current_circle.len() as i64 + self.current_circle.len() as i64) as usize) % self.current_circle.len();
                        self.scores[self.current_player] += i + self.current_circle[other_index];
                        self.current_circle.remove(other_index);
                        self.current_index = (other_index) % self.current_circle.len();
                    }
                    _ => {
                        self.current_index = (self.current_index + 2) % self.current_circle.len();
                        self.current_circle.insert(self.current_index, i);
                    }
                }
                self.current_player = (self.current_player + 1) % self.player_count;
                None
            }
        }
    }
}

fn main() {
    let split_input = include_str!("input.txt")
        .split(" ")
        .collect::<Vec<_>>();
    let (players, high_marble): (usize, usize) = (
        split_input[0].parse().unwrap(),
        split_input[6].parse().unwrap(),
    );
    let mut game = MarbleGame::new(players, high_marble);
    loop {
        match game.iterate() {
            Some(score) => {println!("Winning score for game 1 was: {}", score); break},
            None => (),
        }
    }
    let mut player_scores = vec![0; players];
    let mut game: VecDeque<usize> = VecDeque::with_capacity(high_marble*100);
    game.push_back(0);
    for i in 1..(high_marble*100+1) {
        if i % 23 == 0 {
            for _ in 0..7 {
                let tmp = game.pop_back().unwrap();
                game.push_front(tmp);
            }
            player_scores[i % players] += game.pop_front().unwrap() + i;
        } else {
            for _ in 0..2 {
                let tmp = game.pop_front().unwrap();
                game.push_back(tmp);
            }
            game.push_front(i);
        }
    }
    println!("Winning score for game 2 was: {}", player_scores.iter().max().unwrap());
}
