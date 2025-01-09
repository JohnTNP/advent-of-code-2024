use std::str::FromStr;

pub mod template;

// Use this file to add helper functions and additional modules.

/// A struct representing a nested vector of `u64` values.
pub struct U64Matrix {
    /// The nested vector of `u64` values.
    pub data: Vec<Vec<u64>>,
}

impl FromStr for U64Matrix {
    type Err = String;

    /// Parses a string into an `InputVec`.
    ///
    /// The input string should consist of lines, each containing comma-separated `u64` values.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the values cannot be parsed as `u64`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Result<Vec<Vec<u64>>, _> = s.lines()
            .map(|line| {
                line.split(',')
                    .map(|num_str| num_str.trim().parse::<u64>())
                    .collect()
            })
            .collect();

        match data {
            Ok(data) => Ok(U64Matrix { data }),
            Err(_) => Err("Failed to parse input string".to_string()),
        }
    }
}
