pub fn part1(input: &str) -> u32 {
    input.lines().fold(0, |a, x| {
        let (b, i) = x[..x.len() - 1].chars().max_battery();
        a + b * 10 + x[i..].chars().max_battery().0
    })
}

pub fn part2(input: &str) -> u64 {
    input.lines().fold(0, |a, x| {
        let mut joltage: u64 = 0;
        let mut start_index = 0;
        for battery in (0..12).rev() {
            let (b, i) = x[start_index..x.len() - battery].chars().max_battery();
            start_index += i;
            joltage = joltage * 10 + b as u64;
        }
        a + joltage
    })
}

trait MaxBattery: Iterator {
    fn max_battery(&mut self) -> (u32, usize);
}

impl<I> MaxBattery for I
where
    I: Iterator<Item = char>,
{
    fn max_battery(&mut self) -> (u32, usize) {
        let mut max = '0';
        let mut index = 0;
        for (i, char) in self.enumerate() {
            if char > max {
                max = char;
                index = i;
            }
            if max == '9' {
                break;
            }
        }
        (max.to_digit(10).unwrap(), index + 1)
    }
}

const INPUT: &str = include_str!("../../inputs/day_03.txt");

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
987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 357);
    }

    const EXAMPLE_INPUT_2: &str = EXAMPLE_INPUT_1;

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 3121910778619);
    }
}
