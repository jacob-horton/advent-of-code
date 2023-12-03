import os
import sys

if len(sys.argv) != 2:
    print("Invalid number of arguments. Please provide the day number")
    exit(1)

day = int(sys.argv[1])
folder_name = f"day-{day:02}"
os.system(f"cargo new {folder_name}")

os.remove(f"{folder_name}/src/main.rs")
os.mkdir(f"{folder_name}/src/bin")
os.mkdir(f"{folder_name}/src/inputs")

file_contents = """fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

fn process(input: &str) -> u32 {
    0
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part[PART].txt");
        let result = process(input);
        assert_eq!(result, 0);
    }
}
"""

for part in range(1, 3):
    with open(f"{folder_name}/src/bin/part{part}.rs", "w") as f:
        f.writelines(file_contents.replace("[PART]", str(part)))

    open(f"{folder_name}/src/inputs/test_part{part}.txt", "a").close()

open(f"{folder_name}/src/inputs/input.txt", "a").close()
