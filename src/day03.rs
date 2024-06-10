use std::fs;

struct Schematic {
    grid: Vec<Vec<char>>,
    nums: Vec<Num>,
}

struct Num {
    value: u32,
    row: usize,
    col: usize,
    length: u32,
}

fn parse_num(grid: &Vec<Vec<char>>, row: usize, col: usize) -> Option<Num> {
    let slice = &grid[row][col..];
    if !slice[0].is_ascii_digit() {
        return None;
    }

    let num = Num {
        value: slice[0].to_digit(10).unwrap(), // checked for ascii digit eairler
        row: row,
        col: col,
        length: 1,
    };

    None
}

fn get_input(path: &str) -> Schematic {
    let grid: Vec<Vec<char>> = fs::read_to_string(path)
        .expect("Failed to read file")
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let mut nums = Vec::new();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if let Some(num) = parse_num(&grid, row, col) {
                nums.push(num)
            }
        }
    }

    Schematic { grid, nums }
}

fn part1(sch: &Schematic) -> u32 {
    0
}

pub fn run() {
    let input = get_input("inputs/day03txt");
    println!("{}", part1(&input));
    // println!("{}", part2(&input));
}

pub fn name() -> &'static str {
    "Gear Ratios"
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let sample_input = get_input("inputs/day03_sample1.txt");
        assert_eq!(part1(&sample_input), 4361);
    }

    // #[test]
    // fn test_part2() {
    //     let sample_input = get_input("inputs/day03_sample2.txt");
    //     assert_eq!(part2(&sample_input), 281);
    // }
}
