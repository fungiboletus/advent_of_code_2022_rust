use std::collections::VecDeque;

#[derive(Debug)]
pub enum Action {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub enum GameResult {
    Win,
    Lose,
    Draw,
}

impl std::str::FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Don't ask me why the letters don't make any sense
            "A" => Ok(Action::Rock),
            "B" => Ok(Action::Paper),
            "C" => Ok(Action::Scissors),

            "X" => Ok(Action::Rock),
            "Y" => Ok(Action::Paper),
            "Z" => Ok(Action::Scissors),
            _ => Err(()),
        }
    }
}

impl Action {
    pub fn against(&self, other: &Action) -> GameResult {
        match (self, other) {
            (Action::Rock, Action::Rock) => GameResult::Draw,
            (Action::Rock, Action::Paper) => GameResult::Lose,
            (Action::Rock, Action::Scissors) => GameResult::Win,
            (Action::Paper, Action::Rock) => GameResult::Win,
            (Action::Paper, Action::Paper) => GameResult::Draw,
            (Action::Paper, Action::Scissors) => GameResult::Lose,
            (Action::Scissors, Action::Rock) => GameResult::Lose,
            (Action::Scissors, Action::Paper) => GameResult::Win,
            (Action::Scissors, Action::Scissors) => GameResult::Draw,
        }
    }

    pub fn points(&self) -> i64 {
        match self {
            Action::Rock => 1,
            Action::Paper => 2,
            Action::Scissors => 3,
        }
    }
}

impl GameResult {
    pub fn points(&self) -> i64 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Lose => 0,
        }
    }
}

pub fn parse_input_data(input: &str) -> Vec<(Action, Action)> {
    return input
        // Split by lines
        .lines()
        .map(|play| {
            let mut actions = play
                .split_whitespace()
                .map(|action| action.parse::<Action>().expect("Failed to parse action"))
                .collect::<VecDeque<Action>>();
            return (
                actions.pop_front().expect("Failed to get action"),
                actions.pop_front().expect("Failed to get action"),
            );
        })
        .collect();
}

pub fn day_2_part_1(data: &str) -> i64 {
    let games = parse_input_data(data);

    let game_results = games.iter().map(|(a, b)| b.against(a));

    let score = games.iter().map(|(_, action)| action.points()).sum::<i64>()
        + game_results.map(|result| result.points()).sum::<i64>();

    return score;
}

pub fn day_2_part_2(data: &str) -> i64 {
    let data = parse_input_data(data);

    return data.len() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "A Y
    B X
    C Z";

    #[test]
    fn test_day_2_part_1() {
        assert_eq!(day_2_part_1(EXAMPLE), 15);
    }

    #[test]
    fn test_day_2_part_2() {
        assert_eq!(day_2_part_2(EXAMPLE), 45000);
    }
}
