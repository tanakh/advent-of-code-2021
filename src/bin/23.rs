use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

use advent_of_code_2021::*;

aoc!(Day);

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct State<const D: usize> {
    rooms: [[usize; D]; 4],
    hallway: [usize; 7],
}

impl<const D: usize> Display for State<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = |d| match d {
            0 => '.',
            1 => 'A',
            2 => 'B',
            3 => 'C',
            4 => 'D',
            _ => unreachable!(),
        };

        writeln!(f, "#############")?;
        writeln!(
            f,
            "#{}{}.{}.{}.{}.{}{}#",
            c(self.hallway[0]),
            c(self.hallway[1]),
            c(self.hallway[2]),
            c(self.hallway[3]),
            c(self.hallway[4]),
            c(self.hallway[5]),
            c(self.hallway[6]),
        )?;

        for i in 0..D {
            write!(f, "{}", if i == 0 { "###" } else { "  #" })?;
            for j in 0..4 {
                write!(f, "{}#", c(self.rooms[j][i]))?;
            }
            writeln!(f, "{}", if i == 0 { "##" } else { "  " })?;
        }

        writeln!(f, "  #########  ")?;

        Ok(())
    }
}

impl<const D: usize> State<D> {
    fn check(&self) -> bool {
        for i in 0..4 {
            let mut j = 0;
            while j < D && self.rooms[i][j] == 0 {
                j += 1;
            }

            for k in j..D {
                if self.rooms[i][k] == 0 {
                    return false;
                }
            }
        }

        let mut cnt = vec![0; 5];

        for i in 0..4 {
            for j in 0..D {
                cnt[self.rooms[i][j]] += 1;
            }
        }

        for i in 0..7 {
            cnt[self.hallway[i]] += 1;
        }

        let ok = &cnt[1..] == &[D, D, D, D];

        if !ok {
            dbg!(self, cnt);
        }

        return ok;
    }
}

impl AOC for Day {
    type Input = State<2>;

    type Output = i64;

    fn input() -> Result<Self::Input> {
        let c = get_contents()?;

        let bd = c
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let f = |c: char| -> usize {
            match c {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                'D' => 4,
                _ => panic!(),
            }
        };

        let rooms = [
            [f(bd[2][3]), f(bd[3][3])],
            [f(bd[2][5]), f(bd[3][5])],
            [f(bd[2][7]), f(bd[3][7])],
            [f(bd[2][9]), f(bd[3][9])],
        ];

        Ok(State {
            rooms,
            hallway: [0; 7],
        })
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        let goal = State {
            rooms: [[1, 1], [2, 2], [3, 3], [4, 4]],
            hallway: [0; 7],
        };

        solve(input, &goal)
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let start = State {
            rooms: [
                [input.rooms[0][0], 4, 4, input.rooms[0][1]],
                [input.rooms[1][0], 3, 2, input.rooms[1][1]],
                [input.rooms[2][0], 2, 1, input.rooms[2][1]],
                [input.rooms[3][0], 1, 3, input.rooms[3][1]],
            ],
            hallway: [0; 7],
        };

        let goal = State {
            rooms: [[1, 1, 1, 1], [2, 2, 2, 2], [3, 3, 3, 3], [4, 4, 4, 4]],
            hallway: [0; 7],
        };

        solve(&start, &goal)
    }
}

fn solve<const D: usize>(start: &State<D>, goal: &State<D>) -> i64 {
    let mut done = HashMap::<State<D>, Option<State<D>>>::new();
    let mut q = BinaryHeap::<(Reverse<i64>, State<D>, Option<State<D>>)>::new();

    let cost = |c| -> i64 { 10_i64.pow(c as u32 - 1) };

    q.push((Reverse(0), start.clone(), None));

    while let Some((Reverse(dep), cur, prev)) = q.pop() {
        assert!(cur.check());

        if done.contains_key(&cur) {
            continue;
        }
        done.insert(cur.clone(), prev);

        if &cur == goal {
            // let mut hist = vec![];

            // hist.push(cur.clone());

            // let mut cur = cur.clone();

            // while let Some(prev) = &done[&cur] {
            //     hist.push(cur.clone());
            //     cur = prev.clone();
            // }

            // hist.reverse();

            // for s in hist {
            //     println!("{}", s);
            // }

            return dep;
        }

        // hallway to room

        for i in 0..4 {
            let c = i + 1;

            let mut ok = true;
            for j in 0..D {
                if !(cur.rooms[i][j] == 0 || cur.rooms[i][j] == c) {
                    ok = false;
                    break;
                }
            }

            if !ok {
                continue;
            }

            let mut j = 0;
            while j < D && cur.rooms[i][j] == 0 {
                j += 1;
            }
            if j == 0 {
                continue;
            }
            j -= 1;

            for k in i + 2..7 {
                if cur.hallway[k] == 0 {
                    continue;
                }

                if cur.hallway[k] != c {
                    break;
                }

                let mut next = cur.clone();
                next.rooms[i][j] = c;
                next.hallway[k] = 0;

                if !done.contains_key(&next) {
                    let steps = j + 2 + (k - i - 2) * 2 - if k == 6 { 1 } else { 0 };

                    q.push((
                        Reverse(dep + cost(c) * steps as i64),
                        next,
                        Some(cur.clone()),
                    ));
                }

                break;
            }

            for k in (0..i + 2).rev() {
                if cur.hallway[k] == 0 {
                    continue;
                }

                if cur.hallway[k] != c {
                    break;
                }

                let mut next = cur.clone();
                next.rooms[i][j] = c;
                next.hallway[k] = 0;

                if !done.contains_key(&next) {
                    let steps = j + 2 + (i + 1 - k) * 2 - if k == 0 { 1 } else { 0 };

                    q.push((
                        Reverse(dep + cost(c) * steps as i64),
                        next,
                        Some(cur.clone()),
                    ));
                }

                break;
            }
        }

        // room to hallway

        for i in 0..4 {
            let mut j = 0;
            while j < D && cur.rooms[i][j] == 0 {
                j += 1;
            }
            if j >= D {
                continue;
            }

            let c = cur.rooms[i][j];

            for k in i + 2..7 {
                if cur.hallway[k] != 0 {
                    break;
                }

                let mut next = cur.clone();
                next.rooms[i][j] = 0;
                next.hallway[k] = c;

                if !done.contains_key(&next) {
                    let steps = j + 2 + (k - i - 2) * 2 - if k == 6 { 1 } else { 0 };

                    q.push((
                        Reverse(dep + cost(c) * steps as i64),
                        next,
                        Some(cur.clone()),
                    ));
                }
            }

            for k in (0..i + 2).rev() {
                if cur.hallway[k] != 0 {
                    break;
                }

                let mut next = cur.clone();
                next.rooms[i][j] = 0;
                next.hallway[k] = c;

                if !done.contains_key(&next) {
                    let steps = j + 2 + (i + 1 - k) * 2 - if k == 0 { 1 } else { 0 };

                    q.push((
                        Reverse(dep + cost(c) * steps as i64),
                        next,
                        Some(cur.clone()),
                    ));
                }
            }
        }
    }

    unreachable!()
}
