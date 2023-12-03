
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

fn get_input(s: &str) -> Vec<Game> {
    Vec::new()
}

fn part1(input: &Vec<Game>) -> usize {
    0
}

fn part2(input: &Vec<Game>) -> usize {
    0
}

pub fn run() {
    let input = get_input("inputs/day02.txt");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

pub fn name() -> &'static str {
    "Cube Conundrum"
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let sample_input = get_input("inputs/day01_sample1.txt");
        assert_eq!(part1(&sample_input), 8);
    }

    #[test]
    fn test_part2() {
        let sample_input = get_input("inputs/day01_sample2.txt");
        assert_eq!(part2(&sample_input), 0);
    }
}