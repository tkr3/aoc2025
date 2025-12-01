pub fn part1(input: &str) -> u32 {
    let mut sum = 0;
    input.lines().fold(50, |current, line| {
        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let distance: i32 = String::from_iter(chars).parse().unwrap();

        let next = match direction {
            'L' => current - distance,
            'R' => current + distance,
            _ => panic!("Invalid direction"),
        };

        if next % 100 == 0 {
            sum += 1;
        }

        next
    });

    sum
}

pub fn part2(input: &str) -> i32 {
    let mut sum = 0;
    input.lines().fold(50, |current, line| {
        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let mut distance: i32 = String::from_iter(chars).parse().unwrap();

        sum += distance / 100;
        distance %= 100;

        let next = match direction {
            'L' => current - distance,
            'R' => current + distance,
            _ => panic!("Invalid direction"),
        };
        if current != 0 && next <= 0 || next >= 100 {
            sum += 1;
        }
        next.rem_euclid(100)
    });

    sum
}

const INPUT: &str = include_str!("../../inputs/day_01.txt");

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
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 3);
    }

    const EXAMPLE_INPUT_2: &str = EXAMPLE_INPUT_1;

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 6);
    }
}
