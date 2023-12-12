use day_12::Spring;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
struct Run {
    start: u32,
    length: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ImportantRuns {
    damaged: Vec<Run>,
    unknown: Vec<Run>,
}

fn main() {
    let input = include_str!("../inputs/input.txt");
    // let input = include_str!("../inputs/test_part1.txt");
    let result = process(input);
    println!("{result}");
}

fn parse_line(line: &str) -> (Vec<Spring>, Vec<u32>) {
    let (partial_springs_str, contiguous_damaged_str) = line.split_once(' ').unwrap();
    let partial_springs = partial_springs_str
        .chars()
        .map(|c| match c {
            '?' => Spring::Unknown,
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            _ => panic!("Unknown spring type"),
        })
        .collect();

    let contiguous_damaged = contiguous_damaged_str
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    (partial_springs, contiguous_damaged)
}

fn get_runs(springs: Vec<Spring>) -> ImportantRuns {
    let mut damaged = Vec::new();
    let mut unknown = Vec::new();

    let mut run_type = springs[0];
    let mut run_start = 0;

    let spring_len = springs.len() as u32;
    for (i, s) in springs.into_iter().enumerate().skip(1) {
        if s != run_type {
            match run_type {
                Spring::Damaged => damaged.push(Run {
                    start: run_start,
                    length: i as u32 - run_start,
                }),
                Spring::Unknown => unknown.push(Run {
                    start: run_start,
                    length: i as u32 - run_start,
                }),
                _ => (),
            }

            run_start = i as u32;
            run_type = s;
        }
    }

    match run_type {
        Spring::Damaged => damaged.push(Run {
            start: run_start,
            length: spring_len - run_start,
        }),
        Spring::Unknown => unknown.push(Run {
            start: run_start,
            length: spring_len - run_start,
        }),
        _ => (),
    }

    ImportantRuns { damaged, unknown }
}

fn unfold(
    mut partial_springs: Vec<Spring>,
    contiguous_damaged: Vec<u32>,
) -> (Vec<Spring>, Vec<u32>) {
    partial_springs.push(Spring::Unknown);

    (
        partial_springs
            .repeat(5)
            .into_iter()
            .take(partial_springs.len() * 5 - 1)
            .collect(),
        contiguous_damaged.repeat(5),
    )
}

fn is_possible(runs: &ImportantRuns, contiguous_damaged: &Vec<u32>) -> bool {
    let max_len = contiguous_damaged.iter().max().unwrap();
    if runs.damaged.iter().any(|r| r.length > *max_len) {
        return false;
    }

    if !runs.damaged.is_empty()
        && !runs.unknown.is_empty()
        && runs.damaged[0].start < runs.unknown[0].start
        && runs.damaged[0].length > contiguous_damaged[0]
    {
        return false;
    }

    true
}

fn is_valid(damaged_runs: Vec<Run>, contiguous_damaged: &Vec<u32>) -> bool {
    damaged_runs.len() == contiguous_damaged.len()
        && damaged_runs
            .into_iter()
            .zip(contiguous_damaged)
            .all(|(r, len)| r.length == *len)
}

// TODO: existing runs as binary tree?
// Only works if no overlaps
// Also existing_runs must already be sorted
fn add_run(mut existing_runs: Vec<Run>, mut new_run: Run) -> Vec<Run> {
    let adjacent_before = existing_runs
        .iter()
        .position(|r| r.start + r.length == new_run.start)
        .map(|idx| existing_runs[idx]);

    let adjacent_after = existing_runs
        .iter()
        .position(|r| r.start == new_run.start + new_run.length)
        .map(|idx| existing_runs[idx]);

    if let Some(adjacent_before) = adjacent_before {
        existing_runs.retain(|r| r != &adjacent_before);
        new_run = Run {
            start: adjacent_before.start,
            length: adjacent_before.length + new_run.length,
        };
    }

    if let Some(adjacent_after) = adjacent_after {
        existing_runs.retain(|r| r != &adjacent_after);
        new_run = Run {
            start: new_run.start,
            length: new_run.length + adjacent_after.length,
        };
    }

    existing_runs.push(new_run);
    existing_runs.sort_by(|r1, r2| r1.start.cmp(&r2.start));
    existing_runs
}

fn print_row(runs: ImportantRuns) {
    let mut row = vec![
        '.';
        u32::max(
            runs.damaged
                .iter()
                .map(|r| r.start + r.length)
                .max()
                .unwrap_or(0),
            runs.unknown
                .iter()
                .map(|r| r.start + r.length)
                .max()
                .unwrap_or(0),
        ) as usize
    ];

    for run in runs.damaged {
        for i in run.start..run.start + run.length {
            row[i as usize] = '#';
        }
    }

    for run in runs.unknown {
        for i in run.start..run.start + run.length {
            row[i as usize] = '?';
        }
    }

    println!("{}", row.into_iter().collect::<String>());
}

fn get_arrangements(runs: ImportantRuns, contiguous_damaged: &Vec<u32>) -> u32 {
    // print_row(runs.clone());

    if !is_possible(&runs, &contiguous_damaged) {
        return 0;
    }

    if is_valid(runs.damaged.clone(), &contiguous_damaged) {
        return 1;
    }
    if runs.unknown.len() == 0 {
        return 0;
    }

    let mut sum = 0;
    for (idx, unknown) in runs.unknown.iter().enumerate() {
        for i in 0..unknown.length {
            let mut new_unknown: Vec<_> = runs.unknown[(idx as usize)..]
                .to_owned()
                .into_iter()
                .filter(|r| r != unknown)
                .collect();

            let length = unknown.length - i - 1;
            if length != 0 {
                new_unknown.push(Run {
                    start: unknown.start + i + 1,
                    length,
                });
            }

            let new_damaged = add_run(
                runs.damaged.clone(),
                Run {
                    start: unknown.start + i,
                    length: 1,
                },
            );

            let new_runs = ImportantRuns {
                unknown: new_unknown,
                damaged: new_damaged,
            };

            sum += get_arrangements(new_runs, &contiguous_damaged);
        }
    }

    sum
}

fn process(input: &str) -> u32 {
    let mut arrangements = 0;
    for line in input.split('\n') {
        if line.trim().is_empty() {
            continue;
        }

        let (partial_springs, contiguous_damaged) = parse_line(line);
        let (partial_springs, contiguous_damaged) = unfold(partial_springs, contiguous_damaged);
        // println!("{partial_springs:?} {contiguous_damaged:?}\n");

        let runs = get_runs(partial_springs);
        let a = get_arrangements(runs, &contiguous_damaged);
        arrangements += a;
    }

    arrangements
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input);
        assert_eq!(result, 525152);
    }
}
