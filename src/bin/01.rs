advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let [mut lefts,mut rights] = match extract_lefts_and_rights(input) {
        Some(value) => value,
        None => panic!("Invalid input"),
    };

    lefts.sort();
    rights.sort();

    let mut sum = 0;
    for (i, left) in lefts.iter().enumerate() {
        sum += u64::abs_diff(*left, rights[i]);
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let [lefts, rights] = match extract_lefts_and_rights(input) {
        Some(value) => value,
        None => panic!("Invalid input"),
    };

    let mut similarity_score = 0;

    for left in &lefts {
        let mut n_appearance = 0;
        for right in &rights {
            if left == right  {
                n_appearance += 1;
            }
        }

        similarity_score += left * n_appearance;
    }

    Some(similarity_score)
}

/// Extracts lefts and rights as vectors from input string.
fn extract_lefts_and_rights(input: &str) -> Option<[Vec<u64>; 2]> {
    let mut lefts: Vec<u64> = vec![];
    let mut rights: Vec<u64> = vec![];

    for list in input.split("\n").into_iter() {
        let splits: Vec<&str> = list.split_whitespace().collect();
        let left_value: u64 = match splits.get(0) {
            Some(value) => match value.parse() {
                Ok(value) => value,
                Err(_) => return None,
            },
            None => return None,
        };
        let right_value: u64 = match splits.get(1) {
            Some(value) => match value.parse() {
                Ok(value) => value,
                Err(_) => return None,
            },
            None => return None,
        };

        lefts.push(left_value);
        rights.push(right_value);
    }

    Some([lefts, rights])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }

    #[test]
    #[should_panic]
    fn test_part_one_invalid_input() {
        let result = part_one("");
        assert_eq!(result, None);
    }

    #[test]
    #[should_panic]
    fn test_part_two_invalid_input() {
        let result = part_two("");
        assert_eq!(result, None);
    }
}
