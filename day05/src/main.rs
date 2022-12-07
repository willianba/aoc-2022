#[derive(Clone)]
struct Stack {
    data: Vec<String>,
}

impl Stack {
    fn new() -> Stack {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, item: String) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<String> {
        self.data.pop()
    }
}

struct Movement {
    amount: usize,
    from: usize,
    to: usize,
}

impl Movement {
    fn new(line: &str) -> Movement {
        let split_values = line.split(" ").collect::<Vec<&str>>();
        Movement {
            amount: split_values[1].parse::<usize>().unwrap(),
            from: split_values[3].parse::<usize>().unwrap() - 1,
            to: split_values[5].parse::<usize>().unwrap() - 1,
        }
    }
}

fn part1(mut stacks: Vec<Stack>, movements: &str) -> String {
    movements.lines().map(Movement::new).for_each(|mov| {
        for _ in 0..mov.amount {
            let popped = stacks[mov.from].pop().unwrap();
            stacks[mov.to].push(popped);
        }
    });

    let mut result = String::new();
    for stack in stacks {
        let mut stack = stack.clone();
        result.push_str(&stack.pop().unwrap());
    }

    result
}

fn part2(mut stacks: Vec<Stack>, movements: &str) -> String {
    movements
        .lines()
        .map(Movement::new)
        .for_each(|mov| match mov.amount > 1 {
            true => {
                let mut temp_stack = Stack::new();

                for _ in 0..mov.amount {
                    let popped = stacks[mov.from].pop().unwrap();
                    temp_stack.push(popped);
                }

                for _ in 0..mov.amount {
                    let popped = temp_stack.pop().unwrap();
                    stacks[mov.to].push(popped);
                }
            }
            false => {
                let popped = stacks[mov.from].pop().unwrap();
                stacks[mov.to].push(popped);
            }
        });

    let mut result = String::new();
    for stack in stacks {
        let mut stack = stack.clone();
        result.push_str(&stack.pop().unwrap());
    }

    result
}

fn main() {
    let file = include_str!("../input.txt");

    let positions = [1, 5, 9, 13, 17, 21, 25, 29, 33];
    let mut stacks = vec![Stack::new(); 9];

    let (boxes, movements) = file.split_once("\n\n").unwrap();

    boxes.lines().rev().skip(1).for_each(|line| {
        for (i, p) in positions.into_iter().enumerate() {
            match line.chars().nth(p) {
                Some(' ') => {}
                Some(p) => stacks[i].push(p.to_string()),
                None => {}
            }
        }
    });

    println!("Part 1: {}", part1(stacks.clone(), movements));
    println!("Part 2: {}", part2(stacks.clone(), movements));
}
