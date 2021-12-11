use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let mut grid = input
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut count = 0;

    for _ in 0..100 {
        let flash_count = run_generation(&mut grid);
        count += flash_count;
    }

    println!("part 1: {}", count);

    for i in 100..10000 {
        let flash_count = run_generation(&mut grid);
        if flash_count == 100 {
            println!("part 2: {}", i + 1);
            break;
        }
    }
}

fn run_generation(mut grid: &mut [Vec<usize>]) -> usize {
    let mut flashed_set = HashSet::new();

    let height = grid.len();
    let width = grid[0].len();
    // increment
    for x in 0..width {
        for y in 0..height {
            grid[y][x] += 1;
        }
    }

    loop {
        let mut any_added = false;

        for x in 0..width {
            for y in 0..height {
                if grid[y][x] > 9 {
                    if flashed_set.insert((x, y)) {
                        any_added = true;
                        update_energy(&mut grid, x as isize, y as isize);
                    }
                }
            }
        }
        if !any_added {
            break;
        }
    }

    let count = flashed_set.len();

    for (x, y) in flashed_set {
        grid[y][x] = 0;
    }

    count
}

fn update_energy(mut grid: &mut [Vec<usize>], x: isize, y: isize) {
    inc_at(&mut grid, x - 1, y - 1);
    inc_at(&mut grid, x, y - 1);
    inc_at(&mut grid, x + 1, y - 1);

    inc_at(&mut grid, x - 1, y);
    // center
    inc_at(&mut grid, x + 1, y);

    inc_at(&mut grid, x - 1, y + 1);
    inc_at(&mut grid, x, y + 1);
    inc_at(&mut grid, x + 1, y + 1);
}

fn inc_at(grid: &mut [Vec<usize>], x: isize, y: isize) {
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;
    if y >= 0 && y < height && x >= 0 && x < width {
        grid[y as usize][x as usize] += 1;
    }
}
