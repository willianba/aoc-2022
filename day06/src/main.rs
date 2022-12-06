fn solve(file: &str, size: usize) -> usize {
    let mut result = 0;
    for (i, c) in file.chars().enumerate() {
        let next = file.chars().skip(i + 1).take(size).collect::<String>();
        let concat = c.to_string() + &next;
        if concat.chars().all(|c| concat.matches(c).count() == 1) {
            result = i + 1 + size;
            break;
        }
    }
    result
}

fn main() {
    let file = include_str!("../input.txt");
    println!("Part 1: {}", solve(file, 3));
    println!("Part 2: {}", solve(file, 13));
}
