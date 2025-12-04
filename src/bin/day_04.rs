pub fn part1(input: &str) -> usize {
    let mut row_len: isize = 0;
    let grid: Vec<bool> = input
        .lines()
        .flat_map(|line| {
            row_len = line.len() as isize;
            line.chars().map(|char| char == '@')
        })
        .collect();

    grid.iter()
        .enumerate()
        .filter(|(_, &x)| x)
        .filter(|(i, _)| {
            let index = *i as isize;
            let above = index - row_len;
            let below = index + row_len;
            let border_top = index < row_len;
            let border_bottom = index >= grid.len() as isize - row_len;
            let border_left = index % row_len == 0;
            let border_right = (index + 1) % row_len == 0;
            let paper_count = [
                (!border_top && !border_left).then_some(above - 1),
                (!border_top).then_some(above),
                (!border_top && !border_right).then_some(above + 1),
                (!border_left).then_some(index - 1),
                (!border_right).then_some(index + 1),
                (!border_bottom && !border_left).then_some(below - 1),
                (!border_bottom).then_some(below),
                (!border_bottom && !border_right).then_some(below + 1),
            ]
            .into_iter()
            .filter(|x| x.is_some_and(|y| grid[y as usize]))
            .count();
            paper_count < 4
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    let mut row_len: isize = 0;
    let mut grid: Vec<bool> = input
        .lines()
        .flat_map(|line| {
            row_len = line.len() as isize;
            line.chars().map(|char| char == '@')
        })
        .collect();

    let mut removed_rolls = 0;

    loop {
        let to_remove: Vec<usize> = grid
            .iter()
            .enumerate()
            .filter(|(_, &x)| x)
            .filter(|(i, _)| {
                let index = *i as isize;
                let above = index - row_len;
                let below = index + row_len;
                let border_top = index < row_len;
                let border_bottom = index >= grid.len() as isize - row_len;
                let border_left = index % row_len == 0;
                let border_right = (index + 1) % row_len == 0;
                let paper_count = [
                    (!border_top && !border_left).then_some(above - 1),
                    (!border_top).then_some(above),
                    (!border_top && !border_right).then_some(above + 1),
                    (!border_left).then_some(index - 1),
                    (!border_right).then_some(index + 1),
                    (!border_bottom && !border_left).then_some(below - 1),
                    (!border_bottom).then_some(below),
                    (!border_bottom && !border_right).then_some(below + 1),
                ]
                .into_iter()
                .filter(|x| x.is_some_and(|y| grid[y as usize]))
                .count();
                paper_count < 4
            })
            .map(|(i, _)| i)
            .collect();
        if to_remove.is_empty() {
            break;
        }

        removed_rolls += to_remove.len();

        for pos in to_remove {
            grid[pos] = false;
        }
    }

    removed_rolls
}

const INPUT: &str = include_str!("../../inputs/day_04.txt");

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
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 13);
    }

    const EXAMPLE_INPUT_2: &str = EXAMPLE_INPUT_1;

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 43);
    }
}
