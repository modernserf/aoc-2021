use std::collections::HashSet;

struct Enhancer {
    data: [u8; 64],
}

impl Enhancer {
    fn from_string(input: &str) -> Self {
        let mut data = [0; 64];
        for (i, ch) in input.chars().enumerate() {
            if ch == '#' {
                let byte_index = i >> 3;
                let bit_index = i & 0b111;
                data[byte_index] += 1 << bit_index;
            }
        }

        Enhancer { data }
    }
    fn enhance(&self, index: usize) -> bool {
        let byte_index = index >> 3;
        let bit_index = index & 0b111;
        let value = (self.data[byte_index] >> bit_index) & 1;
        value == 1
    }
}

struct Image {
    data: HashSet<(isize, isize)>,
    outer_fill: bool,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl Image {
    pub fn from_string(input: &str) -> Self {
        let mut data = HashSet::new();
        let mut x_max = 0;
        let mut y_max = 0;
        for (y, line) in input.split('\n').enumerate() {
            y_max = y as isize;
            for (x, ch) in line.chars().enumerate() {
                x_max = x as isize;
                if ch == '#' {
                    data.insert((x as isize, y as isize));
                }
            }
        }

        Image {
            data,
            outer_fill: false,
            x_min: 0,
            y_min: 0,
            x_max,
            y_max,
        }
    }
    pub fn enhance(&self, enhancer: &Enhancer) -> Self {
        let mut next_data = HashSet::new();
        let expand = 1;
        let x_min = self.x_min - expand;
        let x_max = self.x_max + expand;
        let y_min = self.y_min - expand;
        let y_max = self.y_max + expand;

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let idx = self.get_index(x, y);
                if enhancer.enhance(idx) {
                    next_data.insert((x, y));
                }
            }
        }

        let outer_fill = if self.outer_fill {
            enhancer.enhance(0x1FF)
        } else {
            enhancer.enhance(0)
        };

        Image {
            data: next_data,
            x_min,
            x_max,
            y_min,
            y_max,
            outer_fill,
        }
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    fn get_index(&self, x: isize, y: isize) -> usize {
        self.get_bit(x - 1, y - 1, 8)
            + self.get_bit(x, y - 1, 7)
            + self.get_bit(x + 1, y - 1, 6)
            + self.get_bit(x - 1, y, 5)
            + self.get_bit(x, y, 4)
            + self.get_bit(x + 1, y, 3)
            + self.get_bit(x - 1, y + 1, 2)
            + self.get_bit(x, y + 1, 1)
            + self.get_bit(x + 1, y + 1, 0)
    }
    fn get_bit(&self, x: isize, y: isize, shift: usize) -> usize {
        if x < self.x_min || x > self.x_max || y < self.y_min || y > self.y_max {
            return if self.outer_fill { 1 << shift } else { 0 };
        }

        if self.data.contains(&(x, y)) {
            1 << shift
        } else {
            0
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in self.y_min..=self.y_max {
            let mut line = String::new();
            for x in self.x_min..=self.x_max {
                if self.data.contains(&(x, y)) {
                    line.push('#')
                } else {
                    line.push('.')
                }
            }
            println!("{}", line);
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut parts = input.split("\n\n");
    let enhancer = Enhancer::from_string(parts.next().unwrap());
    let image = Image::from_string(parts.next().unwrap());

    let image = image.enhance(&enhancer);
    let image = image.enhance(&enhancer);

    println!("part 1: {}", image.count());

    let mut image = image;
    for _ in 2..50 {
        image = image.enhance(&enhancer);
    }

    println!("part 2: {}", image.count());
}
