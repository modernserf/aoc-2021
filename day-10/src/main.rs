#[derive(PartialEq)]
enum Bracket {
    Round,
    Square,
    Curly,
    Angle,
}

fn main() {
    let input = include_str!("input.txt");
    let lines = input.split("\n").collect::<Vec<_>>();

    let mut score = 0;
    let mut incomplete_lines = Vec::new();
    for line in lines {
        match check_line(line) {
            Ok(stack) => incomplete_lines.push(stack),
            Err(line_score) => score += line_score,
        }
    }

    println!("part 1: {}", score);

    let mut scores = incomplete_lines
        .iter()
        .map(|line| score_incomplete_line(&line))
        .collect::<Vec<_>>();

    scores.sort_by(|l, r| l.cmp(r));

    let median = scores[(scores.len() >> 1)];

    println!("part 2: {}", median);
}

fn check_line(line: &str) -> Result<Vec<Bracket>, usize> {
    let mut stack = Vec::new();
    for ch in line.chars() {
        match ch {
            '(' => stack.push(Bracket::Round),
            '[' => stack.push(Bracket::Square),
            '{' => stack.push(Bracket::Curly),
            '<' => stack.push(Bracket::Angle),
            ')' => {
                if stack.pop().unwrap() != Bracket::Round {
                    return Err(3);
                }
            }
            ']' => {
                if stack.pop().unwrap() != Bracket::Square {
                    return Err(57);
                }
            }
            '}' => {
                if stack.pop().unwrap() != Bracket::Curly {
                    return Err(1197);
                }
            }
            '>' => {
                if stack.pop().unwrap() != Bracket::Angle {
                    return Err(25137);
                }
            }
            _ => panic!("unknown char"),
        }
    }
    Ok(stack)
}

fn score_incomplete_line(remaining: &[Bracket]) -> usize {
    let mut score = 0;

    for brak in remaining.iter().rev() {
        score *= 5;
        match brak {
            Bracket::Round => score += 1,
            Bracket::Square => score += 2,
            Bracket::Curly => score += 3,
            Bracket::Angle => score += 4,
        }
    }

    score
}
