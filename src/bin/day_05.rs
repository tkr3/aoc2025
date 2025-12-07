pub fn part1(input: &str) -> usize {
    let (a, b) = input.split_once("\n\n").unwrap();

    let fresh = a
        .lines()
        .map(|x| {
            let (start, end) = x.split_once('-').unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            (start, end)
        })
        .collect::<Vec<_>>();

    b.lines()
        .filter(|x| {
            let x: u64 = x.parse().unwrap();

            fresh.iter().any(|y| y.0 <= x && y.1 >= x)
        })
        .count()
}

pub fn part2(input: &str) -> u64 {
    let (a, _) = input.split_once("\n\n").unwrap();

    let mut fresh = a
        .lines()
        .map(|x| {
            let (start, end) = x.split_once('-').unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            (start, end)
        })
        .collect::<Vec<_>>();

    fresh.sort_unstable_by_key(|x| x.0);

    let mut last_end = 0;
    fresh
        .into_iter()
        .filter_map(|(start, end)| {
            let mut range = None;
            if last_end >= start {
                if last_end < end {
                    range.replace((last_end + 1, end));
                    last_end = end;
                }
            } else {
                last_end = end;
                range.replace((start, end));
            };
            range
        })
        .map(|(start, end)| end - start + 1)
        .sum::<u64>()
}

const INPUT: &str = include_str!("../../inputs/day_05.txt");

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
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 3);
    }

    const EXAMPLE_INPUT_2: &str = EXAMPLE_INPUT_1;

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 14);
    }
}
