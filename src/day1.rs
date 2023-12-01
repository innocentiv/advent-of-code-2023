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

fn first_last_digit_to_int(s: &str) -> Option<u32> {
    let two_numbers = Regex::new(r"(?P<first>\d).*(?P<last>\d)").unwrap();

    let one_number = Regex::new(r"(?P<one>\d)").unwrap();

    match two_numbers.captures(s) {
        Some(caps) => {
            word_to_digit(&caps["first"]).and_then(|first| {
                word_to_digit(&caps["last"]).map(|last| first * 10 + last)
            })
        }
        _ => {
            one_number.captures(s)
                .and_then(|caps| {
                    word_to_digit(&caps["one"]).map(|one| one * 10 + one)
                })
        }
    }
}

fn first_last_digit_and_words_to_int(s: &str) -> Option<u32> {
    let two_numbers = Regex::new(
        r"(?P<first>\d|one|two|three|four|five|six|seven|eight|nine).*(?P<last>\d|one|two|three|four|five|six|seven|eight|nine)"
    ).unwrap();

    let one_number = Regex::new(
        r"(?P<one>\d|one|two|three|four|five|six|seven|eight|nine)"
    ).unwrap();

    match two_numbers.captures(s) {
        Some(caps) => {
            word_to_digit(&caps["first"]).and_then(|first| {
                word_to_digit(&caps["last"]).map(|last| first * 10 + last)
            })
        }
        _ => {
            one_number.captures(s)
                .and_then(|caps| {
                    word_to_digit(&caps["one"]).map(|one| one * 10 + one)
                })
        }
    }
}

#[aoc_generator(day1, part1)]
pub fn generator1(input: &str) -> Vec<u32> {
    return input
        .lines()
        .filter_map(|line| first_last_digit_to_int(line)).collect();
}

#[aoc(day1, part1)]
pub fn solver_part1(input: &Vec<u32>) -> u32 {
    return input.iter().sum();
}

#[aoc_generator(day1, part2)]
pub fn generator2(input: &str) -> Vec<u32> {
    return input
        .lines()
        .filter_map(|line| first_last_digit_and_words_to_int(line)).collect();
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
