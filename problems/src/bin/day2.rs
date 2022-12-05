use std::time::Instant;

use bstr::BString;
use problems::load;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Rps {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

type Parsed = Vec<(Rps, Rps)>;

fn parsing(input: &BString) -> color_eyre::Result<Parsed> {
    Ok(input
        .split(|&b| b == b'\n')
        .filter(|&v| !v.is_empty())
        .map(|v| v.split(|&b| b == b' '))
        .flat_map(|mut v| -> Option<_> { Some((v.next()?, v.next()?)) })
        .flat_map(|(r, l)| {
            Some((
                match r[0] {
                    b'A' => Some(Rps::Rock),
                    b'B' => Some(Rps::Paper),
                    b'C' => Some(Rps::Scissors),

                    _ => None,
                }?,
                match l[0] {
                    b'X' => Some(Rps::Rock),
                    b'Y' => Some(Rps::Paper),
                    b'Z' => Some(Rps::Scissors),
                    _ => None,
                }?,
            ))
        })
        .collect::<Vec<_>>())
}

fn part1(input: Parsed) {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum MatchResult {
        Win(u8),
        Loss(u8),
        Draw(u8),
    }

    let result = input
        .into_iter()
        .map(|m| match m {
            (Rps::Scissors, Rps::Rock) => MatchResult::Win(1),
            (Rps::Rock, Rps::Paper) => MatchResult::Win(2),
            (Rps::Paper, Rps::Scissors) => MatchResult::Win(3),
            (rhs, lhs) if rhs == lhs => MatchResult::Draw(lhs as u8),
            (_, lhs) => MatchResult::Loss(lhs as u8),
        })
        .map(|res| match res {
            MatchResult::Loss(n) => n,
            MatchResult::Win(n) => 6 + n,
            MatchResult::Draw(n) => 3 + n,
        })
        .map(|r| r as u64)
        .sum::<u64>();
    println!("Part 1: {result}");
}

fn part2(input: Parsed) {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum MatchResult {
        Win(u8),
        Loss(u8),
        Draw(u8),
    }
    #[derive(Clone, Debug, Copy, PartialEq, Eq)]
    enum PredicatedResult {
        Win,
        Draw,
        Loss,
    }

    fn create_guess(lhs: Rps, res: PredicatedResult) -> Rps {
        match lhs {
            Rps::Rock => match res {
                PredicatedResult::Win => Rps::Paper,
                PredicatedResult::Draw => Rps::Rock,
                PredicatedResult::Loss => Rps::Scissors,
            },
            Rps::Paper => match res {
                PredicatedResult::Win => Rps::Scissors,
                PredicatedResult::Draw => Rps::Paper,
                PredicatedResult::Loss => Rps::Rock,
            },
            Rps::Scissors => match res {
                PredicatedResult::Win => Rps::Rock,
                PredicatedResult::Draw => Rps::Scissors,
                PredicatedResult::Loss => Rps::Paper,
            },
        }
    }

    let result = input
        .into_iter()
        .map(|(lhs, predicated_result)| {
            (
                lhs,
                match predicated_result {
                    Rps::Rock => PredicatedResult::Loss,
                    Rps::Paper => PredicatedResult::Draw,
                    Rps::Scissors => PredicatedResult::Win,
                },
            )
        })
        .map(|(lhs, res)| (lhs, create_guess(lhs, res)))
        .map(|m| match m {
            (Rps::Scissors, Rps::Rock) => MatchResult::Win(1),
            (Rps::Rock, Rps::Paper) => MatchResult::Win(2),
            (Rps::Paper, Rps::Scissors) => MatchResult::Win(3),
            (rhs, lhs) if rhs == lhs => MatchResult::Draw(lhs as u8),
            (_, lhs) => MatchResult::Loss(lhs as u8),
        })
        .map(|res| match res {
            MatchResult::Loss(n) => n,
            MatchResult::Win(n) => 6 + n,
            MatchResult::Draw(n) => 3 + n,
        })
        .map(|r| r as u64)
        .sum::<u64>();
    println!("Part 2: {result}");
}

fn main() -> color_eyre::Result<()> {
    let context = load(1)?;

    let start = Instant::now();
    let parsed = parsing(&context.input)?;
    let elapsed = humantime::format_duration(start.elapsed());

    let start = Instant::now();
    if context.part == 1 {
        part1(parsed);
    } else {
        part2(parsed);
    }
    let elapsed_part = humantime::format_duration(start.elapsed());

    println!("  Parsing: {elapsed}");
    println!("  Solving: {elapsed_part}");

    Ok(())
}
