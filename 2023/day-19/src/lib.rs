use std::collections::HashMap;

use nom::{
    branch::alt,
    character::{
        self,
        complete::{alpha1, anychar, char, newline},
    },
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

#[derive(Debug, Clone)]
pub enum Comparison {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub property: char,
    pub number: u32,
    pub comparison: Comparison,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub condition: Option<Condition>,
    pub workflow: String,
}

#[derive(Debug, Clone)]
pub struct Workflow {
    pub name: String,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct Part {
    pub properties: HashMap<char, u32>,
}

#[derive(Debug, Clone)]
pub struct System {
    pub workflows: HashMap<String, Workflow>,
    pub parts: Vec<Part>,
}

fn parse_rule_with_condition(input: &str) -> IResult<&str, Rule> {
    let (input, property) = anychar(input)?;
    let (input, comparison) = alt((char('<'), char('>')))(input)?;
    let (input, number) = character::complete::u32(input)?;
    let (input, workflow) = preceded(char(':'), alpha1)(input)?;

    let rule = Rule {
        workflow: workflow.to_string(),
        condition: Some(Condition {
            property,
            comparison: match comparison {
                '>' => Comparison::GreaterThan,
                '<' => Comparison::LessThan,
                _ => panic!("Unknown comparison operator"),
            },
            number,
        }),
    };

    Ok((input, rule))
}

fn parse_rule_without_condition(input: &str) -> IResult<&str, Rule> {
    let (input, workflow) = alpha1(input)?;

    let rule = Rule {
        workflow: workflow.to_string(),
        condition: None,
    };

    Ok((input, rule))
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = alpha1(input)?;
    let (input, rules) = delimited(
        char('{'),
        separated_list1(
            char(','),
            alt((parse_rule_with_condition, parse_rule_without_condition)),
        ),
        char('}'),
    )(input)?;

    Ok((
        input,
        Workflow {
            name: name.to_string(),
            rules,
        },
    ))
}

fn parse_workflows(input: &str) -> IResult<&str, HashMap<String, Workflow>> {
    let (input, workflow_list) = separated_list1(newline, parse_workflow)(input)?;

    let workflows = workflow_list
        .into_iter()
        .map(|w| (w.name.clone(), w))
        .collect();
    Ok((input, workflows))
}

fn parse_property(input: &str) -> IResult<&str, (char, u32)> {
    separated_pair(anychar, char('='), character::complete::u32)(input)
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, parts) = delimited(
        char('{'),
        separated_list1(char(','), parse_property),
        char('}'),
    )(input)?;

    Ok((
        input,
        Part {
            properties: parts.into_iter().collect(),
        },
    ))
}

fn parse_parts(input: &str) -> IResult<&str, Vec<Part>> {
    separated_list1(newline, parse_part)(input)
}

pub fn parse_input(input: &str) -> System {
    let (input, workflows) = parse_workflows(input).unwrap();

    // Double newline
    let (input, _) = many1(newline::<&str, ()>)(input).unwrap();

    let (_, parts) = parse_parts(input).unwrap();

    System { workflows, parts }
}
