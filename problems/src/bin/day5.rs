#![feature(type_alias_impl_trait)]
use std::{rc::Rc, time::Instant};

use bstr::{BString, ByteSlice};
use color_eyre::eyre::{bail, Context};
use problems::load;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Move {
    from: u32,
    to: u32,
    number: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Board {
    inner: Vec<Vec<char>>,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, col) in self.inner.iter().enumerate() {
            let index = index + 1;
            writeln!(f, "{index} {col:?}")?;
        }
        Ok(())
    }
}

impl std::str::FromStr for Board {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();
        let mut board_inner = lines
            .iter()
            .rev()
            .skip(1)
            .map(|line| line.trim())
            .map(|line| line.split_whitespace())
            .map(|line| line.map(|word| word.trim_matches(&['[', ']'][..])))
            .map(|line| line.map(|word| word.chars().next().unwrap()))
            .map(|line| line.collect::<Vec<char>>())
            .collect::<Vec<_>>();

        let max_length = board_inner.iter().map(Vec::len).max().unwrap_or(0);
        for row in &mut board_inner {
            while row.len() < max_length {
                row.push('0')
            }
        }
        board_inner.reverse();
        let new_width = board_inner.len() - 1;
        let new_height = board_inner[0].len() - 1;
        let mut new_board = vec![vec!['0'; board_inner.len()]; board_inner[0].len()];

        for row in 0..board_inner.len() {
            for col in 0..board_inner[0].len() {
                new_board[new_height - col][new_width - row] = board_inner[row][col];
            }
        }

        new_board.reverse();
        for line in &mut new_board {
            while let Some('0') = line.last() {
                line.pop();
            }
        }
        Ok(Board { inner: new_board })
    }
}

impl std::str::FromStr for Move {
    type Err = color_eyre::eyre::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let Some("move") = words.next() else {bail!("Invalid move: missing 'move' word")};
        let Some(Ok(num)) = words.next().map(str::parse::<u32>) else {bail!("Invalid move: missing number of move")};
        let Some("from") = words.next() else {bail!("Invalid move: missing 'from' word")};
        let Some(Ok(start)) = words.next().map(str::parse::<u32>) else {bail!("Invalid move: missing start number")};
        let Some("to") = words.next() else {bail!("Invalid move: missing 'to' word")};
        let Some(Ok(end)) = words.next().map(str::parse::<u32>) else {bail!("Invalid move: missing end number")};
        Ok(Move {
            number: num,
            from: start,
            to: end,
        })
    }
}

type Parsed<'a> = (Board, impl Iterator<Item = Move> + 'a);

fn parsing(input: &BString) -> color_eyre::Result<Parsed<'_>> {
    let Some((board, move_)) = input.split_once_str("\n\n") else {color_eyre::eyre::bail!("Error when splitting at two newlines")};
    let mut zeros = Vec::new();
    let mut last_line = false;
    for line in board.lines().rev() {
        let mut chars = line.chars();

        if let Some(' ') = chars.next() {
            zeros.push(4);
            chars.next();
            chars.next();
            chars.next();
        }

        if let Some(' ') = chars.clone().rev().next() {
            last_line = true;
            chars.next();
            chars.next();
            chars.next();
            chars.next();
        }
        for _ in 1..(chars.count() / 5) {
            zeros.push(5);
        }
        if last_line {
            zeros.push(4);
        }
    }
    let mut board = board.to_vec();
    let buffer = String::from("     ");
    for zero in zeros {
        board = board.replacen(&buffer[0..zero], " [0] ", 1);
    }

    let board = str::parse::<Board>(
        board
            .to_str()
            .map_err(|_| color_eyre::eyre::eyre!("Error when parsing board"))?,
    )
    .map_err(|_| color_eyre::eyre::eyre!("Error when parsing board"))?;

    Ok((
        board,
        move_
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| -> color_eyre::Result<Move> {
                str::parse::<Move>(l.to_str().wrap_err("unable to parse move")?)
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter(),
    ))
}

fn part1((mut board, moves): Parsed<'_>) {
    for move_ in moves.flat_map(|m| std::iter::repeat(m).take(m.number as usize)) {
        let move_: Move = move_;
        let Some(elem) = board.inner.get_mut(move_.from as usize -1).and_then(|r| r.pop()) else {continue;};
        if let Some(r) = board.inner.get_mut(move_.to as usize - 1) {
            r.push(elem);
        }
    }
    print!("Part 1: ");
    for row in &board.inner {
        let Some(elem) = row.last() else {continue};
        print!("{elem}");
    }
    println!();
}

fn part2((mut board, moves): Parsed<'_>) {
    for move_ in moves {
        let move_: Move = move_;
        let to_add = {
            let num = move_.number;
            let mut out = Vec::with_capacity(move_.number as usize);
            for _ in 0..num {
                let Some(o) = board.inner[move_.from as usize - 1].pop() else {continue};
                out.push(o);
            }
            out.reverse();
            out
        };
        board.inner[move_.to as usize - 1].extend(to_add);

        println!("{board}");
    }
    print!("Part 2: ");
    for col in &board.inner {
        let Some(elem) = col.last() else {continue};
        print!("{elem}");
    }
    println!();
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part2() {
        const INPUT: &str = r"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let input = BString::from(&INPUT[1..]);
        let parsed = parsing(&input).unwrap();
        println!("{}", parsed.0);
        part2(parsed);
        panic!();
    }
}
