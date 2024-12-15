advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let [mut lefts, mut rights] = extract_lefts_and_rights(input);

    lefts.sort();
    rights.sort();

    let mut sum = 0;
    for (i, left) in lefts.iter().enumerate() {
        sum += u64::abs_diff(*left, rights[i]);
    }

    Some(sum)
}

#[allow(unused)]
pub fn part_two(input: &str) -> Option<u64> {
    let [mut lefts, mut rights] = extract_lefts_and_rights(input);

    None
}

/// Extracts lefts and rights as vectors from input string.
fn extract_lefts_and_rights(input: &str) -> [Vec<u64>; 2] {
    let mut lefts: Vec<u64> = vec![];
    let mut rights: Vec<u64> = vec![];

    for list in input.split("\n").into_iter() {
        let splits: Vec<&str> = list.split_whitespace().collect();
        lefts.push(splits[0].parse().unwrap());
        rights.push(splits[1].parse().unwrap());
    }

    [lefts, rights]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
