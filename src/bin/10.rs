use std::collections::BTreeMap;

use advent_of_code_2021::*;

aoc!(Day);

impl AOC for Day {
    type Input = Vec<String>;

    type Output = i64;

    fn input() -> Result<Self::Input> {
        Ok(get_contents()?.lines().map(|l| l.to_string()).collect())
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        let dat = [
            (')', ('(', 3)),
            (']', ('[', 57)),
            ('}', ('{', 1197)),
            ('>', ('<', 25137)),
        ];
        let mm = BTreeMap::<char, (char, i64)>::from_iter(dat.iter().cloned());

        let mut ret = 0;

        for line in input {
            let mut stk = vec![];
            let mut corrupt = false;
            let mut score = 0;

            for c in line.trim().chars() {
                match c {
                    '(' | '[' | '{' | '<' => stk.push(c),
                    ')' | ']' | '}' | '>' => {
                        let top = stk.pop();

                        if top.is_none() {
                            break;
                        }

                        let top = top.unwrap();

                        if top != mm[&c].0 {
                            corrupt = true;
                            score = mm[&c].1;
                            break;
                        }
                    }
                    _ => unreachable!(),
                }
            }

            if corrupt {
                ret += score;
            }
        }

        ret
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let dat = [
            (')', ('(', 1)),
            (']', ('[', 2)),
            ('}', ('{', 3)),
            ('>', ('<', 4)),
        ];
        let mm = BTreeMap::<char, (char, i64)>::from_iter(dat.iter().cloned());

        let mut ret = vec![];

        for line in input {
            let mut stk = vec![];
            let mut corrupt = false;

            for c in line.trim().chars() {
                match c {
                    '(' | '[' | '{' | '<' => stk.push(c),
                    ')' | ']' | '}' | '>' => {
                        let top = stk.pop();

                        if top.is_none() {
                            corrupt = true;
                            break;
                        }

                        let top = top.unwrap();

                        if top != mm[&c].0 {
                            corrupt = true;
                            break;
                        }
                    }
                    _ => unreachable!(),
                }
            }

            if corrupt {
                continue;
            }

            let mut score = 0;

            for c in stk.into_iter().rev() {
                score = score * 5 + dat.iter().find(|(_, (d, _))| c == *d).unwrap().1 .1;
            }

            ret.push(score);
        }

        ret.sort();
        assert!(ret.len() % 2 == 1);

        ret[ret.len() / 2]
    }
}
