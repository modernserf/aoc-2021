use std::collections::HashMap;
use std::ops::RangeInclusive;

type Coord = (usize, usize);

fn main() {
    let input = include_str!("input.txt");
    let mut map: HashMap<Coord, usize> = HashMap::new();

    let pairs = input
        .split("\n")
        .map(|line| {
            let coords = line.split(" -> ").collect::<Vec<_>>();
            let from = parse_pair(&coords[0]);
            let to = parse_pair(&coords[1]);
            (from, to)
        })
        .collect::<Vec<_>>();

    for (from, to) in pairs.iter() {
        fill_line(&mut map, *from, *to);
    }

    let straight_intersections = map.values().filter(|val| **val > 1).count();

    println!("part 1: {}", straight_intersections);

    for (from, to) in pairs {
        fill_diagonal(&mut map, from, to);
    }

    let all_intersections = map.values().filter(|val| **val > 1).count();

    // for y in 0..10 {
    //     for x in 0..10 {
    //         match map.get(&(x, y)) {
    //             None => {
    //                 print!(".")
    //             }
    //             Some(t) => {
    //                 print!("{}", *t)
    //             }
    //         }
    //     }
    //     print!("\n");
    // }

    println!("part 2: {}", all_intersections);
}

fn parse_pair(s: &str) -> Coord {
    let pair = s.split(",").collect::<Vec<_>>();
    (
        pair[0].parse::<usize>().unwrap(),
        pair[1].parse::<usize>().unwrap(),
    )
}

fn fill_line(mut map: &mut HashMap<Coord, usize>, from: Coord, to: Coord) {
    if from.0 == to.0 {
        for y in abs_range(from.1, to.1) {
            increment_count(&mut map, (from.0, y))
        }
    } else if from.1 == to.1 {
        for x in abs_range(from.0, to.0) {
            increment_count(&mut map, (x, from.1))
        }
    }
}

fn fill_diagonal(mut map: &mut HashMap<Coord, usize>, from: Coord, to: Coord) {
    if from.0 == to.0 || from.1 == to.1 {
        return;
    }

    let distance = (from.0 as isize - to.0 as isize).abs() as usize;
    let x_delta: isize = if from.0 <= to.0 { 1 } else { -1 };
    let y_delta: isize = if from.1 <= to.1 { 1 } else { -1 };
    let mut x = from.0 as isize;
    let mut y = from.1 as isize;

    for _ in 0..=distance {
        increment_count(&mut map, (x as usize, y as usize));
        x += x_delta;
        y += y_delta;
    }
}

fn increment_count(map: &mut HashMap<Coord, usize>, at: Coord) {
    map.entry(at).and_modify(|count| *count += 1).or_insert(1);
}

fn abs_range(a: usize, b: usize) -> RangeInclusive<usize> {
    if a <= b {
        a..=b
    } else {
        b..=a
    }
}
