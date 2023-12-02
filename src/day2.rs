use regex::Regex;

pub struct Game {
    game: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn new(game_number: u32) -> Game {
        Game {
            game: game_number,
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn update_color(&mut self, color: &str, value: u32) {
        match color {
            "red" => self.red = self.red.max(value),
            "green" => self.green = self.green.max(value),
            "blue" => self.blue = self.blue.max(value),
            _ => {}
        }
    }

    fn get_valid_id(&self) -> Option<u32> {
        const MAX_RED: u32 = 12;
        const MAX_GREEN: u32 = 13;
        const MAX_BLUE: u32 = 14;

        if self.red <= MAX_RED && self.green <= MAX_GREEN && self.blue <= MAX_BLUE {
            Some(self.game)
        } else {
            None
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn parse_game_string(game_str: &str, game_regex: &Regex, colors_regex: &Regex) -> Option<Game> {
    if let Some(caps) = game_regex.captures(game_str) {
        if let (Some(game_caps), Some(colors_str)) = (caps.get(1), caps.get(2)) {
            let game_number: u32 = game_caps.as_str().parse().unwrap();
            let mut game = Game::new(game_number);

            for col_caps in colors_regex.captures_iter(colors_str.as_str()) {
                if let (Some(num_caps), Some(color_caps)) = (col_caps.get(1), col_caps.get(2)) {
                    let number: u32 = num_caps.as_str().parse().unwrap();
                    let color: &str = color_caps.as_str();
                    game.update_color(color, number);
                }
            }

            return Some(game);
        }
    }

    None
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Game> {
    let game_regex = Regex::new(r"Game (\d+): (.*)").unwrap();
    let colors_regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    return input
        .lines()
        .filter_map(|line| parse_game_string(line, &game_regex, &colors_regex))
        .collect();
}

#[aoc(day2, part1)]
pub fn solver_part1(input: &Vec<Game>) -> u32 {
    input.iter().filter_map(|game| game.get_valid_id()).sum()
}

#[aoc(day2, part2)]
pub fn solver_part2(input: &Vec<Game>) -> u32 {
    input.iter().map(|game| game.power()).sum()
}

// :: ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solver_part1(&generator(input)), 8);
    }

    #[test]
    fn example_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solver_part2(&generator(input)), 2286);
    }
}
