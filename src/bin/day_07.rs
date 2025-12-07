pub fn part1(input: &str) -> u64 {
    let mut splits = 0;
    input
        .lines()
        .step_by(2)
        .map(|x| x.char_indices().map(|(_, c)| c != '.').collect::<Vec<_>>())
        .reduce(|mut x, y| {
            for i in 0..x.len() {
                if x[i] && y[i] {
                    splits += 1;
                    x[i] = false;
                    if i > 0 {
                        x[i - 1] = true;
                    }
                    if i < x.len() - 1 {
                        x[i + 1] = true;
                    }
                }
            }
            x
        });
    splits
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .step_by(2)
        .map(|x| {
            x.char_indices()
                .map(|(_, c)| if c != '.' { 1 } else { 0 })
                .collect::<Vec<_>>()
        })
        .reduce(|mut x, y| {
            for i in 0..x.len() {
                if x[i] > 0 && y[i] > 0 {
                    if i > 0 {
                        x[i - 1] += x[i];
                    }
                    if i < x.len() - 1 {
                        x[i + 1] += x[i];
                    }
                    x[i] = 0;
                }
            }
            x
        })
        .iter()
        .flatten()
        .sum::<u64>()
}

const INPUT: &str = include_str!("../../inputs/day_07.txt");

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
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 21);
    }

    const EXAMPLE_INPUT_2: &str = EXAMPLE_INPUT_1;

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 40);
    }
}
