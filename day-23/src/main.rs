use std::collections::HashMap;

/*
#############
#0123456789X#
##0.#.#.#.###
  1.#.#.#.#
  2.#.#.#.#
  3.#.#.#.#
  #A#B#C#D#
*/

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Color {
    A,
    B,
    C,
    D,
}

impl Color {
    fn hall_position(&self) -> usize {
        match self {
            Color::A => 2,
            Color::B => 4,
            Color::C => 6,
            Color::D => 8,
        }
    }
    fn move_cost(&self) -> usize {
        match self {
            Color::A => 1,
            Color::B => 10,
            Color::C => 100,
            Color::D => 1000,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Shrimp(Color, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Slot {
    Hallway(usize),
    Column(Color, usize),
}

struct State {
    positions: HashMap<Shrimp, Slot>,
    cost: usize,
}

impl State {
    fn new(cols: Vec<(Color, Vec<Shrimp>)>) -> Self {
        let mut positions = HashMap::new();

        for (col_color, col) in cols {
            for (idx, shrimp) in col.into_iter().enumerate() {
                positions.insert(shrimp, Slot::Column(col_color, idx));
            }
        }

        State { positions, cost: 0 }
    }

    fn assert_slot_free(&self, slot: Slot) {
        for (_, item) in self.positions.iter() {
            if slot == *item {
                panic!("collision")
            }
        }
    }

    fn mov(&mut self, shrimp: Shrimp, target: Slot) {
        let current_pos = *self.positions.get(&shrimp).unwrap();
        if current_pos == target {
            return;
        }

        let next_pos = match (current_pos, target) {
            (Slot::Column(from_col, from_idx), Slot::Column(to_col, _)) if from_col == to_col => {
                // move deeper into column
                Slot::Column(from_col, from_idx + 1)
            }
            (Slot::Column(from_col, from_idx), _) if from_idx == 0 => {
                // move into hallway
                Slot::Hallway(from_col.hall_position())
            }
            (Slot::Column(from_col, from_idx), _) => {
                // move towards hallway
                Slot::Column(from_col, from_idx - 1)
            }
            (Slot::Hallway(from_pos), Slot::Column(to_col, _)) => {
                let to_entrance = to_col.hall_position();
                if from_pos == to_entrance {
                    Slot::Column(to_col, 0)
                } else if from_pos < to_entrance {
                    Slot::Hallway(from_pos + 1)
                } else {
                    Slot::Hallway(from_pos - 1)
                }
            }
            (Slot::Hallway(from_pos), Slot::Hallway(to_pos)) => {
                if from_pos < to_pos {
                    Slot::Hallway(from_pos + 1)
                } else {
                    Slot::Hallway(from_pos - 1)
                }
            }
        };
        println!("move {:?} to {:?}", shrimp, next_pos);
        self.assert_slot_free(next_pos);
        self.cost += shrimp.0.move_cost();
        self.positions.insert(shrimp, next_pos);
        return self.mov(shrimp, target);
    }

    fn final_cost(&self) -> usize {
        for (shrimp @ Shrimp(color, _), slot) in self.positions.iter() {
            match slot {
                Slot::Column(col_color, _) if color == col_color => {
                    println!("{:?} in {:?}", shrimp, slot)
                }
                _ => {
                    panic!("error: {:?} in {:?}", shrimp, slot)
                }
            }
        }
        self.cost
    }
}

fn main() {
    use Color::*;
    use Slot::*;

    let mut state = State::new(vec![
        (
            A,
            vec![Shrimp(A, 0), Shrimp(D, 0), Shrimp(D, 1), Shrimp(C, 0)],
        ),
        (
            B,
            vec![Shrimp(D, 2), Shrimp(C, 1), Shrimp(B, 0), Shrimp(D, 3)],
        ),
        (
            C,
            vec![Shrimp(C, 2), Shrimp(B, 1), Shrimp(A, 1), Shrimp(B, 2)],
        ),
        (
            D,
            vec![Shrimp(A, 2), Shrimp(A, 3), Shrimp(C, 3), Shrimp(B, 3)],
        ),
    ]);
    state.mov(Shrimp(A, 0), Hallway(1));
    state.mov(Shrimp(D, 0), Hallway(10));
    state.mov(Shrimp(D, 1), Hallway(9));
    state.mov(Shrimp(C, 0), Hallway(3));
    state.mov(Shrimp(A, 0), Column(A, 3));
    state.mov(Shrimp(C, 0), Hallway(0));
    state.mov(Shrimp(A, 2), Column(A, 2));
    state.mov(Shrimp(A, 3), Column(A, 1));
    state.mov(Shrimp(C, 3), Hallway(5));
    state.mov(Shrimp(B, 3), Hallway(7));
    state.mov(Shrimp(D, 1), Column(D, 3));
    state.mov(Shrimp(D, 0), Column(D, 2));
    state.mov(Shrimp(B, 3), Hallway(10));
    state.mov(Shrimp(C, 3), Hallway(9));
    state.mov(Shrimp(D, 2), Column(D, 1));
    state.mov(Shrimp(C, 1), Hallway(1));
    state.mov(Shrimp(B, 0), Hallway(3));
    state.mov(Shrimp(D, 3), Column(D, 0));
    state.mov(Shrimp(C, 2), Hallway(7));
    state.mov(Shrimp(B, 0), Column(B, 3));
    state.mov(Shrimp(B, 1), Column(B, 2));
    state.mov(Shrimp(A, 1), Column(A, 0));
    state.mov(Shrimp(B, 2), Column(B, 1));
    state.mov(Shrimp(C, 2), Column(C, 3));
    state.mov(Shrimp(C, 2), Column(C, 3));
    state.mov(Shrimp(C, 1), Column(C, 2));
    state.mov(Shrimp(C, 0), Column(C, 1));
    state.mov(Shrimp(C, 3), Column(C, 0));
    state.mov(Shrimp(B, 3), Column(B, 0));
    // 51425 = "too low"
    println!("part 2 {}", state.final_cost());
}
