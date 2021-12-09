fn main() {
    // let input = "16,1,2,0,4,2,7,1,2,14";
    let input = include_str!("input.txt");
    let positions = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let min = *positions.iter().min_by(|l, r| l.cmp(r)).unwrap();
    let max = *positions.iter().max_by(|l, r| l.cmp(r)).unwrap();

    let mut best_score = 100_000_000;

    for i in min..=max {
        let score = positions.iter().map(|x| simple_cost(*x, i)).sum::<usize>();
        if score < best_score {
            best_score = score;
        }
    }

    println!("part 1: {}", best_score);
    best_score = 100_000_000;

    for i in min..=max {
        let score = positions.iter().map(|x| complex_cost(*x, i)).sum::<usize>();
        if score < best_score {
            best_score = score;
        }
    }

    println!("part 2: {}", best_score);
}

fn simple_cost(current: usize, target: usize) -> usize {
    if target >= current {
        target - current
    } else {
        current - target
    }
}

fn complex_cost(current: usize, target: usize) -> usize {
    let d = simple_cost(current, target);
    ((d + 1) * d) / 2
}
