use advent_of_code_2021::*;

aoc!(Day);

impl AOC for Day {
    type Input = Vec<(String, i64)>;

    type Output = i64;

    fn input() -> Result<Self::Input> {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s)?;
        Ok(s.lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let name = parts.next().unwrap().to_string();
                let value = parts.next().unwrap().parse::<i64>().unwrap();
                (name, value)
            })
            .collect())
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        let mut x = 0;
        let mut y = 0;

        for (name, value) in input {
            match name.as_str() {
                "forward" => x += value,
                "down" => y += value,
                "up" => y -= value,
                _ => unreachable!(),
            }
        }

        x * y
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let mut x = 0;
        let mut y = 0;
        let mut aim = 0;

        for (name, value) in input {
            match name.as_str() {
                "forward" => {
                    x += value;
                    y += value * aim;
                }
                "down" => aim += value,
                "up" => aim -= value,
                _ => unreachable!(),
            }
        }

        x * y
    }
}
