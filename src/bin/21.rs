use std::cmp::max;

use advent_of_code_2021::*;

aoc!(Day);

impl AOC for Day {
    type Input = (i64, i64);

    type Output = i64;

    fn input() -> Result<Self::Input> {
        // Ok((4, 8))
        Ok((4, 6))
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        let mut scores = vec![0, 0];
        let mut poss = vec![input.0, input.1];
        let mut dice = 1;
        let mut rolls = 0;

        let mut roll = || {
            let ret = dice;
            rolls += 1;
            dice = dice % 100 + 1;
            ret
        };

        for i in 0.. {
            let p = i % 2;
            let movs = roll() + roll() + roll();
            poss[p] = (poss[p] - 1 + movs) % 10 + 1;
            scores[p] += poss[p];

            if scores[p] >= 1000 {
                return rolls * scores[p ^ 1];
            }
        }

        unreachable!()
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let (a, b) = solve(0, &[input.0, input.1], &[0, 0]);
        max(a, b) as i64
    }
}

#[memoise::memoise(phase, poss[0], poss[1], scores[0], scores[1])]
fn solve(phase: usize, poss: &[i64], scores: &[i64]) -> (usize, usize) {
    let p = phase / 4;

    if phase % 4 == 3 {
        let mut nscores = [scores[0], scores[1]];
        nscores[p] += poss[p];

        if nscores[0] >= 21 {
            return (1, 0);
        } else if nscores[1] >= 21 {
            return (0, 1);
        }

        solve((phase + 1) % 8, poss, &nscores)
    } else {
        let mut ret = (0, 0);

        for d in 1..=3 {
            let mut nposs = [poss[0], poss[1]];
            nposs[p] = (poss[p] - 1 + d) % 10 + 1;
            let t = solve(phase + 1, &nposs, scores);
            ret.0 += t.0;
            ret.1 += t.1;
        }

        ret
    }
}
