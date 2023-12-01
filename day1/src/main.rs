fn main() {
    println!("Day 1");
    println!("Part 1");
    let file = include_str!("input");
    let calibration_value = compute_calibration_value(file);
    println!("calibration_value: {calibration_value}");

    println!("Part 2");
    let file = word_to_num(file);
    let calibration_value = compute_calibration_value(&file);
    println!("calibration_value: {calibration_value}");
}

fn compute_calibration_value(file_str: &str) -> u32 {
    file_str
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|f| f.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .filter_map(|nums| {
            nums.first()
                .zip(nums.last())
                .map(|(first, last)| first * 10 + last)
        })
        .sum()
}

fn word_to_num(file_str: &str) -> String {
    const WORDS_TO_NUM: [(&str, &str); 17] = [
        // NOTE: this is a bruh moment because it's not coverd in the examples
        ("twone", "21"),
        ("fiveight", "58"),
        ("threeight", "38"),
        ("nineight", "98"),
        ("oneight", "18"),
        ("eighthree", "83"),
        ("eightwo", "82"),
        ("sevenine", "79"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut result = file_str.to_string();
    for (word, num) in &WORDS_TO_NUM {
        result = result.replace(word, num);
    }

    result
}

#[cfg(test)]
mod test {
    use crate::{compute_calibration_value, word_to_num};

    #[test]
    fn part_1() {
        let calibration_value = compute_calibration_value(
            r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#,
        );
        assert_eq!(calibration_value, 142)
    }

    #[test]
    fn part_2() {
        let file = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        let file = word_to_num(file);
        let calibration_value = compute_calibration_value(&file);
        assert_eq!(calibration_value, 281)
    }
}
