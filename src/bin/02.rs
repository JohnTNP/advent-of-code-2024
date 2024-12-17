use std::{fmt::Debug, str::FromStr};

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let reports: Vec<Vec<u64>> = extract_2d_vectors(input);

    let mut safe_reports = 0;
    for report in &reports {
        let mut is_report_safe = true;
        let mut prev_direction = 0;

        for (i, level) in report.iter().enumerate() {
            if !is_report_safe {
                continue;
            }
            if i == 0 {
                continue;
            }

            let diff = (*level as i64) - (report[i - 1] as i64);
            if diff.abs() > 3 {
                is_report_safe = false;
                continue;
            }

            match diff {
                d if d < 0 => {
                    if prev_direction == 1 {
                        is_report_safe = false;
                        continue;
                    }
                    prev_direction = -1;
                }
                d if d > 0 => {
                    if prev_direction == -1 {
                        is_report_safe = false;
                        continue;
                    }

                    prev_direction = 1;
                }
                0 => {
                    is_report_safe = false;
                    continue;
                }
                _ => panic!("Unexpected diff amount."),
            }
        }

        if is_report_safe {
            safe_reports += 1;
        }
    }

    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u64> {
    let reports: Vec<Vec<u64>> = extract_2d_vectors(input);

    None
}

fn extract_2d_vectors<T>(input: &str) -> Vec<Vec<T>>
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    input
        .lines()
        .map(|row| {
            row.split_whitespace()
                .map(|item| item.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
