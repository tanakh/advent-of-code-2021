use std::collections::BTreeMap;

use advent_of_code_2021::*;

aoc!(Day);

struct Input {
    start: String,
    rules: Vec<([char; 2], char)>,
}

impl AOC for Day {
    type Input = Input;

    type Output = usize;

    fn input() -> Result<Self::Input> {
        let c = get_contents()?;
        let mut it = c.lines();

        let start = it.next().unwrap().to_string();
        it.next().unwrap();

        let re = regex::Regex::new(r"^(\w+) -> (\w+)$").unwrap();

        let mut rules = vec![];

        for line in it {
            let caps = re.captures(line).unwrap();
            let lhs = &caps[1].chars().collect::<Vec<_>>();
            let rhs = &caps[2].chars().collect::<Vec<_>>();

            assert_eq!(lhs.len(), 2);
            assert_eq!(rhs.len(), 1);

            rules.push(([lhs[0], lhs[1]], rhs[0]));
        }

        Ok(Input { start, rules })
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        let mut cur = input.start.chars().collect::<Vec<_>>();

        for _ in 0..10 {
            let mut next = vec![];

            for i in 0..cur.len() - 1 {
                let pat = [cur[i], cur[i + 1]];
                let c = input.rules.iter().find(|r| r.0 == pat).unwrap().1;
                next.push(pat[0]);
                next.push(c);
            }

            next.push(cur[cur.len() - 1]);

            cur = next;
        }

        let mut mm = BTreeMap::<char, usize>::new();
        for &c in &cur {
            *mm.entry(c).or_default() += 1;
        }

        mm.values().max().unwrap() - mm.values().min().unwrap()
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let mut pairs = BTreeMap::<[char; 2], usize>::new();

        let s = input.start.chars().collect::<Vec<_>>();
        for i in 1..s.len() {
            *pairs.entry([s[i - 1], s[i]]).or_default() += 1;
        }

        for _ in 0..40 {
            let mut next = BTreeMap::<[char; 2], usize>::new();

            for (pat, count) in pairs.iter() {
                let c = input.rules.iter().find(|r| &r.0 == pat).unwrap().1;
                *next.entry([pat[0], c]).or_default() += count;
                *next.entry([c, pat[1]]).or_default() += count;
            }

            pairs = next;
        }

        let mut mm = BTreeMap::<char, usize>::new();
        for (pat, count) in pairs {
            *mm.entry(pat[0]).or_default() += count;
            *mm.entry(pat[1]).or_default() += count;
        }

        *mm.entry(s[0]).or_default() += 1;
        *mm.entry(s[s.len() - 1]).or_default() += 1;

        mm.values().max().unwrap() / 2 - mm.values().min().unwrap() / 2
    }
}
