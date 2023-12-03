use std::fs;

fn get_input(s: &str) -> Vec<String> {
    fs::read_to_string(s)
        .expect("Failed to read file")
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}


fn first_num(s: &str) -> u32 {
    for c in s.chars() {
        if let Some(n) = c.to_digit(10) {
            return n;
        }
    }
    0
}

fn part1(input: &Vec<String>) -> u32 {
    let mut sum = 0;

    for s in input {
        let rev: String = s.chars().rev().collect();
        sum += (first_num(&s)*10) + first_num(&rev);
    }
    sum
}

fn get_digit(s: &str) -> Option<u32> {
    if let Some(n) = s.chars().next().map(|c| c.to_digit(10)).flatten() {
        return Some(n);
    }

    if s.starts_with("one") {
        Some(1)
    } else if s.starts_with("two") {
        Some(2)
    } else if s.starts_with("three") {
        Some(3)
    } else if s.starts_with("four") {
        Some(4)
    } else if s.starts_with("five") {
        Some(5)
    } else if s.starts_with("six") {
        Some(6)
    } else if s.starts_with("seven") {
        Some(7)
    } else if s.starts_with("eight") {
        Some(8)
    } else if s.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}

fn digits_in_string(s: &str) -> Vec<u32> {
    let mut digits = Vec::new();
    for cursor in 0..s.len() {
        if let Some(n) = get_digit(&s[cursor..]) {
            digits.push(n);
        }
    }
    digits
}


fn part2(input: &Vec<String>) -> u32 {
    let mut sum = 0;
    for s in input {
        let digits = digits_in_string(&s);
        sum += (digits.first().unwrap()*10) + digits.last().unwrap()
    }
    sum
}

pub fn run() {
    let input = get_input("inputs/day01.txt");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

pub fn name() -> &'static str {
    "Trebuchet?!"
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let sample_input = get_input("inputs/day01_sample1.txt");
        assert_eq!(part1(&sample_input), 142);
    }

    #[test]
    fn test_part2() {
        let sample_input = get_input("inputs/day01_sample2.txt");
        assert_eq!(part2(&sample_input), 281);
    }
}
