use advent_of_code_2021::*;

aoc!(Day);

impl AOC for Day {
    type Input = Vec<Vec<i64>>;

    type Output = usize;

    fn input() -> Result<Self::Input> {
        use proconio::{input, marker::Chars};
        input! {
            bd: [Chars; 10]
        }
        Ok(bd
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|c| c.to_digit(10).unwrap() as i64)
                    .collect()
            })
            .collect())
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        let mut ret = 0;
        let mut bd = input.clone();

        for _ in 0..100 {
            ret += step(&mut bd);
        }

        ret
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let mut bd = input.clone();

        for i in 0.. {
            if bd.iter().all(|row| row.iter().all(|&v| v == 0)) {
                return i;
            }
            step(&mut bd);
        }

        unreachable!()
    }
}

fn step(bd: &mut Vec<Vec<i64>>) -> usize {
    let mut ret = 0;

    let mut done = vec![vec![false; 10]; 10];

    for i in 0..10 {
        for j in 0..10 {
            bd[i][j] += 1;
        }
    }

    loop {
        let mut cont = false;

        for i in 0..10 {
            for j in 0..10 {
                if bd[i][j] >= 10 && !done[i][j] {
                    cont = true;
                    done[i][j] = true;
                    ret += 1;
                    for di in -1..=1 {
                        for dj in -1..=1 {
                            if (di, dj) == (0, 0) {
                                continue;
                            }
                            let ci = i as i64 + di;
                            let cj = j as i64 + dj;
                            if ci >= 0 && ci < 10 && cj >= 0 && cj < 10 {
                                bd[ci as usize][cj as usize] += 1;
                            }
                        }
                    }
                }
            }
        }

        if !cont {
            break;
        }
    }

    for i in 0..10 {
        for j in 0..10 {
            if bd[i][j] >= 10 {
                bd[i][j] = 0;
            }
        }
    }

    ret
}
