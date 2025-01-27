use std::str::FromStr;
use advent_of_code::U64Matrix;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let reports = U64Matrix::from_str(input).unwrap().data;

    let mut safe_reports = 0;
    for levels in &reports {
        if check_level(levels) {
            safe_reports += 1;
        }
    }

    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u64> {
    let reports = U64Matrix::from_str(input).unwrap().data;

    let mut safe_reports = 0;
    for levels in &reports {
        if check_level(levels) {
            safe_reports += 1;
        } else {
            for (i, _) in levels.iter().enumerate() {
                let dampered_vec = get_dampered_vec(levels, i);
                if check_level(&dampered_vec) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }

    Some(safe_reports)
}

fn check_level(levels: &Vec<u64>) -> bool {
    let mut is_report_safe = true;
    let mut prev_direction = 0;

    for (i, level) in levels.iter().enumerate() {
        if !is_report_safe {
            continue;
        }
        if i == 0 {
            continue;
        }

        let diff = (*level as i64) - (levels[i - 1] as i64);
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

    is_report_safe
}

fn get_dampered_vec(levels: &Vec<u64>, i: usize) -> Vec<u64> {
    levels
        .into_iter()
        .enumerate()
        .filter(|(index, _)| *index != i)
        .map(|(_, val)| *val)
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
        assert_eq!(result, Some(4));
    }
}
