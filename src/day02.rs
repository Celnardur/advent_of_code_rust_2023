use std::fs;

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

impl Game {
    fn new(s: &str) -> Game {
        let mut parts = s.split(':');
        let id_part = parts.next().unwrap();
        let draws_part = parts.next().unwrap();

        let id = id_part.split(' ').rev().next().unwrap().parse().unwrap();

        let mut draws = Vec::new();
        for draw_str in draws_part.split(';') {
            draws.push(Draw::new(draw_str));
        }

        Game {
            id,
            draws,
        }
    }

    fn is_impossible(&self, max_draw: &Draw) -> bool {
        for draw in &self.draws {
            if draw.red > max_draw.red || draw.green > max_draw.green || draw.blue > max_draw.blue {
                return true;
            }
        }
        false
    }

    fn min_game_power(&self) -> usize {
        let mut min_draw = Draw::zero_draw();
        for draw in &self.draws {
            if draw.red > min_draw.red {
                min_draw.red = draw.red;
            }
            if draw.green > min_draw.green {
                min_draw.green = draw.green;
            }
            if draw.blue > min_draw.blue {
                min_draw.blue = draw.blue;
            }
        }
        min_draw.power()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

impl Draw {
    fn zero_draw() -> Draw {
        Draw {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn new(s: &str) -> Draw {
        let mut draw = Draw::zero_draw();

        for cube in s.split(',') {
            let mut parts = cube.trim().split(' ');
            let number = parts.next().unwrap().parse().unwrap();
            match parts.next().unwrap() {
                "red" => draw.red = number,
                "green" => draw.green = number,
                "blue" => draw.blue = number,
                _ => panic!("Expected Color")
            }
        }
        draw
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

fn get_input(s: &str) -> Vec<Game> {
    fs::read_to_string(s)
        .expect("Failed to read file")
        .lines()
        .map(|s| Game::new(s))
        .collect()
}

fn part1(input: &Vec<Game>) -> usize {
    let max_draw = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut id_sum = 0;
    for game in input {
        if !game.is_impossible(&max_draw) {
            id_sum += game.id;
        }
    }
    id_sum
}

fn part2(input: &Vec<Game>) -> usize {
    let mut power_sum = 0;
    for game in input {
        power_sum += game.min_game_power();
    }
    power_sum
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
    fn test_input_parsing() {
        let sample_game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string();
        assert_eq!(
            Game::new(&sample_game), 
            Game {
                id: 1,
                draws: vec![
                    Draw {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    Draw {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Draw {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        let sample_input = get_input("inputs/day02_sample1.txt");
        assert_eq!(part1(&sample_input), 8);
    }

    #[test]
    fn test_part2() {
        let sample_input = get_input("inputs/day02_sample2.txt");
        assert_eq!(part2(&sample_input), 2286);
    }
}