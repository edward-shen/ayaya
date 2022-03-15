use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, u8};
use nom::combinator::all_consuming;
use nom::multi::fold_many1;
use nom::IResult;

#[derive(Parser, Debug)]
struct Args {
    input: PathBuf,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Ansi {
    Character { color: Color, character: char },
    Newline,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Color {
    Inherit,
    Value { foreground: u8, background: u8 },
}

fn main() -> Result<()> {
    let args = Args::parse();
    let bytes = std::fs::read_to_string(args.input)?;
    let data = parse(&bytes);
    // remove
    let data: Vec<_> = data
        .into_iter()
        .scan(None, |state, ansi| match (state.as_mut(), ansi) {
            (None, Ansi::Character { color, .. }) => {
                *state = Some(color);
                Some(ansi)
            }
            (_, Ansi::Newline) => {
                *state = None;
                Some(ansi)
            }
            (Some(prev_color), Ansi::Character { color, character }) => {
                if *prev_color == color {
                    Some(Ansi::Character {
                        color: Color::Inherit,
                        character,
                    })
                } else {
                    *prev_color = color;
                    Some(ansi)
                }
            }
        })
        .collect();

    let color_mapping: BTreeSet<_> = data
        .iter()
        .copied()
        .filter_map(|item| match item {
            Ansi::Character {
                color:
                    Color::Value {
                        foreground,
                        background,
                    },
                ..
            } => Some((foreground, background)),
            _ => None,
        })
        .collect();

    let rev_color_mapping: HashMap<_, _> = color_mapping
        .iter()
        .enumerate()
        .map(|(k, v)| (*v, k as u8))
        .collect();

    let char_mapping: BTreeSet<_> = data
        .iter()
        .copied()
        .filter_map(|item| match item {
            Ansi::Character { character, .. } => Some(character),
            _ => None,
        })
        .collect();

    let rev_char_mapping: HashMap<_, _> = char_mapping
        .iter()
        .enumerate()
        .map(|(k, v)| (*v, k as u8))
        .collect();

    let color_map_contents = format!(
        "pub const COLOR_MAPPING: [(u8, u8); {}] = [\n    {},\n];",
        color_mapping.len(),
        color_mapping
            .into_iter()
            .map(|(fg, bg)| format!("({fg}, {bg})"))
            .collect::<Vec<_>>()
            .join(",\n    ")
    );

    let char_map_contents = format!(
        "pub const CHAR_MAPPING: [char; {}] = [\n    {},\n];",
        char_mapping.len(),
        char_mapping
            .into_iter()
            .map(|v| format!("'{v}'"))
            .collect::<Vec<_>>()
            .join(",\n    ")
    );

    std::fs::write(
        "mapping.rs",
        [color_map_contents, char_map_contents].join("\n"),
    )?;

    let data_file_contents =
        data.iter()
            .fold(Vec::with_capacity(data.len() * 3), |mut accum, value| {
                match value {
                    Ansi::Character { color, character } => match color {
                        Color::Inherit => {
                            accum.push(0xfb + *rev_char_mapping.get(character).unwrap());
                        }
                        Color::Value {
                            foreground,
                            background,
                        } => {
                            accum
                                .push(*rev_color_mapping.get(&(*foreground, *background)).unwrap());
                            accum.push(*rev_char_mapping.get(character).unwrap());
                        }
                    },
                    Ansi::Newline => {
                        accum.push(0xff);
                    }
                }
                accum
            });

    std::fs::write("data", data_file_contents)?;
    Ok(())
}

fn parse(data: &str) -> Vec<Ansi> {
    all_consuming(image)(data).map(|(_, data)| data).unwrap()
}

fn image(i: &str) -> IResult<&str, Vec<Ansi>> {
    fold_many1(ansi, Vec::new, |mut acc, item| {
        acc.push(item);
        acc
    })(i)
}

fn ansi(i: &str) -> IResult<&str, Ansi> {
    alt((ansi_char, ansi_newline))(i)
}

fn ansi_char(i: &str) -> IResult<&str, Ansi> {
    let (i, color) = ansi_color(i)?;
    let (i, character) = anychar(i)?;
    Ok((i, Ansi::Character { color, character }))
}

fn ansi_color(i: &str) -> IResult<&str, Color> {
    let (i, _) = ansi_escape(i)?;
    let (i, _) = tag("38;5;")(i)?;
    let (i, foreground) = u8(i)?;
    let (i, _) = tag(";48;5;")(i)?;
    let (i, background) = u8(i)?;
    let (i, _) = char('m')(i)?;
    Ok((
        i,
        Color::Value {
            foreground,
            background,
        },
    ))
}

fn ansi_newline(i: &str) -> IResult<&str, Ansi> {
    let (i, _) = ansi_escape(i)?;
    let (i, _) = tag("0m\n")(i)?;
    Ok((i, Ansi::Newline))
}

fn ansi_escape(i: &str) -> IResult<&str, ()> {
    let (i, _) = tag("\u{1b}[")(i)?;
    Ok((i, ()))
}
