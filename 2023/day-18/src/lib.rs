use nom::{
    branch::alt,
    character::complete::{self, char, hex_digit1, newline},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug, Clone)]
pub struct Instruction {
    pub dir: (i64, i64),
    pub distance: i64,
}

fn parse_dir(input: &str) -> IResult<&str, (i64, i64)> {
    let (input, dir) = alt((char('R'), char('L'), char('D'), char('U')))(input)?;

    let dir = match dir {
        'U' => (0, -1),
        'D' => (0, 1),
        'L' => (-1, 0),
        'R' => (1, 0),
        _ => panic!("Unknown direction"),
    };

    Ok((input, dir))
}

fn parse_colour(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, hex)) = delimited(char('('), tuple((char('#'), hex_digit1)), char(')'))(input)?;
    let distance = hex.chars().take(5);
    let dir = hex.chars().last().unwrap();

    let distance = i64::from_str_radix(&distance.collect::<String>(), 16).unwrap();
    let dir = match dir {
        '0' => (1, 0),
        '1' => (0, 1),
        '2' => (-1, 0),
        '3' => (0, -1),
        _ => panic!("Unknown direction"),
    };

    Ok((input, Instruction { dir, distance }))
}

fn parse_line_part1(input: &str) -> IResult<&str, Instruction> {
    let (input, (dir, _, distance, _, _)) =
        tuple((parse_dir, char(' '), complete::i64, char(' '), parse_colour))(input)?;

    Ok((input, Instruction { dir, distance }))
}

pub fn parse_input_part1(input: &str) -> Vec<Instruction> {
    let (_, instructions) = separated_list1(newline, parse_line_part1)(input).unwrap();
    instructions
}

fn parse_line_part2(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, _, _, _, instruction)) =
        tuple((parse_dir, char(' '), complete::i64, char(' '), parse_colour))(input)?;

    Ok((input, instruction))
}

pub fn parse_input_part2(input: &str) -> Vec<Instruction> {
    let (_, instructions) = separated_list1(newline, parse_line_part2)(input).unwrap();
    instructions
}
