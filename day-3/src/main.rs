fn main() {
    let input = include_str!("input.txt");
    let values = input
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '0' => 0,
                    '1' => 1,
                    _ => {
                        panic!("invalid char")
                    }
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    part_1(&values);
    part_2(&values);
}

fn part_1(values: &[Vec<usize>]) {
    let size = values[0].len();
    let mut counter = vec![0 as usize; size];
    for row in values {
        for i in 0..size {
            counter[i] += row[i];
        }
    }
    let mut gamma = vec![0 as usize; size];
    let mut epsilon = vec![0 as usize; size];
    let half_length = values.len() >> 1;
    for i in 0..size {
        if counter[i] > half_length {
            gamma[i] = 1;
            epsilon[i] = 0;
        } else {
            gamma[i] = 0;
            epsilon[i] = 1;
        }
    }
    println!("part 1: {}", to_int(&gamma) * to_int(&epsilon));
}

fn part_2(values: &[Vec<usize>]) {
    let oxygen_rating = get_rating(values, true);
    let co2_rating = get_rating(values, false);
    println!("part 2:  {}", oxygen_rating * co2_rating);
}

fn get_rating(values: &[Vec<usize>], get_most: bool) -> usize {
    let size = values[0].len();
    let mut current = values.to_vec();
    for idx in 0..size {
        let mut ones = Vec::new();
        let mut zeroes = Vec::new();
        for row in &current {
            if row[idx] == 1 {
                ones.push(row.to_vec());
            } else {
                zeroes.push(row.to_vec());
            }
        }
        if (ones.len() >= zeroes.len()) == get_most {
            current = ones;
        } else {
            current = zeroes;
        }
        if current.len() == 1 {
            break;
        }
    }
    to_int(&current[0])
}

fn to_int(row: &[usize]) -> usize {
    let mut acc = 0;
    for bit in row {
        acc = acc << 1;
        acc += bit;
    }

    acc
}
