pub enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32),
}
pub fn str_to_direction(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|str| str.splitn(2, ' '))
        .map(|mut splitn| {
            (
                splitn.next().unwrap(),
                splitn.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .map(|(dir, dist)| match dir {
            "forward" => Direction::Forward(dist),
            "up" => Direction::Up(dist),
            "down" => Direction::Down(dist),
            _ => unreachable!(),
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (x, y) = str_to_direction(input)
        .iter()
        .fold((0, 0), |(x, z), dir| match dir {
            Direction::Forward(dist) => (x + dist, z),
            Direction::Down(dist) => (x, z + dist),
            Direction::Up(dist) => (x, z - dist),
        });
    Some(x * y)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (x, y, aim) =
        str_to_direction(input)
            .iter()
            .fold((0, 0, 0), |(x, z, aim), dir| match dir {
                Direction::Forward(dist) => (x + dist, z + aim * dist, aim),
                Direction::Down(dist) => (x, z, aim + dist),
                Direction::Up(dist) => (x, z, aim - dist),
            });
    Some(x * y)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(150));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(900));
    }
}
