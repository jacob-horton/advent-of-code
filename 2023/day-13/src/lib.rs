fn line_to_bit_pattern(line: &str) -> u32 {
    let mut bit_pattern = 0;

    for (i, char) in line.chars().rev().enumerate() {
        if char == '#' {
            bit_pattern += 2u32.pow(i as u32);
        }
    }

    bit_pattern
}

pub fn does_reflect(pattern: &[u32], mirror: u32, smudges: u32, smudges_exact: bool) -> bool {
    let mut smudges_left = smudges as i32;

    for (l, r) in pattern[..mirror as usize]
        .iter()
        .rev()
        .zip(&pattern[mirror as usize..])
    {
        let diff = (l ^ r).count_ones() as i32;
        if diff != 0 {
            if smudges_left <= 0 {
                return false;
            }

            // Some cells are different (smudges)
            if diff <= smudges_left {
                smudges_left -= diff;
            } else {
                return false;
            }
        }
    }

    if smudges_exact {
        smudges_left == 0
    } else {
        true
    }
}

pub struct BitPatterns {
    pub horizontal: Vec<u32>,
    pub vertical: Vec<u32>,
}

pub fn get_bit_patterns(input: &str) -> Vec<BitPatterns> {
    input
        .trim()
        .split("\n\n")
        .map(|lines| {
            let sub_lines: Vec<_> = lines.split('\n').collect();
            let mut vertical = Vec::new();
            for i in 0..sub_lines[0].len() {
                let vert_str = sub_lines
                    .iter()
                    .map(|l| l.chars().nth(i).unwrap())
                    .collect::<String>();

                vertical.push(line_to_bit_pattern(&vert_str));
            }

            BitPatterns {
                horizontal: lines.split('\n').map(line_to_bit_pattern).collect(),
                vertical,
            }
        })
        .collect()
}
