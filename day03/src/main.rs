use std::{collections::HashSet, str::Lines};

fn part1(lines: Lines) -> u16 {
    lines
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let left_nums = map_to_num_vec(left);
            let right_nums = map_to_num_vec(right);
            (left_nums, right_nums)
        })
        .map(|(left, right)| {
            let left = remove_duplicates(left);
            let right = remove_duplicates(right);
            left.intersection(&right).fold(0, |acc, x| acc + x)
        })
        .sum::<u16>()
}

fn part2(lines: Lines) -> u16 {
    lines
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            let first = map_to_num_vec(chunk[0]);
            let second = map_to_num_vec(chunk[1]);
            let third = map_to_num_vec(chunk[2]);
            (first, second, third)
        })
        .map(|(first, second, third)| {
            let first = remove_duplicates(first);
            let second = remove_duplicates(second);
            let third = remove_duplicates(third);
            first
                .intersection(&second)
                .filter(|x| third.contains(x))
                .fold(0, |acc, x| acc + x)
        })
        .sum::<u16>()
}

fn get_letter_number(letter: char) -> u16 {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    // could be done with a match, but this is more concise
    alphabet.find(letter).unwrap() as u16 + 1
}

fn map_to_num_vec(line: &str) -> Vec<u16> {
    line.chars().map(get_letter_number).collect::<Vec<u16>>()
}

fn remove_duplicates(nums: Vec<u16>) -> HashSet<u16> {
    nums.into_iter().collect::<HashSet<u16>>()
}

fn main() {
    let file = include_str!("../input.txt");
    println!("Part 1: {}", part1(file.lines()));
    println!("Part 2: {}", part2(file.lines()));
}
