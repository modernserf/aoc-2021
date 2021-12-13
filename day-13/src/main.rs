use std::collections::HashSet;

type Grid = HashSet<(usize, usize)>;

#[derive(Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize),
}

fn parse(s: &str) -> (Grid, Vec<Fold>) {
    let mut sections = s.split("\n\n");
    let grid = sections
        .next()
        .unwrap()
        .split("\n")
        .map(|line| {
            let mut coords = line.split(",");
            let x = coords.next().unwrap().parse::<usize>().unwrap();
            let y = coords.next().unwrap().parse::<usize>().unwrap();
            (x, y)
        })
        .collect::<Grid>();
    let folds = sections
        .next()
        .unwrap()
        .split("\n")
        .map(|line| {
            let trimmed = &line["fold along ".len()..];
            let mut fold = trimmed.split("=");
            let axis = fold.next().unwrap().chars().next().unwrap();
            let value = fold.next().unwrap().parse::<usize>().unwrap();
            match axis {
                'x' => Fold::X(value),
                'y' => Fold::Y(value),
                _ => panic!("invalid axis"),
            }
        })
        .collect::<Vec<_>>();

    (grid, folds)
}

fn count_dots(grid: &Grid) -> usize {
    grid.len()
}

fn fold_grid(grid: &Grid, fold: Fold) -> Grid {
    let mut next_grid = HashSet::new();

    for (x, y) in grid {
        let x = if let Fold::X(fold_x) = fold {
            if *x > fold_x {
                fold_x - (*x - fold_x)
            } else {
                *x
            }
        } else {
            *x
        };
        let y = if let Fold::Y(fold_y) = fold {
            if *y > fold_y {
                fold_y - (*y - fold_y)
            } else {
                *y
            }
        } else {
            *y
        };
        next_grid.insert((x, y));
    }

    next_grid
}

fn print_grid(grid: &Grid) {
    println!("part 2:");

    let (width, height) = grid.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
        (max_x.max(*x), max_y.max(*y))
    });
    for y in 0..=height {
        for x in 0..=width {
            if grid.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (grid, folds) = parse(&input);
    let first_fold = fold_grid(&grid, folds[0]);
    let count = count_dots(&first_fold);

    println!("part 1: {}", count);

    let final_grid = folds
        .iter()
        .fold(grid, |prev_grid, fold| fold_grid(&prev_grid, *fold));

    print_grid(&final_grid);
}
