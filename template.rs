#![feature(type_alias_impl_trait)]
use std::time::Instant;

use bstr::{BString, ByteSlice};
use problems::load;

type Parsed<'a> = impl Iterator<Item = ()> + 'a;

fn parsing(input: &BString) -> color_eyre::Result<Parsed<'_>> {
    todo!("Parsing")
}

fn part1(input: Parsed<'_>) {
    todo!("todo part1")
}

fn part2(input: Parsed<'_>) {
    todo!("todo part2")
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
