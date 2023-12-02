use regex::Regex;

fn word_to_digit(word: &str) -> Option<u32> {
    match word {
        "1" => Some(1),
        "2" => Some(2),
        "3" => Some(3),
        "4" => Some(4),
        "5" => Some(5),
        "6" => Some(6),
        "7" => Some(7),
        "8" => Some(8),
        "9" => Some(9),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn first_last_to_int(s: &str, reg: &Regex) -> Option<u32> {
    let matches: Vec<_> = reg.find_iter(s).collect();

    matches.first().and_then(|first_match| {
        word_to_digit(first_match.as_str()).and_then(|first| {
            matches.last().and_then(|last_match| {
                word_to_digit(last_match.as_str()).map(|last| first * 10 + last)
            })
        })
    })
}

#[aoc_generator(day1, part1)]
pub fn generator1(input: &str) -> Vec<u32> {
    let pattern = Regex::new(r"(\d)").unwrap();

    return input
        .lines()
        .filter_map(|line| first_last_to_int(line, &pattern))
        .collect();
}

#[aoc(day1, part1)]
pub fn solver_part1(input: &Vec<u32>) -> u32 {
    return input.iter().sum();
}

#[aoc_generator(day1, part2)]
pub fn generator2(input: &str) -> Vec<u32> {
    let pattern = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    return input
        .lines()
        .filter_map(|line| first_last_to_int(line, &pattern))
        .collect();
}

#[aoc(day1, part2)]
pub fn solver_part2(input: &Vec<u32>) -> u32 {
    return input.iter().sum();
}

// :: ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";

        assert_eq!(solver_part1(&generator1(input)), 142);
    }

    #[test]
    fn example_2() {
        let input = "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";

        assert_eq!(solver_part2(&generator2(input)), 281);
    }
}
