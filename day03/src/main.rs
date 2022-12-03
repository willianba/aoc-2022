use std::{collections::HashSet, str::Lines};

fn part1(lines: Lines) -> u16 {
    lines
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let (left, right) = (convert_to_num_set(left), convert_to_num_set(right));
            left.intersection(&right).fold(0, |acc, x| acc + x)
        })
        .sum::<u16>()
}

fn part2(lines: Lines) -> u16 {
    lines
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            let (first, second, third) = (
                convert_to_num_set(chunk[0]),
                convert_to_num_set(chunk[1]),
                convert_to_num_set(chunk[2]),
            );
            first
                .intersection(&second)
                .filter(|x| third.contains(x))
                .fold(0, |acc, x| acc + x)
        })
        .sum::<u16>()
}

fn convert_to_num_set(line: &str) -> HashSet<u16> {
    line.chars()
        .map(get_letter_number)
        .collect::<HashSet<u16>>()
}

fn get_letter_number(letter: char) -> u16 {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    // could be done with a match, but this is more concise
    alphabet.find(letter).unwrap() as u16 + 1
}

fn main() {
    let file = include_str!("../input.txt");
    println!("Part 1: {}", part1(file.lines()));
    println!("Part 2: {}", part2(file.lines()));
}
