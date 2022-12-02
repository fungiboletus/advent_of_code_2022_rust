use std::collections::VecDeque;

#[derive(Debug, Clone)]
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

            // For Part 1
            "X" => Ok(Action::Rock),
            "Y" => Ok(Action::Paper),
            "Z" => Ok(Action::Scissors),
            _ => Err(()),
        }
    }
}

impl std::str::FromStr for GameResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err(()),
        }
    }
}

impl Action {
    pub fn against(&self, opponent_action: &Action) -> GameResult {
        match (self, opponent_action) {
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

    pub fn compute_move(&self, opponent_action: &Action) -> Action {
        match self {
            GameResult::Win => match opponent_action {
                Action::Rock => Action::Paper,
                Action::Paper => Action::Scissors,
                Action::Scissors => Action::Rock,
            },
            GameResult::Draw => opponent_action.clone(),
            GameResult::Lose => match opponent_action {
                Action::Rock => Action::Scissors,
                Action::Paper => Action::Rock,
                Action::Scissors => Action::Paper,
            },
        }
    }
}

pub fn parse_input_data_part_1(input: &str) -> Vec<(Action, Action)> {
    return input
        // Split by lines
        .lines()
        .map(|game| {
            let mut actions = game
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

pub fn parse_input_data_part_2(input: &str) -> Vec<(Action, GameResult)> {
    return input
        .lines()
        .map(|game| {
            let mut tuple = game.split_whitespace().collect::<VecDeque<&str>>();
            let action = tuple
                .pop_front()
                .expect("Failed to get action")
                .parse::<Action>()
                .expect("Failed to parse action");
            let game_result = tuple
                .pop_front()
                .expect("Failed to get game result")
                .parse::<GameResult>()
                .expect("Failed to parse game result");
            return (action, game_result);
        })
        .collect();
}

pub fn day_2_part_1(data: &str) -> i64 {
    let played_actions = parse_input_data_part_1(data);

    let game_results = played_actions.iter().map(|(a, b)| b.against(a));

    let score = played_actions
        .iter()
        .map(|(_, action)| action.points())
        .sum::<i64>()
        + game_results.map(|result| result.points()).sum::<i64>();

    return score;
}

pub fn day_2_part_2(data: &str) -> i64 {
    let game_results = parse_input_data_part_2(data);

    let played_actions = game_results
        .iter()
        .map(|(action, game_result)| game_result.compute_move(action));

    let score = played_actions.map(|action| action.points()).sum::<i64>()
        + game_results
            .iter()
            .map(|(_, game_result)| game_result.points())
            .sum::<i64>();

    return score;
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
        assert_eq!(day_2_part_2(EXAMPLE), 12);
    }
}
