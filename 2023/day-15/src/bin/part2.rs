use nom::{
    branch::alt,
    character::{self, complete::alpha1},
};

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn hash(val: &str) -> u32 {
    let mut hash = 0;
    for ascii in val.bytes() {
        hash += ascii as u32;
        hash *= 17;
        hash %= 256;
    }

    hash
}

#[derive(Debug, Clone)]
enum Operation {
    Equals(u32),
    Dash,
}

fn process_letters(input: &str) -> (&str, u32, Operation) {
    let (input, label) = alpha1::<&str, ()>(input).unwrap();
    let (input, op) = alt((
        character::complete::char::<&str, ()>('='),
        character::complete::char::<&str, ()>('-'),
    ))(input)
    .unwrap();

    let operation = match op {
        '=' => {
            let (_, focal_length): (_, u32) = character::complete::u32::<&str, ()>(input).unwrap();
            Operation::Equals(focal_length)
        }
        '-' => Operation::Dash,
        _ => panic!("Invalid operation"),
    };

    let hash = hash(label);

    (label, hash, operation)
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u32,
}

fn process(input: &str) -> u32 {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    for letters in input.trim().split(',') {
        let (label, hash, op) = process_letters(letters);

        match op {
            Operation::Dash => {
                boxes[hash as usize].retain(|l| l.label != label);
            }
            Operation::Equals(f) => {
                let existing = boxes[hash as usize].iter().position(|l| l.label == label);
                match existing {
                    Some(i) => boxes[hash as usize][i].focal_length = f,
                    None => boxes[hash as usize].push(Lens {
                        label: label.to_string(),
                        focal_length: f,
                    }),
                };
            }
        }
    }

    let mut sum = 0;
    for (i, lens_box) in boxes.into_iter().enumerate() {
        for (j, lens) in lens_box.into_iter().enumerate() {
            sum += (i + 1) as u32 * (j + 1) as u32 * lens.focal_length;
        }
    }

    sum
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input);
        assert_eq!(result, 145);
    }
}
