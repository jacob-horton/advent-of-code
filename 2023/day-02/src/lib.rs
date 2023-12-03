use std::{iter::Sum, ops};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace0, newline},
    multi::separated_list0,
    sequence::preceded,
    IResult,
};

#[derive(Debug, Clone)]
pub struct Game {
    pub id: u32,
    pub subsets: Vec<Subset>,
}

#[derive(Default, Debug, Clone)]
pub struct Subset {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Sum for Subset {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, s| Subset::piecewise_max(acc, s))
            .unwrap_or(Subset::default())
    }
}

impl ops::Add<Subset> for Subset {
    type Output = Subset;

    fn add(self, _rhs: Subset) -> Subset {
        Subset {
            red: self.red + _rhs.red,
            green: self.green + _rhs.green,
            blue: self.blue + _rhs.blue,
        }
    }
}

impl Subset {
    pub fn piecewise_max(a: Subset, b: Subset) -> Subset {
        Subset {
            red: u32::max(a.red, b.red),
            green: u32::max(a.green, b.green),
            blue: u32::max(a.blue, b.blue),
        }
    }
}

fn parse_number_cubes(input: &str) -> IResult<&str, Subset> {
    let (input, amt) = complete::u32(input)?;
    let (input, _) = multispace0(input)?;
    let (input, colour) = alt((tag("red"), tag("green"), tag("blue")))(input)?;

    let subset = match colour {
        "red" => Subset {
            red: amt,
            ..Default::default()
        },
        "green" => Subset {
            green: amt,
            ..Default::default()
        },
        "blue" => Subset {
            blue: amt,
            ..Default::default()
        },
        _ => panic!("Invalid colour"),
    };

    Ok((input, subset))
}

fn parse_subset(input: &str) -> IResult<&str, Subset> {
    let (input, cubes) = separated_list0(tag(", "), parse_number_cubes)(input)?;
    let subset = cubes.into_iter().sum();

    Ok((input, subset))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), complete::u32)(input)?;
    let (input, subsets) = preceded(tag(": "), separated_list0(tag("; "), parse_subset))(input)?;

    Ok((input, Game { id, subsets }))
}

pub fn parse_input(input: &str) -> Vec<Game> {
    let (_, games) = separated_list0(newline, parse_game)(input).expect("Error parsing");
    games
}
