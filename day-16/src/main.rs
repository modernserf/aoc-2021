fn input_to_bytes(input: &str) -> Vec<u8> {
    let mut out = Vec::new();
    let mut iter = input.chars().map(|ch| ch.to_digit(16).unwrap());

    while let Some(left) = iter.next() {
        let right = iter.next().unwrap_or(0);
        out.push(((left << 4) + right) as u8)
    }
    out
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum LengthType {
    TotalLength(usize),
    SubLength(usize),
}

#[derive(Debug, PartialEq, Clone)]
enum Packet {
    Literal {
        version: usize,
        value: usize,
    },
    Op {
        version: usize,
        op: usize,
        args: Vec<Packet>,
    },
}

impl Packet {
    pub fn version_sum(&self) -> usize {
        match self {
            Packet::Literal { version, .. } => *version,
            Packet::Op { version, args, .. } => {
                let sum = args.iter().map(|arg| arg.version_sum()).sum::<usize>();
                *version + sum
            }
        }
    }

    pub fn eval(&self) -> usize {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Op { op, args, .. } => match op {
                0 => args.iter().fold(0, |sum, arg| sum + arg.eval()),
                1 => args.iter().fold(1, |product, arg| product * arg.eval()),
                2 => args.iter().fold(usize::MAX, |min, arg| min.min(arg.eval())),
                3 => args.iter().fold(0, |max, arg| max.max(arg.eval())),
                // literal is #4
                5 => {
                    let l = args[0].eval();
                    let r = args[1].eval();
                    if l > r {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    let l = args[0].eval();
                    let r = args[1].eval();
                    if l < r {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    let l = args[0].eval();
                    let r = args[1].eval();
                    if l == r {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("unknown op"),
            },
        }
    }
}

struct Parser {
    input: Vec<u8>,
    index: usize,
}

impl Parser {
    pub fn parse(input: Vec<u8>) -> Packet {
        Parser::init(input).match_packet()
    }

    fn init(input: Vec<u8>) -> Parser {
        Parser { input, index: 0 }
    }
    fn next(&mut self, count: usize) -> usize {
        if count > 32 {
            panic!("cannot take >32 bits");
        };
        let mut acc = 0;
        for _ in 0..count {
            let bit_index = self.index & 7;
            let byte_index = self.index >> 3;
            let byte = self.input[byte_index];
            let bit = (byte >> (7 - bit_index)) & 1;
            self.index += 1;
            acc = (acc << 1) + bit as usize;
        }
        acc
    }
    fn bit_index(&self) -> usize {
        self.index
    }

    fn match_packet(&mut self) -> Packet {
        let version = self.match_version();
        match self.next(3) {
            4 => self.match_literal(version),
            n => self.match_op(version, n),
        }
    }

    fn match_version(&mut self) -> usize {
        self.next(3)
    }

    fn match_literal(&mut self, version: usize) -> Packet {
        let value = self.match_nybble_seq();
        Packet::Literal { version, value }
    }

    fn match_nybble_seq(&mut self) -> usize {
        let mut out = 0;
        loop {
            let (has_more, value) = self.match_nybble();
            out = (out << 4) + value;
            if !has_more {
                break;
            }
        }
        out
    }

    fn match_nybble(&mut self) -> (bool, usize) {
        let has_more = self.next(1);
        let value = self.next(4);
        (has_more == 1, value)
    }

    fn match_op(&mut self, version: usize, op: usize) -> Packet {
        let len = self.match_length();
        match len {
            LengthType::SubLength(len) => {
                let mut args = Vec::new();
                for _ in 0..len {
                    let packet = self.match_packet();
                    args.push(packet);
                }

                Packet::Op { version, op, args }
            }
            LengthType::TotalLength(bit_len) => {
                let mut args = Vec::new();
                let end_at = self.bit_index() + bit_len;

                while self.bit_index() < end_at {
                    let packet = self.match_packet();
                    args.push(packet);
                }

                Packet::Op { version, op, args }
            }
        }
    }

    fn match_length(&mut self) -> LengthType {
        let type_bit = self.next(1);

        if type_bit == 0 {
            let len = self.next(15);
            LengthType::TotalLength(len)
        } else {
            let len = self.next(11);
            LengthType::SubLength(len)
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let data = input_to_bytes(&input);
    let parsed = Parser::parse(data);

    println!("part 1: {}", parsed.version_sum());

    println!("part 2: {}", parsed.eval());
}
