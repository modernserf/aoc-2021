use std::collections::HashMap;

struct PolymerState {
    counts: HashMap<(char, char), usize>,
    end_char: char,
    rules: HashMap<(char, char), char>,
}

fn parse_input(input: &str) -> PolymerState {
    let mut sections = input.split("\n\n");
    let init = sections
        .next()
        .unwrap()
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect::<Vec<_>>();
    let end_char = init[init.len() - 1];

    let mut counts = HashMap::new();
    for i in 1..init.len() {
        counts.insert((init[i - 1], init[i]), 1);
    }

    let rules = sections
        .next()
        .unwrap()
        .split("\n")
        .map(|line| {
            let mut chars = line.chars();
            let left = chars.next().unwrap();
            let right = chars.next().unwrap();
            let added = chars.nth(4).unwrap();
            ((left, right), added)
        })
        .collect::<HashMap<_, _>>();

    PolymerState {
        counts,
        end_char,
        rules,
    }
}

fn run_generation(state: &mut PolymerState) {
    let prev = std::mem::take(&mut state.counts);

    for ((l, r), count) in prev {
        let c = *state.rules.get(&(l, r)).unwrap();
        state
            .counts
            .entry((l, c))
            .and_modify(|c| *c += count)
            .or_insert(count);
        state
            .counts
            .entry((c, r))
            .and_modify(|c| *c += count)
            .or_insert(count);
    }
}

fn count_elements(state: &PolymerState) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    map.insert(state.end_char, 1);
    for ((l, _), count) in state.counts.iter() {
        map.entry(*l).and_modify(|c| *c += *count).or_insert(*count);
    }
    map
}

fn main() {
    let input = include_str!("input.txt");
    let mut state = parse_input(input);
    for _ in 0..10 {
        run_generation(&mut state);
    }
    let counts = count_elements(&state);

    let min_count = counts.values().min().unwrap();
    let max_count = counts.values().max().unwrap();

    println!("part 1: {}", max_count - min_count);

    for _ in 10..40 {
        run_generation(&mut state);
    }
    let counts = count_elements(&state);

    let min_count = counts.values().min().unwrap();
    let max_count = counts.values().max().unwrap();

    println!("part 2: {}", max_count - min_count);
}
