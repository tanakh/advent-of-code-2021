use anyhow::Result;
use std::{fmt::Display, io::Read};

pub trait AOC {
    type Input;
    type Output: Display;

    fn input() -> Result<Self::Input>;
    fn part_one(input: &Self::Input) -> Self::Output;
    fn part_two(input: &Self::Input) -> Self::Output;
}

pub fn run<T: AOC>() -> Result<()> {
    let input = T::input()?;
    println!("Part One: {}", T::part_one(&input));
    println!("Part Two: {}", T::part_two(&input));

    Ok(())
}

#[macro_export]
macro_rules! aoc {
    ($t:ident) => {
        use anyhow::Result;
        use std::io::Read;

        fn main() -> Result<()> {
            advent_of_code_2021::run::<$t>()
        }

        struct $t;
    };
}

pub fn get_contents() -> Result<String> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(input)
}
