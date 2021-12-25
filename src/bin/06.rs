use advent_of_code_2021::*;

aoc!(Day);

impl AOC for Day {
    type Input = Vec<i64>;

    type Output = usize;

    fn input() -> Result<Self::Input> {
        let c = get_contents()?;
        let ret = c
            .trim()
            .split(',')
            .map(|w| w.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        Ok(ret)
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        solve(input, 80)
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        solve(input, 256)
    }
}

fn solve(init: &[i64], steps: usize) -> usize {
    let mut state = [0; 10];

    for &v in init {
        state[v as usize] += 1;
    }

    for _ in 0..steps {
        let mut new_state = [0; 10];

        new_state[6] += state[0];
        new_state[8] += state[0];

        for i in 1..10 {
            new_state[(i - 1) as usize] += state[i];
        }

        state = new_state;
    }

    state.iter().sum()
}
