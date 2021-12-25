use std::collections::BTreeMap;

use advent_of_code_2021::*;

aoc!(Day);

impl AOC for Day {
    type Input = Vec<(String, String)>;

    type Output = usize;

    fn input() -> Result<Self::Input> {
        let c = get_contents()?;
        let ret = c
            .lines()
            .map(|line| {
                let mut it = line.split("-");
                let a = it.next().unwrap().to_string();
                let b = it.next().unwrap().to_string();
                assert!(is_uppercase(&a) || is_lowercase(&a));
                assert!(is_uppercase(&b) || is_lowercase(&b));
                assert!(!(is_uppercase(&a) && is_uppercase(&b)));
                (a, b)
            })
            .collect();
        Ok(ret)
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        let mut g = BTreeMap::new();
        for (u, v) in input {
            g.entry(u.as_str()).or_insert(vec![]).push(v.as_str());
            g.entry(v.as_str()).or_insert(vec![]).push(u.as_str());
        }
        solve(&g, "start", "end", &mut vec![])
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let mut g = BTreeMap::new();
        for (u, v) in input {
            g.entry(u.as_str()).or_insert(vec![]).push(v.as_str());
            g.entry(v.as_str()).or_insert(vec![]).push(u.as_str());
        }
        solve2(&g, "start", "start", "end", &mut vec![], false)
    }
}

fn solve<'a>(
    g: &BTreeMap<&'a str, Vec<&'a str>>,
    u: &'a str,
    end: &'a str,
    hist: &mut Vec<&'a str>,
) -> usize {
    if u == end {
        return 1;
    }

    if is_lowercase(u) {
        hist.push(u);
    }

    let mut ret = 0;

    for &v in g[u].iter() {
        if hist.contains(&v) {
            continue;
        }

        ret += solve(g, v, end, hist);
    }

    if is_lowercase(u) {
        hist.pop();
    }

    ret
}

fn solve2<'a>(
    g: &BTreeMap<&'a str, Vec<&'a str>>,
    u: &'a str,
    start: &'a str,
    end: &'a str,
    hist: &mut Vec<&'a str>,
    used: bool,
) -> usize {
    if u == end {
        return 1;
    }

    if is_lowercase(u) {
        hist.push(u);
    }

    let mut ret = 0;

    for &v in g[u].iter() {
        if v == start {
            continue;
        }

        let contains = hist.contains(&v);

        if contains && used {
            continue;
        }

        ret += solve2(g, v, start, end, hist, used || contains);
    }

    if is_lowercase(u) {
        hist.pop();
    }

    ret
}

fn is_lowercase(s: &str) -> bool {
    s.chars().all(|c| c.is_lowercase())
}

fn is_uppercase(s: &str) -> bool {
    s.chars().all(|c| c.is_uppercase())
}
