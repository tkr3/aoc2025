use std::collections::HashMap;

type CoordComp = i64;
type Coord = (CoordComp, CoordComp, CoordComp);

pub fn part1(input: &str, connections: usize) -> usize {
    let junction_boxes: Vec<Coord> = input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|x| x.parse::<CoordComp>().unwrap());
            (
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect();

    let mut distances: Vec<(usize, CoordComp)> = junction_boxes
        .iter()
        .flat_map(|x| junction_boxes.iter().map(move |y| (x, y)))
        .enumerate()
        .filter(|(_, (x, y))| x != y)
        .map(|(i, (x, y))| (i, euclidean_distance(x, y)))
        .collect();
    distances.sort_unstable_by_key(|x| x.1);

    let mut circuits: HashMap<usize, usize> = HashMap::with_capacity(connections);
    let mut circuit_id = 0;

    for (i, _distance) in distances.into_iter().step_by(2).take(connections) {
        let coord_a = i % junction_boxes.len();
        let coord_b = i / junction_boxes.len();

        let circuit_id_a = circuits.get(&coord_a);
        let circuit_id_b = circuits.get(&coord_b);

        match (circuit_id_a, circuit_id_b) {
            // merge circuits
            (Some(a), Some(b)) => {
                let a = *a;
                let b = *b;
                for (_, id) in circuits.iter_mut() {
                    if *id == b {
                        *id = a;
                    }
                }
            }
            // add to circuit
            (Some(a), None) => {
                circuits.insert(coord_b, *a);
            }
            (None, Some(b)) => {
                circuits.insert(coord_a, *b);
            }
            // create new circuit
            (None, None) => {
                circuits.insert(coord_a, circuit_id);
                circuits.insert(coord_b, circuit_id);
                circuit_id += 1;
            }
        }
    }
    let mut circuit_boxes = vec![0; circuit_id];
    for (_, id) in circuits.into_iter() {
        circuit_boxes[id] += 1;
    }
    circuit_boxes.sort_unstable();
    circuit_boxes.iter().rev().take(3).product()
}

pub fn part2(input: &str) -> CoordComp {
    let junction_boxes: Vec<Coord> = input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|x| x.parse::<CoordComp>().unwrap());
            (
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect();

    let mut distances: Vec<(usize, CoordComp)> = junction_boxes
        .iter()
        .flat_map(|x| junction_boxes.iter().map(move |y| (x, y)))
        .enumerate()
        .filter(|(_, (x, y))| x != y)
        .map(|(i, (x, y))| (i, euclidean_distance(x, y)))
        .collect();
    distances.sort_unstable_by_key(|x| x.1);

    let mut circuits: HashMap<usize, usize> = HashMap::new();
    let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();
    let mut circuit_id = 0;

    for (i, _distance) in distances.into_iter().step_by(2) {
        let coord_a = i % junction_boxes.len();
        let coord_b = i / junction_boxes.len();

        let circuit_id_a = circuits.get(&coord_a);
        let circuit_id_b = circuits.get(&coord_b);

        if match (circuit_id_a, circuit_id_b) {
            // merge circuits
            (Some(a), Some(b)) if a != b => {
                *circuit_sizes.get_mut(a).unwrap() += circuit_sizes[b];
                let a = *a;
                let b = *b;
                for (_, id) in circuits.iter_mut() {
                    if *id == b {
                        *id = a;
                    }
                }
                circuit_sizes[&a]
            }
            // add to circuit
            (Some(a), None) => {
                let size = circuit_sizes[a] + 1;
                circuit_sizes.insert(*a, size);
                circuits.insert(coord_b, *a);
                size
            }
            (None, Some(b)) => {
                let size = circuit_sizes[b] + 1;
                circuit_sizes.insert(*b, size);
                circuits.insert(coord_a, *b);
                size
            }
            // create new circuit
            (None, None) => {
                circuits.insert(coord_a, circuit_id);
                circuits.insert(coord_b, circuit_id);
                circuit_sizes.insert(circuit_id, 2);
                circuit_id += 1;
                2
            }
            _ => 0,
        } == junction_boxes.len()
        {
            return junction_boxes[coord_a].0 * junction_boxes[coord_b].0;
        }
    }
    unreachable!();
}

fn euclidean_distance(x: &Coord, y: &Coord) -> CoordComp {
    (x.0 - y.0).pow(2) + (x.1 - y.1).pow(2) + (x.2 - y.2).pow(2)
}

const INPUT: &str = include_str!("../../inputs/day_08.txt");

fn main() {
    let p1 = part1(INPUT, 1000);
    println!("Part 1: {p1}");

    let p2 = part2(INPUT);
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1, 10), 40);
    }

    const EXAMPLE_INPUT_2: &str = EXAMPLE_INPUT_1;

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 25272);
    }
}
