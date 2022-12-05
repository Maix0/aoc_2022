#![feature(type_alias_impl_trait)]
use std::{ops::RangeInclusive, time::Instant};

use bstr::{BString, ByteSlice};
use problems::load;

type Parsed<'a> = impl Iterator<Item = [RangeInclusive<u32>; 2]> + 'a;

fn parsing(input: &BString) -> color_eyre::Result<Parsed<'_>> {
    Ok(input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.split_once_str(",").unwrap())
        .map(|(first, second)| [first, second])
        .map(|arr| arr.map(|s: &[u8]| s.split_once_str("-").unwrap()))
        .map(|arr| arr.map(|s: (&[u8], &[u8])| (s.0.to_str().unwrap(), s.1.to_str().unwrap())))
        .map(|arr| {
            arr.map(|(start, end)| {
                (
                    str::parse::<u32>(start).unwrap(),
                    str::parse::<u32>(end).unwrap(),
                )
            })
        })
        .map(|arr| arr.map(|(start, end)| start..=end)))
}

fn part1(input: Parsed<'_>) {
    let result = input
        .filter(|[r1, r2]| {
            (r1.contains(r2.start()) && r1.contains(r2.end()))
                || (r2.contains(r1.start()) && r2.contains(r1.end()))
        })
        .count();
    println!("Part 1: {result}");
}

fn part2(input: Parsed<'_>) {
    let result = input
        .filter(|[r1, r2]| {
            r1.contains(r2.start())
                || r1.contains(r2.end())
                || r2.contains(r1.start())
                || r2.contains(r1.end())
        })
        .count();
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
