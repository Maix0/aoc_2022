use std::time::Instant;

use bstr::BString;
use problems::load;

//type Parsed = ;

fn parsing(
    input: &BString,
) -> color_eyre::Result<impl Iterator<Item = impl Iterator<Item = u64> + '_> + '_> {
    Ok(std::str::from_utf8(input.as_slice())
        .unwrap()
        .split("\n\n")
        .map(|s| s.split('\n').flat_map(|v| v.parse::<u64>().ok())))
}

fn part1(input: impl Iterator<Item = impl Iterator<Item = u64>>) {
    println!(
        "part1: {}",
        input.map(|i| i.sum::<u64>()).max().unwrap_or(0)
    );
}

fn part2(input: impl Iterator<Item = impl Iterator<Item = u64>>) {
    let mut elfs_vec = input.map(|i| i.sum::<u64>()).collect::<Vec<_>>();
    elfs_vec.sort();

    println!("part2: {}", elfs_vec.iter().rev().take(3).sum::<u64>());
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
