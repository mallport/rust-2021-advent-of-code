pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .windows(2)
            .filter(|slice| slice[0] < slice[1])
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .windows(4)
            .filter(|slice| slice[0] < slice[3])
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
