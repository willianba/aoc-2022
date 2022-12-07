use std::fs;

fn part1(result: &Vec<u32>) -> u32 {
    result.iter().fold(u32::MIN, |a, b| a.max(*b))
}

fn part2(result: &mut Vec<u32>) -> u32 {
    result.sort();
    result.iter().rev().take(3).sum::<u32>()
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("no panic");

    let mut sum = 0;
    let mut result: Vec<u32> = Vec::new();
    content.lines().for_each(|line| match line.is_empty() {
        true => {
            result.push(sum);
            sum = 0
        }
        false => {
            let num = line.parse::<u32>().unwrap();
            sum += num;
        }
    });

    let part1 = part1(&result);
    let part2 = part2(&mut result);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
