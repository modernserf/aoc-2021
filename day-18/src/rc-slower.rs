use std::rc::Rc;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
enum ValueInner {
  Num(usize),
  Pair(Value, Value),
}
use ValueInner::{Num, Pair};

#[derive(Debug, Clone, PartialEq)]
struct Value(Rc<ValueInner>);

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
      Value::pair(left, right)
    }
    ch => Value::num(ch.to_digit(10).unwrap() as usize),
  }
}

impl Value {
  fn num(value: usize) -> Value {
    Value(Rc::new(Num(value)))
  }
  fn pair(left: Value, right: Value) -> Value {
    Value(Rc::new(Pair(left, right)))
  }

  fn add(&self, right: &Value) -> Value {
    Self::pair(self.clone(), right.clone()).reduce()
  }
  fn reduce(&self) -> Value {
    let (next, pair) = self.explode(0);
    if pair.is_some() {
      return next.reduce();
    };
    let (next, did_split) = next.split();
    if did_split {
      return next.reduce();
    }
    next
  }

  fn explode(&self, depth: usize) -> (Value, Option<(usize, usize)>) {
    match &*self.0 {
      Num(_) => (self.clone(), None),
      Pair(left, right) => {
        if depth < 4 {
          if let (left, Some((l, r))) = left.explode(depth + 1) {
            let right = right.add_left(r);
            (Self::pair(left, right), Some((l, 0)))
          } else if let (right, Some((l, r))) = right.explode(depth + 1) {
            let left = left.add_right(l);
            (Self::pair(left, right), Some((0, r)))
          } else {
            (self.clone(), None)
          }
        } else {
          (Self::num(0), Some((left.magnitude(), right.magnitude())))
        }
      }
    }
  }
  fn add_left(&self, add: usize) -> Value {
    match &*self.0 {
      Num(value) => Self::num(value + add),
      Pair(l, r) => Self::pair(l.add_left(add), r.clone()),
    }
  }
  fn add_right(&self, add: usize) -> Value {
    match &*self.0 {
      Num(value) => Self::num(value + add),
      Pair(l, r) => Self::pair(l.clone(), r.add_right(add)),
    }
  }
  fn split(&self) -> (Value, bool) {
    match &*self.0 {
      Num(value) => {
        if *value < 10 {
          return (self.clone(), false);
        };
        let left = *value / 2;
        let right = *value - left;
        (Self::pair(Self::num(left), Self::num(right)), true)
      }
      Pair(left, right) => {
        let (left, did_split) = left.split();
        if did_split {
          return (Self::pair(left, right.clone()), true);
        }
        let (right, did_split) = right.split();
        if did_split {
          return (Self::pair(left, right), true);
        }
        return (Self::pair(left, right), false);
      }
    }
  }
  fn magnitude(&self) -> usize {
    match &*self.0 {
      Num(num) => *num,
      Pair(left, right) => left.magnitude() * 3 + right.magnitude() * 2,
    }
  }
}

fn main() {
  let input = include_str!("input.txt");
  let items = parse_input(input);
  let mut iter = items.iter();
  let mut result = iter.next().unwrap().clone();

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
      let magnitude = left.add(right).magnitude();
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

  #[test]
  fn reduce_explode() {
    assert_eq!(
      lit("[[[[[9,8],1],2],3],4]").reduce(),
      lit("[[[[0,9],2],3],4]")
    );
    assert_eq!(
      lit("[7,[6,[5,[4,[3,2]]]]]").reduce(),
      lit("[7,[6,[5,[7,0]]]]")
    );
    assert_eq!(
      lit("[[6,[5,[4,[3,2]]]],1]").reduce(),
      lit("[[6,[5,[7,0]]],3]")
    );
    assert_eq!(
      lit("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").reduce(),
      lit("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
    );
  }

  #[test]
  fn reduce_split() {
    assert_eq!(Value::num(10).reduce(), lit("[5,5]"));
    assert_eq!(Value::num(11).reduce(), lit("[5,6]"));
  }

  #[test]
  fn add() {
    assert_eq!(
      lit("[[[[4,3],4],4],[7,[[8,4],9]]]").add(&lit("[1,1]")),
      lit("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    );

    assert_eq!(
      lit("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]")
        .add(&lit("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]")),
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
