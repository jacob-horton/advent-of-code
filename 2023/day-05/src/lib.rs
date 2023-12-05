use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    multi::separated_list0,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone)]
pub struct MappingLine {
    pub source_start: u64,
    pub dest_start: u64,
    pub range_length: u64,
}

#[derive(Debug, Clone)]
pub struct Mapping<'a> {
    pub from: &'a str,
    pub to: &'a str,

    pub mappings: Vec<MappingLine>,
}

impl<'a> Mapping<'a> {
    pub fn map_value(&self, num: u64) -> u64 {
        for mapping in &self.mappings {
            if num >= mapping.source_start && num < mapping.source_start + mapping.range_length {
                return mapping.dest_start + (num - mapping.source_start);
            }
        }

        num
    }

    pub fn reverse_map_value(&self, num: u64) -> u64 {
        for mapping in &self.mappings {
            if num >= mapping.dest_start && num < mapping.dest_start + mapping.range_length {
                return mapping.source_start + (num - mapping.dest_start);
            }
        }

        num
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("seeds: "), separated_list0(tag(" "), complete::u64))(input)
}

// Returns (from, to)
fn parse_mapping_first_line(input: &str) -> IResult<&str, (&str, &str)> {
    terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:\n"))(input)
}

fn parse_mapping_line(input: &str) -> IResult<&str, MappingLine> {
    let (input, (dest, _, src, _, len)) = tuple((
        complete::u64,
        tag(" "),
        complete::u64,
        tag(" "),
        complete::u64,
    ))(input)?;

    Ok((
        input,
        MappingLine {
            source_start: src,
            dest_start: dest,
            range_length: len,
        },
    ))
}

fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
    let (input, (from, to)) = parse_mapping_first_line(input)?;
    let (input, mappings) = separated_list0(newline, parse_mapping_line)(input)?;

    Ok((input, Mapping { from, to, mappings }))
}

pub fn parse_input(input: &str) -> (Vec<u64>, Vec<Mapping>) {
    let (input, seeds) = parse_seeds(input).unwrap();
    let (_, mappings) =
        preceded(tag("\n\n"), separated_list0(tag("\n\n"), parse_mapping))(input).unwrap();

    (seeds, mappings)
}
