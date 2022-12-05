use std::str::Lines;

fn part1(lines: Lines) -> usize {
    lines
        .map(|line| {
            let (l, r) = line.split_once(",").unwrap();
            let ((a, b), (c, d)) = (l.split_once("-").unwrap(), r.split_once("-").unwrap());
            (
                a.parse::<u16>().unwrap(),
                b.parse::<u16>().unwrap(),
                c.parse::<u16>().unwrap(),
                d.parse::<u16>().unwrap(),
            )
        })
        .filter(|(a, b, c, d)| (a <= c && d <= b) || (c <= a && b <= d))
        .count()
}

fn part2(lines: Lines) -> usize {
    lines
        .map(|line| {
            let (l, r) = line.split_once(",").unwrap();
            let ((a, b), (c, d)) = (l.split_once("-").unwrap(), r.split_once("-").unwrap());
            (
                a.parse::<u16>().unwrap(),
                b.parse::<u16>().unwrap(),
                c.parse::<u16>().unwrap(),
                d.parse::<u16>().unwrap(),
            )
        })
        .filter(|(a, b, c, d)| {
            (b >= c && b <= d) || (c >= a && c <= b) || (a >= c && a <= d) || (d >= a && d <= b)
        })
        .count()
}

fn main() {
    let file = include_str!("../input.txt");
    println!("Part 1: {}", part1(file.lines()));
    println!("Part 2: {}", part2(file.lines()));
}
