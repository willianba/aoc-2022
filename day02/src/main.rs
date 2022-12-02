use std::str::Lines;

#[derive(Clone, Copy)]
enum ActionType {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win,
    Lose,
    Draw,
}

trait Action {
    // could the ActionType parameter be the Action trait itself?
    fn get_result(&self, action: &ActionType) -> Result;
    fn get_score(&self, result: &Result) -> u16;
    fn get_expected_result_from_action(&self) -> Result;
    fn get_expected_action(&self, result: &Result) -> ActionType;
}

impl Action for ActionType {
    fn get_result(&self, action: &ActionType) -> Result {
        match (self, action) {
            (ActionType::Rock, ActionType::Scissors) => Result::Win,
            (ActionType::Rock, ActionType::Paper) => Result::Lose,
            (ActionType::Paper, ActionType::Rock) => Result::Win,
            (ActionType::Paper, ActionType::Scissors) => Result::Lose,
            (ActionType::Scissors, ActionType::Paper) => Result::Win,
            (ActionType::Scissors, ActionType::Rock) => Result::Lose,
            _ => Result::Draw,
        }
    }

    fn get_score(&self, result: &Result) -> u16 {
        match (self, result) {
            (ActionType::Rock, Result::Win) => 7,
            (ActionType::Rock, Result::Lose) => 1,
            (ActionType::Rock, Result::Draw) => 4,
            (ActionType::Paper, Result::Win) => 8,
            (ActionType::Paper, Result::Lose) => 2,
            (ActionType::Paper, Result::Draw) => 5,
            (ActionType::Scissors, Result::Win) => 9,
            (ActionType::Scissors, Result::Lose) => 3,
            (ActionType::Scissors, Result::Draw) => 6,
        }
    }

    fn get_expected_result_from_action(&self) -> Result {
        match self {
            ActionType::Rock => Result::Lose,
            ActionType::Paper => Result::Draw,
            ActionType::Scissors => Result::Win,
        }
    }

    fn get_expected_action(&self, result: &Result) -> ActionType {
        match (self, result) {
            (ActionType::Rock, Result::Win) => ActionType::Paper,
            (ActionType::Rock, Result::Lose) => ActionType::Scissors,
            (ActionType::Paper, Result::Win) => ActionType::Scissors,
            (ActionType::Paper, Result::Lose) => ActionType::Rock,
            (ActionType::Scissors, Result::Win) => ActionType::Rock,
            (ActionType::Scissors, Result::Lose) => ActionType::Paper,
            (_, Result::Draw) => *self,
        }
    }
}

struct Player {
    action: ActionType,
}

impl Player {
    fn new(action: String) -> Player {
        Player {
            action: Player::get_action_type(action),
        }
    }

    // how to move to the action trait?
    fn get_action_type(action: String) -> ActionType {
        match action.as_str() {
            "A" | "X" => ActionType::Rock,
            "B" | "Y" => ActionType::Paper,
            "C" | "Z" => ActionType::Scissors,
            _ => panic!("Invalid action"),
        }
    }
}

fn part1(lines: Lines) -> u16 {
    lines
        .map(|line| {
            let moves = line.split(" ").collect::<Vec<&str>>();
            let mut moves_iter = moves.iter();
            let opponent = Player::new(moves_iter.next().unwrap().to_string());
            let player = Player::new(moves_iter.next().unwrap().to_string());
            (player.action, player.action.get_result(&opponent.action))
        })
        .map(|(action, result)| action.get_score(&result))
        .sum::<u16>()
}

fn part2(lines: Lines) -> u16 {
    lines
        .map(|line| {
            let moves = line.split(" ").collect::<Vec<&str>>();
            let mut moves_iter = moves.iter();
            let opponent = Player::new(moves_iter.next().unwrap().to_string());
            let mut player = Player::new(moves_iter.next().unwrap().to_string());
            let expected_player_result = player.action.get_expected_result_from_action();
            player.action = opponent.action.get_expected_action(&expected_player_result);
            (player.action, player.action.get_result(&opponent.action))
        })
        .map(|(action, result)| action.get_score(&result))
        .sum::<u16>()
}

fn main() {
    let file = include_str!("../input.txt");

    let part1 = part1(file.lines());
    let part2 = part2(file.lines());

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
