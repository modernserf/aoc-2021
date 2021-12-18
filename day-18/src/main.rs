use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Num(usize),
    Pair(Box<Value>, Box<Value>),
}

fn parse_input(input: &str) -> Vec<Value> {
    input
        .split("\n")
        .map(|line| {
            let mut chars = line.chars();
            parse_value(&mut chars)
        })
        .collect::<Vec<_>>()
}

fn parse_value(chars: &mut Chars) -> Value {
    match chars.next().unwrap() {
        '[' => {
            let left = parse_value(chars);
            chars.next(); // ','
            let right = parse_value(chars);
            chars.next(); // ']'
            Value::Pair(Box::new(left), Box::new(right))
        }
        ch => Value::Num(ch.to_digit(10).unwrap() as usize),
    }
}

impl Value {
    fn print(&self) -> String {
        match self {
            Value::Num(num) => num.to_string(),
            Value::Pair(left, right) => {
                format!("[{},{}]", left.print(), right.print())
            }
        }
    }
    fn add(self, right: Value) -> Value {
        let mut sum = Value::Pair(Box::new(self), Box::new(right));
        sum.reduce();
        sum
    }
    fn reduce(&mut self) {
        loop {
            if self.explode(0).is_some() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }

    fn explode(&mut self, depth: usize) -> Option<(usize, usize)> {
        match self {
            Value::Num(_) => None,
            Value::Pair(left, right) => {
                if depth < 4 {
                    if let Some((l, r)) = left.explode(depth + 1) {
                        right.add_left(r);
                        Some((l, 0))
                    } else if let Some((l, r)) = right.explode(depth + 1) {
                        left.add_right(l);
                        Some((0, r))
                    } else {
                        None
                    }
                } else {
                    let pair = (left.magnitude(), right.magnitude());
                    *self = Value::Num(0);
                    Some(pair)
                }
            }
        }
    }
    fn add_left(&mut self, add: usize) {
        match self {
            Value::Num(value) => *value += add,
            Value::Pair(l, _) => l.add_left(add),
        }
    }
    fn add_right(&mut self, add: usize) {
        match self {
            Value::Num(value) => *value += add,
            Value::Pair(_, r) => r.add_right(add),
        }
    }
    fn split(&mut self) -> bool {
        match self {
            Value::Num(value) => {
                let value = *value;
                if value < 10 {
                    return false;
                };
                let left = value / 2;
                let right = value - left;
                *self = Value::Pair(Box::new(Value::Num(left)), Box::new(Value::Num(right)));

                return true;
            }
            Value::Pair(left, right) => {
                if left.split() {
                    true
                } else if right.split() {
                    true
                } else {
                    false
                }
            }
        }
    }
    fn magnitude(&self) -> usize {
        match self {
            Value::Num(num) => *num,
            Value::Pair(left, right) => left.magnitude() * 3 + right.magnitude() * 2,
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let items = parse_input(input);
    let mut iter = items.iter().cloned();
    let mut result = iter.next().unwrap();

    for value in iter {
        result = result.add(value);
    }

    println!("part 1: {}", result.magnitude());

    let mut max = 0;
    for (l, left) in items.iter().enumerate() {
        for (r, right) in items.iter().enumerate() {
            if l == r {
                continue;
            }
            let magnitude = left.clone().add(right.clone()).magnitude();
            max = max.max(magnitude);
        }
    }

    println!("part 2: {}", max);
}

#[cfg(test)]
mod test {
    use super::*;
    fn lit(input: &str) -> Value {
        parse_input(input).pop().unwrap()
    }

    fn reduced(input: &str) -> Value {
        let mut value = lit(input);
        value.reduce();
        value
    }

    #[test]
    fn reduce_explode() {
        assert_eq!(reduced("[[[[[9,8],1],2],3],4]"), lit("[[[[0,9],2],3],4]"));
        assert_eq!(reduced("[7,[6,[5,[4,[3,2]]]]]"), lit("[7,[6,[5,[7,0]]]]"));
        assert_eq!(reduced("[[6,[5,[4,[3,2]]]],1]"), lit("[[6,[5,[7,0]]],3]"));
        assert_eq!(
            reduced("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"),
            lit("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
        );
    }

    #[test]
    fn reduce_split() {
        let mut val = Value::Num(10);
        val.reduce();
        assert_eq!(val, lit("[5,5]"));
        let mut val = Value::Num(11);
        val.reduce();
        assert_eq!(val, lit("[5,6]"));
    }

    #[test]
    fn add() {
        assert_eq!(
            lit("[[[[4,3],4],4],[7,[[8,4],9]]]").add(lit("[1,1]")),
            lit("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );

        assert_eq!(
            lit("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]")
                .add(lit("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]")),
            lit("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")
        );
    }

    #[test]
    fn magnitude() {
        assert_eq!(
            lit("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]").magnitude(),
            4140
        );
    }
}
