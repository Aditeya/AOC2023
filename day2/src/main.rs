use day2::{Game, Set};

fn main() {
    println!("Day 2");
    println!("Part 1");

    let file_str = include_str!("input");
    let games = parse_input(file_str);
    let games_sum = find_games_sum(&games, (12,13,14).into());
    println!("sum = {games_sum}");

    println!("Part 2");
    let games_min_set_power_sum = find_game_min_cube_power_set_sum(&games);
    println!("games_min_set_power_sum = {games_min_set_power_sum}");
}

fn parse_input(file_str: &str) -> Vec<Game> {
    file_str.lines().filter_map(parse_line).collect()
}

fn parse_line(line: &str) -> Option<Game> {
    let (id, sets) = line.split_once(": ").and_then(|(g_id, s)| {
        g_id.strip_prefix("Game ")
            .and_then(|s| s.parse::<u32>().ok())
            .zip(Some(s))
    })?;

    let rounds = sets
        .split("; ")
        .map(|set| {
            set.split(", ").fold(Set::default(), |mut acc, cube| {
                let Some((count, color)) = cube
                    .split_once(' ')
                    .and_then(|(cnt, col)| cnt.parse::<u32>().ok().zip(Some(col)))
                else {
                    return acc;
                };

                match color {
                    "blue" => acc.b = count,
                    "green" => acc.g = count,
                    "red" => acc.r = count,
                    _ => (),
                }

                acc
            })
        })
        .collect();

    Some(Game { id, sets: rounds })
}

fn find_games_sum(games: &[Game], round: Set) -> u32 {
    games.iter().filter_map(|game| {
        if game.sets.iter().all(|r| r.is_round_valid(&round)) {
            return Some(game.id);
        }

        None
    }).sum()
}

fn find_game_min_cube_power_set_sum(games: &[Game]) -> u32 {
    games.iter()
        .map(|game| game.min_set())
        .map(|s| s.power())
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{parse_input, find_games_sum, find_game_min_cube_power_set_sum};

    const TEST_INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn parse_test() {
        let games = parse_input(TEST_INPUT);
        let games_eq = vec![
            (
                1,
                vec![(4, 0, 3).into(), (1, 2, 6).into(), (0, 2, 0).into()],
            )
                .into(),
            (
                2,
                vec![(0, 2, 1).into(), (1, 3, 4).into(), (0, 1, 1).into()],
            )
                .into(),
            (
                3,
                vec![(20, 8, 6).into(), (4, 13, 5).into(), (1, 5, 0).into()],
            )
                .into(),
            (
                4,
                vec![(3, 1, 6).into(), (6, 3, 0).into(), (14, 3, 15).into()],
            )
                .into(),
            (5, vec![(6, 3, 1).into(), (1, 2, 2).into()]).into(),
        ];

        for (a, b) in games.iter().zip(games_eq) {
            assert_eq!(*a, b);
        }
    }

    #[test]
    fn find_games_sum_test() {
        let games = parse_input(TEST_INPUT);
        assert_eq!(find_games_sum(&games, (12,13,14).into()), 8);
    }

    #[test]
    fn find_games_min_cube_power_sum_test() {
        let games = parse_input(TEST_INPUT);
        assert_eq!(find_game_min_cube_power_set_sum(&games), 2286);
    }
}
