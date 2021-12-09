use std::collections::HashMap;

fn main() {
    // let input = "3,4,3,1,2";
    let input = include_str!("input.txt");
    let mut population = HashMap::new();

    for s in input.split(",") {
        let age = s.parse::<usize>().unwrap();
        increment_count(&mut population, age, 1)
    }

    for _ in 0..80 {
        generation(&mut population);
    }
    println!("part 1: {}", total_pop(&population));

    for _ in 80..256 {
        generation(&mut population);
    }

    println!("part 2: {}", total_pop(&population));
}

fn generation(mut population: &mut HashMap<usize, u64>) {
    let mut spawn_count = 0;

    let mut next_map = HashMap::new();

    for (age, count) in population.iter() {
        if *age == 0 {
            spawn_count += *count;
            increment_count(&mut next_map, 6, *count);
        } else {
            increment_count(&mut next_map, *age - 1, *count);
        }
    }
    population.clear();
    for (k, v) in next_map.iter() {
        population.insert(*k, *v);
    }

    if spawn_count > 0 {
        increment_count(&mut population, 8, spawn_count);
    }
}

fn increment_count(map: &mut HashMap<usize, u64>, key: usize, value: u64) {
    map.entry(key)
        .and_modify(|count| *count += value)
        .or_insert(value);
}

fn total_pop(map: &HashMap<usize, u64>) -> u64 {
    map.values().sum::<u64>()
}
