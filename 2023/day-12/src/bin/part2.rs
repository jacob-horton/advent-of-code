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
    // let input = include_str!("../inputs/test_part2.txt");
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

// Only works if no overlaps
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

    // TODO: optimise
    existing_runs.push(new_run);
    existing_runs.sort_by(|r1, r2| r1.start.cmp(&r2.start));
    existing_runs
}

fn format_row(runs: &ImportantRuns) -> String {
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

    for run in &runs.damaged {
        for i in run.start..run.start + run.length {
            row[i as usize] = '#';
        }
    }

    for run in &runs.unknown {
        for i in run.start..run.start + run.length {
            row[i as usize] = '?';
        }
    }

    row.into_iter().collect::<String>()
}

fn get_all_possible_arrangements(
    runs: &ImportantRuns,
    contiguous_damaged: &[u32],
    max_len: u32,
) -> Vec<Vec<Run>> {
    // println!();
    // println!("Incoming row: {}", format_row(runs));
    // println!("contig {contiguous_damaged:?}");
    // println!("max_len {max_len:?}");
    if contiguous_damaged.is_empty() {
        return vec![Vec::new()];
    }

    let min_overall_len =
        contiguous_damaged.iter().sum::<u32>() + contiguous_damaged.len() as u32 - 1;

    if min_overall_len > max_len {
        return Vec::new();
    }

    if min_overall_len == max_len {
        let mut arrangement = Vec::new();
        let mut curr = 0;
        for n in contiguous_damaged {
            arrangement.push(Run {
                start: curr,
                length: *n,
            });
            curr += n + 1;
        }

        return vec![arrangement];
    }

    let joined_runs = runs
        .damaged
        .iter()
        .fold(runs.unknown.clone(), |acc, r| add_run(acc, *r));

    let mut arrangements = Vec::new();
    let length = contiguous_damaged[0];
    for i in 0..(max_len - min_overall_len) {
        let new_run = Run { start: i, length };
        if !joined_runs.iter().any(|r| contains(r, &new_run)) {
            continue;
        }

        if runs
            .damaged
            .iter()
            .filter(|r| r.start + r.length <= i)
            .any(|r| !contains(&new_run, r))
        {
            continue;
        }

        let new_runs = ImportantRuns {
            unknown: runs
                .unknown
                .clone()
                .into_iter()
                .filter(|r| r.start + r.length > length + i + 1)
                .map(|run| Run {
                    start: run
                        .start
                        .saturating_sub(length)
                        .saturating_sub(i)
                        .saturating_sub(1),
                    length: u32::min(run.length, (run.start + run.length) - length - i - 1),
                })
                .collect(),
            damaged: runs
                .damaged
                .clone()
                .into_iter()
                .filter(|r| r.start + r.length > length + i + 1)
                .map(|run| Run {
                    start: run
                        .start
                        .saturating_sub(length)
                        .saturating_sub(i)
                        .saturating_sub(1),
                    length: u32::min(run.length, (run.start + run.length) - length - i - 1),
                })
                .collect(),
        };

        // println!("{new_runs:?}");
        let mut possible_sub_arrangements = get_all_possible_arrangements(
            &new_runs,
            &contiguous_damaged[1..],
            max_len - length - i,
        );

        // println!("{possible_sub_arrangements:?}");
        possible_sub_arrangements
            .iter_mut()
            .for_each(|sub_arrangement| {
                // println!(
                //     "{}",
                //     format_row(&ImportantRuns {
                //         damaged: sub_arrangement.to_owned(),
                //         unknown: Vec::new()
                //     })
                // );
                // Add one for gap
                sub_arrangement
                    .iter_mut()
                    .for_each(|a| a.start += i + length + 1);
                sub_arrangement.insert(0, new_run.clone());
            });

        // TODO: maybe unnecessary. If so, remove extra param on `matches`
        possible_sub_arrangements.retain(|sub_arrangement| {
            let val = matches(runs, sub_arrangement, true);
            // println!("{val}");
            val
        });

        // println!("sdf: {possible_sub_arrangements:?}");
        arrangements.append(&mut possible_sub_arrangements);
    }

    arrangements
}

fn contains(a: &Run, b: &Run) -> bool {
    b.start >= a.start && b.start + b.length <= a.start + a.length
}

fn matches(runs: &ImportantRuns, damaged: &[Run], partial: bool) -> bool {
    // println!("Here: {:?} {damaged:?} {partial}", format_row(runs));
    if !partial {
        for run in &runs.damaged {
            if !damaged.iter().any(|d| contains(d, run)) {
                return false;
            }
        }
    }

    let joined_runs = runs
        .damaged
        .iter()
        .fold(runs.unknown.clone(), |acc, r| add_run(acc, *r));

    if joined_runs.len() == 0 {
        return damaged.len() == 0;
    }

    let mut joined_run_idx = 0;
    for run in damaged {
        while !contains(&joined_runs[joined_run_idx], run) {
            joined_run_idx += 1;
            if joined_run_idx >= joined_runs.len() {
                return false;
            }
        }
    }

    true
}

fn get_arrangements(runs: ImportantRuns, contiguous_damaged: &Vec<u32>) -> u32 {
    let max_len = u32::max(
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
    );
    let all_possible_arrangements =
        get_all_possible_arrangements(&runs, contiguous_damaged, max_len + 1);

    // for arr in all_possible_arrangements.to_owned() {
    // println!(
    //     "{}",
    //     format_row(&ImportantRuns {
    //         damaged: arr,
    //         unknown: Vec::new(),
    //     })
    // );
    // }

    all_possible_arrangements
        .into_iter()
        .filter(|arr| matches(&runs, arr, false))
        .count() as u32
}

fn process(input: &str) -> u32 {
    let mut arrangements = 0;
    for line in input.split('\n') {
        if line.trim().is_empty() {
            continue;
        }

        let (partial_springs, contiguous_damaged) = parse_line(line);
        let (partial_springs, contiguous_damaged) = unfold(partial_springs, contiguous_damaged);

        let runs = get_runs(partial_springs);
        println!("processing {}", format_row(&runs));
        let a = get_arrangements(runs, &contiguous_damaged);
        println!("{a}\t for {line}");
        arrangements += a;
    }

    arrangements
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // #[test]
    // fn test_all_arrangements() {
    //     let result = get_all_possible_arrangements(&[2, 1], 6);
    //     assert_eq!(result.len(), 5);
    // }

    #[test]
    fn test_matches() {
        let (partial_springs, _) = parse_line("?#?#?#?#?#?#?#? 1,3,1,6");
        let runs = get_runs(partial_springs);

        let result = matches(
            &runs,
            &vec![
                Run {
                    start: 1,
                    length: 1,
                },
                Run {
                    start: 3,
                    length: 3,
                },
                Run {
                    start: 7,
                    length: 1,
                },
                // Run {
                //     start: 9,
                //     length: 6,
                // },
            ],
            true,
        );
        assert!(result);

        let (partial_springs, _) = parse_line(".??..??...?##. 1,1,3");
        let runs = get_runs(partial_springs);

        let result = matches(
            &runs,
            &vec![
                Run {
                    start: 1,
                    length: 1,
                },
                // Run {
                //     start: 5,
                //     length: 1,
                // },
                // Run {
                //     start: 10,
                //     length: 3,
                // },
            ],
            true,
        );
        assert!(result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input);
        assert_eq!(result, 525152);
    }
}
