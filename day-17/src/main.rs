#[derive(PartialEq, Debug, Clone, Copy)]
struct Target {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

fn max_dy(y_min: isize) -> isize {
    (y_min * -1) - 1
}

fn apex(target: &Target) -> isize {
    let mut dy = max_dy(target.y_min);
    let mut y = 0;
    let mut apex = 0;
    for _ in 0.. {
        y += dy;
        dy = dy - 1;
        if y > apex {
            apex = y;
        } else {
            return apex;
        }
    }
    unreachable!();
}

fn min_dx(min_x: isize) -> isize {
    for init_dx in 0.. {
        let mut dx = init_dx;
        let mut x = 0;
        for _ in 1.. {
            let next_x = x + dx;
            if x >= min_x {
                return init_dx;
            }
            if next_x == x {
                break;
            }
            x = next_x;
            dx -= 1;
        }
    }
    0
}

fn count_trajectories(target: &Target) -> usize {
    let mut count = 0;
    let min_dy = target.y_min - 1;
    let max_dy = max_dy(target.y_min);
    let min_dx = min_dx(target.x_min);
    let max_dx = target.x_max + 1;

    for init_dy in min_dy..=max_dy {
        for init_dx in min_dx..=max_dx {
            let mut x = 0;
            let mut y = 0;
            let mut dx = init_dx;
            let mut dy = init_dy;
            for _ in 0.. {
                x += dx;
                y += dy;
                dx = 0.max(dx - 1);
                dy = dy - 1;

                if y < target.y_min {
                    break;
                }

                if target.x_min <= x && x <= target.x_max && y <= target.y_max {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}

fn main() {
    let test_input = Target {
        x_min: 20,
        x_max: 30,
        y_min: -10,
        y_max: -5,
    };
    let input = Target {
        x_min: 70,
        x_max: 96,
        y_min: -179,
        y_max: -124,
    };

    println!("part 1: {}", apex(&input));

    println!("part 2 test: {}", count_trajectories(&test_input));
    println!("part 2: {}", count_trajectories(&input));
}
