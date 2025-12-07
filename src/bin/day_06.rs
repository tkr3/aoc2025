pub fn part1(input: &str) -> u64 {
    let lines = input.lines().map(|x| x.trim());

    let mut numbers: Vec<Vec<u64>> = Vec::new();
    for line in lines {
        if line.starts_with('*') {
            return line
                .split_whitespace()
                .zip(numbers)
                .map(|(operator, operands)| {
                    let operator = match operator {
                        "*" => std::ops::Mul::mul,
                        "+" => std::ops::Add::add,
                        _ => unreachable!(),
                    };
                    operands.into_iter().reduce(operator).unwrap()
                })
                .sum();
        }

        for (index, number) in line
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .enumerate()
        {
            if numbers.len() <= index {
                numbers.push(Vec::new());
            }
            numbers
                .get_mut(index)
                .expect("Vec not inserted")
                .push(number);
        }
    }
    unreachable!("no operators line");
}

pub fn part2(input: &str) -> u32 {
    0
}

const INPUT: &str = include_str!("../../inputs/day_06.txt");

fn main() {
    let p1 = part1(INPUT);
    println!("Part 1: {p1}");

    let p2 = part2(INPUT);
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 4277556);
    }

    const EXAMPLE_INPUT_2: &str = EXAMPLE_INPUT_1;

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 1);
    }
}
