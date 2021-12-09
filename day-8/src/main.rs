struct Row {
    left: Vec<u8>,
    right: Vec<u8>,
}

fn main() {
    let input = include_str!("input.txt");
    let rows = input
        .split("\n")
        .map(|line| {
            let parts = line
                .split(" | ")
                .map(|block| {
                    block
                        .split_whitespace()
                        .map(|word| to_byte(word))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            Row {
                left: parts[0].clone(),
                right: parts[1].clone(),
            }
        })
        .collect::<Vec<_>>();

    let mut easy_digit_count = 0;

    for row in rows.iter() {
        for digit in &row.right {
            match digit.count_ones() {
                2 | 3 | 4 | 7 => {
                    easy_digit_count += 1;
                }
                _ => {}
            }
        }
    }

    println!("part 1: {}", easy_digit_count);

    let sum = rows.iter().map(|row| decode_row(row)).sum::<usize>();

    println!("part 2: {}", sum);
}

fn to_byte(s: &str) -> u8 {
    let mut byte: u8 = 0;
    for ch in s.chars() {
        match ch {
            'a' => byte += 1,
            'b' => byte += 2,
            'c' => byte += 4,
            'd' => byte += 8,
            'e' => byte += 16,
            'f' => byte += 32,
            'g' => byte += 64,
            _ => panic!("unknown char"),
        }
    }

    byte
}

fn decode_row(row: &Row) -> usize {
    let mut digits = vec![0; 10];
    let mut sixes: Vec<u8> = Vec::with_capacity(3);
    let mut fives: Vec<u8> = Vec::with_capacity(3);

    for item in row.left.iter() {
        match item.count_ones() {
            2 => digits[1] = *item,
            3 => digits[7] = *item,
            4 => digits[4] = *item,
            5 => fives.push(*item),
            6 => sixes.push(*item),
            7 => digits[8] = *item,
            _ => {}
        }
    }

    for item in sixes {
        if (item & digits[1]) == digits[1] {
            if (item & digits[4]) == digits[4] {
                digits[9] = item
            } else {
                digits[0] = item
            }
        } else {
            digits[6] = item
        }
    }

    for item in fives {
        if (item & digits[1]) == digits[1] {
            digits[3] = item
        } else if (item & digits[6]) == item {
            digits[5] = item
        } else {
            digits[2] = item
        }
    }

    let mut result = 0;

    for item in row.right.iter() {
        result *= 10;
        let index = digits.iter().position(|digit| *digit == *item).unwrap();
        result += index;
    }

    result
}
