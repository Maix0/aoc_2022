#![feature(type_alias_impl_trait)]
use std::time::Instant;

use bstr::{BString, ByteSlice};
use problems::load;

type Parsed<'a> = impl Iterator<Item = &'a bstr::BStr>;

fn parsing(input: &BString) -> color_eyre::Result<Parsed<'_>> {
    Ok(input.lines().map(bstr::BStr::new))
}

fn part1(input: Parsed<'_>) {
    let sum = input
        .map(|v: &bstr::BStr| v.split_at(v.len() / 2))
        .map(
            |(first, second)| -> (
                std::collections::HashSet<char>,
                std::collections::HashSet<char>,
            ) {
                (
                    std::collections::HashSet::from_iter(
                        first.iter().filter_map(|c| char::from_u32(*c as u32)),
                    ),
                    std::collections::HashSet::from_iter(
                        second.iter().filter_map(|c| char::from_u32(*c as u32)),
                    ),
                )
            },
        )
        .flat_map(|(first, second)| {
            first
                .intersection(&second)
                .copied()
                .collect::<Vec<_>>()
                .into_iter()
        })
        .map(|c| match c {
            c @ 'A'..='Z' => (c as u32) - ('A' as u32) + 27,
            c @ 'a'..='z' => (c as u32) - ('a' as u32) + 1,
            _ => 0,
        })
        .sum::<u32>();
    println!("Part 1: {sum}");
}

fn part2(input: Parsed<'_>) {
    let sum = input
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|slice| [slice[0], slice[1], slice[2]])
        .map(|arr| arr.map(|s| s.iter()))
        .map(|arr| arr.map(|i| i.copied().map(Into::into).filter_map(char::from_u32)))
        .map(|arr| -> [std::collections::HashSet<char>; 3] {
            arr.map(std::collections::HashSet::from_iter)
        })
        .flat_map(|[h1, h2, h3]| {
            h1.intersection(&h2)
                .copied()
                .collect::<std::collections::HashSet<_>>()
                .intersection(&h3)
                .copied()
                .collect::<Vec<_>>()
                .into_iter()
        })
        .map(|c| match c {
            c @ 'A'..='Z' => (c as u32) - ('A' as u32) + 27,
            c @ 'a'..='z' => (c as u32) - ('a' as u32) + 1,
            _ => 0,
        })
        .sum::<u32>();
    println!("Part 2: {sum}")
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
