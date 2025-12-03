pub fn part1(input: &str) -> u64 {
    input
        .trim()
        .split(',')
        .filter_map(|x| {
            let (a, b) = x.split_once('-').unwrap();

            if a.len() % 2 == 1 && b.len() == a.len() {
                return None;
            }
            Some((a, b))
        })
        .flat_map(|(a, b)| {
            let start: u64 = if a.len() % 2 == 0 {
                let (a1, a2) = a.split_at(a.len() / 2);
                let a1 = a1.parse().unwrap();
                let a2 = a2.parse::<u64>().unwrap();
                if a2 > a1 {
                    a1 + 1
                } else {
                    a1
                }
            } else {
                10u64.pow(a.len() as u32 / 2)
            };

            let end: u64 = if b.len() % 2 == 0 {
                let (b1, b2) = b.split_at(b.len() / 2);
                let b1 = b1.parse().unwrap();
                let b2 = b2.parse::<u64>().unwrap();
                if b2 >= b1 {
                    b1
                } else {
                    b1 - 1
                }
            } else {
                10u64.pow(b.len() as u32 / 2) - 1
            };

            start..=end
        })
        .map(|x| x * 10u64.pow(x.ilog10() + 1) + x)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .trim()
        .split(',')
        .map(|x| x.split_once('-').unwrap())
        .flat_map(|(a, b)| {
            let a: u64 = a.parse().unwrap();
            let b: u64 = b.parse().unwrap();

            (a..=b).filter(|x| {
                let id: Vec<char> = x.to_string().chars().collect();
                let len = id.len();
                for i in 2..=len {
                    if !len.is_multiple_of(i) {
                        continue;
                    }
                    let mut chunks = id.chunks_exact(len / i);
                    let pattern = chunks.next().unwrap();
                    if chunks.all(|x| x == pattern) {
                        return true;
                    }
                }
                false
            })
        })
        .sum::<u64>()
}

const INPUT: &str = include_str!("../../inputs/day_02.txt");

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
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124\
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 1227775554);
    }

    const EXAMPLE_INPUT_2: &str = EXAMPLE_INPUT_1;

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 4174379265);
    }
}
