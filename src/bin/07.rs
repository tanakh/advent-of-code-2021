use advent_of_code_2021::*;

aoc!(Day);

impl AOC for Day {
    type Input = Vec<i64>;

    type Output = i64;

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
        (0..*input.iter().max().unwrap())
            .map(|c| input.iter().map(|d| (c - d).abs()).sum())
            .min()
            .unwrap()
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        (0..*input.iter().max().unwrap())
            .map(|c| {
                input
                    .iter()
                    .map(|d| {
                        let x = (c - d).abs();
                        x * (x + 1) / 2
                    })
                    .sum()
            })
            .min()
            .unwrap()
    }
}
