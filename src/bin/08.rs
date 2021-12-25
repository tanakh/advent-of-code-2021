use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use advent_of_code_2021::*;

aoc!(Day);

struct Case {
    pats: Vec<String>,
    output: Vec<String>,
}

impl AOC for Day {
    type Input = Vec<Case>;

    type Output = i64;

    fn input() -> Result<Self::Input> {
        let mut ret = vec![];

        for line in std::io::stdin().lock().lines() {
            let line = line?;
            let mut it = line.split(" | ");

            let l = it.next().unwrap();
            let r = it.next().unwrap();

            let pats = l.split_ascii_whitespace().map(|s| s.to_string()).collect();
            let output = r.split_ascii_whitespace().map(|s| s.to_string()).collect();

            ret.push(Case { pats, output });
        }

        Ok(ret)
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        let mut ret = 0;

        for case in input {
            for s in &case.output {
                ret += match s.len() {
                    2 | 3 | 4 | 7 => 1,
                    _ => 0,
                };
            }
        }

        ret
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let mut sum = 0;

        for case in input {
            let mut dict = HashMap::new();
            let mut rev = HashMap::new();
            let mut rest = HashSet::new();

            for s in &case.pats {
                rest.insert(s.clone());
            }

            let mut f = |rev: &mut HashMap<i64, String>, ix: i64, f: &dyn Fn(&str) -> bool| {
                let t = rest.iter().find(|w| f(w)).unwrap().clone();
                rest.remove(&t);

                let mut t = t.chars().collect::<Vec<_>>();
                t.sort();
                let t = t.iter().collect::<String>();

                dict.insert(t.clone(), ix);
                rev.insert(ix, t);
            };

            f(&mut rev, 1, &|w| w.len() == 2);
            f(&mut rev, 4, &|w| w.len() == 4);
            f(&mut rev, 7, &|w| w.len() == 3);
            f(&mut rev, 8, &|w| w.len() == 7);

            {
                let t = rev[&1].clone();
                f(&mut rev, 3, &|w| w.len() == 5 && contains(w, &t));
            }
            {
                let t = rev[&3].clone();
                f(&mut rev, 9, &|w| w.len() == 6 && contains(w, &t));
            }
            {
                let t = rev[&1].clone();
                f(&mut rev, 0, &|w| w.len() == 6 && contains(w, &t));
            }
            f(&mut rev, 6, &|w| w.len() == 6);

            {
                let t = rev[&6].clone();
                f(&mut rev, 5, &|w| w.len() == 5 && contains(&t, w));
            }

            f(&mut rev, 2, &|_| true);

            let mut v = 0;

            for output in &case.output {
                let mut t = output.chars().collect::<Vec<_>>();
                t.sort();
                let t = t.iter().collect::<String>();

                v = v * 10 + dict[&t];
            }

            sum += v;
        }

        sum
    }
}

fn contains(s: &str, t: &str) -> bool {
    t.chars().all(|c| s.contains(c))
}
