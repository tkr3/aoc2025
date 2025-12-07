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

pub fn part2(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let operator_line = lines.last().unwrap();
    let operators: Vec<(usize, char)> = operator_line
        .char_indices()
        .filter(|x| !x.1.is_whitespace())
        .collect();

    let mut numbers: Vec<[String; 5]> = vec![Default::default(); operators.len()];

    for line in lines[..lines.len() - 1].iter() {
        let mut column_index = 0;
        let mut columns = operators.iter().map(|x| x.0);
        let mut current_column = columns.next().unwrap();
        let mut next_column = columns.next().unwrap();
        for (index, char) in line.char_indices() {
            if index >= next_column {
                current_column = next_column;
                next_column = columns.next().unwrap_or(usize::MAX);
                column_index += 1;
            }
            if char.is_whitespace() {
                continue;
            }
            numbers[column_index][index - current_column].push(char);
        }
    }

    numbers
        .into_iter()
        .map(|x| {
            x.iter()
                .filter_map(|y| y.parse::<u64>().ok())
                .collect::<Vec<_>>()
        })
        .zip(operators.into_iter().map(|(_, operator)| match operator {
            '*' => std::ops::Mul::mul,
            '+' => std::ops::Add::add,
            _ => unreachable!(),
        }))
        .map(|(numbers, operator)| numbers.into_iter().reduce(operator).unwrap())
        .sum()
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
        assert_eq!(part2(EXAMPLE_INPUT_2), 3263827);
    }
}
