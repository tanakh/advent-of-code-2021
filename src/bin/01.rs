use advent_of_code_2021::*;

aoc!(Day);

impl AOC for Day {
    type Input = Vec<i64>;
    type Output = usize;

    fn input() -> Result<Self::Input> {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s)?;

        let xs = s
            .lines()
            .map(|w| w.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        Ok(xs)
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        let mut ans = 0;
        for i in 1..input.len() {
            if input[i - 1] < input[i] {
                ans += 1;
            }
        }
        ans
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let xxs = input
            .windows(3)
            .map(|w| w.iter().sum::<i64>())
            .collect::<Vec<_>>();

        let mut ans = 0;
        for i in 1..xxs.len() {
            if xxs[i - 1] < xxs[i] {
                ans += 1;
            }
        }

        ans
    }
}
