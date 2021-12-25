use std::{cmp::max, fmt::Display, iter::Peekable};

use advent_of_code_2021::*;

aoc!(Day);

impl AOC for Day {
    type Input = Vec<List>;

    type Output = i64;

    fn input() -> Result<Self::Input> {
        let c = get_contents()?;
        Ok(c.lines()
            .map(|s| parse(&mut s.chars().peekable()))
            .collect())
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        let mut cur = input[0].clone();

        for list in &input[1..] {
            cur = List::Pair(Box::new(cur), Box::new(list.clone()));

            print!("{}", cur);
            cur.normalize();
            println!(" -> {}", cur);
        }

        cur.magnitude()
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let mut ret = 0;

        for i in 0..input.len() {
            for j in 0..input.len() {
                if i == j {
                    continue;
                }

                let mut cur = List::Pair(Box::new(input[i].clone()), Box::new(input[j].clone()));
                cur.normalize();
                ret = max(ret, cur.magnitude());
            }
        }

        ret
    }
}

#[derive(Debug, Clone)]
enum List {
    Pair(Box<List>, Box<List>),
    Num(i64),
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            List::Pair(l, r) => write!(f, "[{},{}]", l, r),
            List::Num(n) => write!(f, "{}", n),
        }
    }
}

impl List {
    // fn is_pair(&self) -> bool {
    //     match self {
    //         List::Pair(_, _) => true,
    //         _ => false,
    //     }
    // }

    fn is_num(&self) -> bool {
        match self {
            List::Num(_) => true,
            _ => false,
        }
    }

    fn to_num(&self) -> Option<i64> {
        match self {
            List::Num(n) => Some(*n),
            _ => None,
        }
    }

    fn normalize(&mut self) {
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

    fn explode(&mut self, dep: usize) -> Option<(i64, i64)> {
        if dep >= 4 {
            let t = match self {
                List::Pair(l, r) if l.is_num() && r.is_num() => Some((l.to_num()?, r.to_num()?)),
                _ => None,
            };

            if t.is_some() {
                *self = List::Num(0);
                return t;
            }
        }

        match self {
            List::Pair(l, r) => {
                if let Some((lv, rv)) = l.explode(dep + 1) {
                    r.add_first(rv);
                    return Some((lv, 0));
                }

                if let Some((lv, rv)) = r.explode(dep + 1) {
                    l.add_last(lv);
                    return Some((0, rv));
                }

                return None;
            }
            _ => None,
        }
    }

    fn add_first(&mut self, v: i64) {
        match self {
            List::Pair(l, _) => {
                l.add_first(v);
            }
            List::Num(n) => {
                *n += v;
            }
        }
    }

    fn add_last(&mut self, v: i64) {
        match self {
            List::Pair(_, r) => {
                r.add_last(v);
            }
            List::Num(n) => {
                *n += v;
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            List::Pair(l, r) => l.split() || r.split(),
            List::Num(v) => {
                if *v >= 10 {
                    let l = *v / 2;
                    let r = *v - l;
                    *self = List::Pair(Box::new(List::Num(l)), Box::new(List::Num(r)));
                    true
                } else {
                    false
                }
            }
        }
    }

    fn magnitude(&self) -> i64 {
        match self {
            List::Pair(l, r) => l.magnitude() * 3 + r.magnitude() * 2,
            List::Num(n) => *n,
        }
    }
}

fn parse(s: &mut Peekable<impl Iterator<Item = char>>) -> List {
    let c = s.next().unwrap();

    if c == '[' {
        let l = parse(s);
        assert_eq!(s.next().unwrap(), ',');
        let r = parse(s);
        assert_eq!(s.next().unwrap(), ']');
        List::Pair(Box::new(l), Box::new(r))
    } else if c.is_ascii_digit() {
        let mut v = c.to_digit(10).unwrap() as i64;
        while matches!(s.peek(), Some(c) if c.is_ascii_digit()) {
            let c = s.next().unwrap();
            v = v * 10 + c.to_digit(10).unwrap() as i64;
        }
        List::Num(v)
    } else {
        unreachable!()
    }
}
