enum SubCommand {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn main() {
    // let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
    let input = include_str!("input.txt");
    let commands = input
        .split("\n")
        .filter_map(|line| {
            let res = line.split(" ").collect::<Vec<_>>();
            if res.len() == 2 {
                let tag = res[0];
                let value = res[1].parse::<u32>().unwrap();
                match tag {
                    "forward" => Some(SubCommand::Forward(value)),
                    "down" => Some(SubCommand::Down(value)),
                    "up" => Some(SubCommand::Up(value)),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect::<Vec<SubCommand>>();

    part_1(&commands);
    part_2(&commands);
}

fn part_1(commands: &[SubCommand]) {
    let mut depth = 0;
    let mut horiz = 0;
    for command in commands {
        match command {
            SubCommand::Forward(d) => {
                horiz += d;
            }
            SubCommand::Down(d) => {
                depth += d;
            }
            SubCommand::Up(d) => {
                depth -= d;
            }
        }
    }

    println!("part 1: {}", depth * horiz);
}

fn part_2(commands: &[SubCommand]) {
    let mut depth = 0;
    let mut horiz = 0;
    let mut aim = 0;
    for command in commands {
        match command {
            SubCommand::Forward(d) => {
                horiz += d;
                depth += aim * d;
            }
            SubCommand::Down(d) => {
                aim += d;
            }
            SubCommand::Up(d) => {
                aim -= d;
            }
        }
    }

    println!("part 1: {}", depth * horiz);
}
