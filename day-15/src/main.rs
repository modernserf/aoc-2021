use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn neighbors(width: usize, height: usize, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    if x > 0 {
        out.push((x - 1, y));
    }
    if y > 0 {
        out.push((x, y - 1));
    }
    if x < width - 1 {
        out.push((x + 1, y));
    }
    if y < height - 1 {
        out.push((x, y + 1));
    }
    out
}

fn get_dist(grid: &Vec<Vec<usize>>, (x, y): (usize, usize)) -> usize {
    let inner_width = grid[0].len();
    let inner_height = grid.len();

    let inner_x = x % inner_width;
    let inner_y = y % inner_height;
    let value_offset = (x / inner_width) + (y / inner_height);
    let value = grid[inner_y][inner_x] + value_offset;
    ((value - 1) % 9) + 1
}

fn dijkstra_2(grid: &Vec<Vec<usize>>, scale: usize) -> usize {
    let width = grid[0].len() * scale;
    let height = grid.len() * scale;
    let target = (width - 1, height - 1);
    let mut distances = HashMap::new();
    let default_distance = 1_000_000;
    distances.insert((0, 0), 0);
    let mut visited = HashSet::new();

    let mut current = (0, 0);
    loop {
        let current_dist = *distances.get(&current).unwrap_or(&default_distance);
        for neighbor in neighbors(width, height, current) {
            if !visited.contains(&neighbor) {
                let dist = get_dist(&grid, neighbor);
                distances
                    .entry(neighbor)
                    .and_modify(|d| *d = (*d).min(dist + current_dist))
                    .or_insert(dist + current_dist);
            }
        }
        visited.insert(current);
        if current == target {
            return *distances.get(&current).unwrap();
        } else {
            distances.remove(&current);
            let (next, _) = distances.iter().min_by(|(_, l), (_, r)| l.cmp(r)).unwrap();
            current = *next;
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let grid = parse_input(&input);
    let min_path = dijkstra_2(&grid, 1);

    println!("part 1: {}", min_path);
    let min_path = dijkstra_2(&grid, 5);

    println!("part 2: {}", min_path);
}
