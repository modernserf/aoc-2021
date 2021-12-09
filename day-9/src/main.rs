use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");
    let grid = input
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let row_min = 0;
    let row_max = grid.len() - 1;
    let col_min = 0;
    let col_max = grid[0].len() - 1;

    let mut low_points = Vec::new();

    for row in row_min..=row_max {
        for col in col_min..=col_max {
            let height = grid[row][col];
            if (row > row_min) && (grid[row - 1][col] <= height) {
                continue;
            }
            if (row < row_max) && (grid[row + 1][col] <= height) {
                continue;
            }
            if (col > col_min) && (grid[row][col - 1] <= height) {
                continue;
            }
            if (col < col_max) && (grid[row][col + 1] <= height) {
                continue;
            }

            low_points.push((row, col));
        }
    }

    let risk_sum = low_points
        .iter()
        .map(|(row, col)| grid[*row][*col] + 1)
        .sum::<usize>();

    println!("part 1: {}", risk_sum);

    let mut basins = low_points
        .iter()
        .map(|pt| get_basin(&grid, *pt))
        .collect::<Vec<_>>();

    basins.sort_by(|l, r| r.cmp(l));

    let top_three = basins[0] * basins[1] * basins[2];
    println!("part 2: {}", top_three);
}

fn get_basin(grid: &[Vec<usize>], point: (usize, usize)) -> usize {
    let row_min = 0;
    let row_max = grid.len() - 1;
    let col_min = 0;
    let col_max = grid[0].len() - 1;
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from(vec![point]);
    let max_height = 9;
    while let Some(item) = queue.pop_front() {
        if visited.contains(&item) {
            continue;
        }
        visited.insert(item);
        let (row, col) = item;
        if (row > row_min) && (grid[row - 1][col] < max_height) {
            queue.push_back((row - 1, col));
        }
        if (row < row_max) && (grid[row + 1][col] < max_height) {
            queue.push_back((row + 1, col));
        }
        if (col > col_min) && (grid[row][col - 1] < max_height) {
            queue.push_back((row, col - 1));
        }
        if (col < col_max) && (grid[row][col + 1] < max_height) {
            queue.push_back((row, col + 1));
        }
    }

    visited.len()
}
