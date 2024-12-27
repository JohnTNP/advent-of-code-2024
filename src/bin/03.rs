advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;

    let mut mul_splits: Vec<&str> = input.split("mul(").collect();
    mul_splits.remove(0);

    for c in mul_splits {
        let nums_str = c.split(")")
            .into_iter()
            .collect::<Vec<&str>>();

        if nums_str.get(1).is_none() { continue; }

        let nums = nums_str[0].split(",")
            .into_iter()
            .map(|x| {
                match x.parse::<u64>() {
                    Ok(num) => num,
                    Err(_) => 0,
                }
            })
            .collect::<Vec<u64>>();

        if let (Some(first), Some(second)) = (nums.get(0), nums.get(1)) {
            result += first * second;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
