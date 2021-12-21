use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(PartialEq, Debug, Clone)]
struct Counter<K: Eq + Hash>(HashMap<K, u64>);

impl<K: Eq + Hash> Counter<K> {
    fn new() -> Self {
        Counter(HashMap::new())
    }
    fn add(&mut self, key: K, value: u64) {
        self.0
            .entry(key)
            .and_modify(|c| *c += value)
            .or_insert(value);
    }
    fn iter(&self) -> Iter<K, u64> {
        self.0.iter()
    }
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(position: usize) -> Self {
        Player { position, score: 0 }
    }
    fn advance(&mut self, offset: usize) -> usize {
        self.position = ((self.position + offset - 1) % 10) + 1;
        self.score += self.position;
        self.score
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Game {
    a: Player,
    b: Player,
}

#[derive(PartialEq, Debug, Clone)]
struct Multiverse {
    universes: Counter<Game>,
    a_wins: u64,
    b_wins: u64,
}

impl Multiverse {
    fn new() -> Self {
        Multiverse {
            universes: Counter::new(),
            a_wins: 0,
            b_wins: 0,
        }
    }
    fn new_game(a_pos: usize, b_pos: usize) -> Self {
        let a = Player::new(a_pos);
        let b = Player::new(b_pos);
        let mut out = Multiverse::new();
        let game = Game { a, b };
        out.universes.add(game, 1);
        out
    }
    fn round(&mut self) -> bool {
        let mut next_multiverse = Counter::new();
        // player 1 turn
        for (game, count) in self.universes.iter() {
            for a in 1..=3 {
                for b in 1..=3 {
                    for c in 1..=3 {
                        let roll = (a + b + c) as usize;
                        let mut next_game = game.clone();
                        next_game.a.advance(roll);
                        if next_game.a.score >= 21 {
                            self.a_wins += count
                        } else {
                            next_multiverse.add(next_game, *count);
                        }
                    }
                }
            }
        }
        self.universes = next_multiverse;
        if self.universes.len() == 0 {
            return false;
        }

        let mut next_multiverse = Counter::new();
        // player 2 turn
        for (game, count) in self.universes.iter() {
            for a in 1..=3 {
                for b in 1..=3 {
                    for c in 1..=3 {
                        let roll = (a + b + c) as usize;
                        let mut next_game = game.clone();
                        next_game.b.advance(roll);
                        if next_game.b.score >= 21 {
                            self.b_wins += count
                        } else {
                            next_multiverse.add(next_game, *count);
                        }
                    }
                }
            }
        }
        self.universes = next_multiverse;
        self.universes.len() > 0
    }
}

fn main() {
    // let mut a = Player::new(4);
    // let mut b = Player::new(8);
    let mut a = Player::new(1);
    let mut b = Player::new(2);

    for i in 0.. {
        let base = i * 6;
        if a.advance((base * 3) + 1 + 2 + 3) >= 1000 {
            let roll_count = base + 3;
            println!("part 1: {}", roll_count * b.score);
            break;
        }
        if b.advance((base * 3) + 4 + 5 + 6) >= 1000 {
            let roll_count = base + 6;
            println!("part 1: {}", roll_count * a.score);
            break;
        }
    }

    let mut m = Multiverse::new_game(1, 2);
    while m.round() {}
    println!("part 2: {} {}", m.a_wins, m.b_wins);
}
