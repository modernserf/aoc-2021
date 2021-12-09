fn main() {
    let contents = include_str!("input.txt");
    let values = contents
        .split("\n")
        .filter_map(|line| line.parse::<u32>().ok())
        .collect::<Vec<u32>>();
    {
        let mut prev = values[0];
        let mut increasing_count = 0;
        for i in 1..(values.len()) {
            let item = values[i];
            if item > prev {
                increasing_count += 1;
            }
            prev = item;
        }
        println!("part 1: {}", increasing_count);
    }

    {
        let mut prev = values[0] + values[1] + values[2];
        let mut increasing_count = 0;

        for i in 3..(values.len()) {
            let sum = values[i - 2] + values[i - 1] + values[i];
            if sum > prev {
                increasing_count += 1;
            }
            prev = sum;
        }

        println!("part 2: {}", increasing_count);
    }
}
