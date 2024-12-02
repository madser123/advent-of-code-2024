use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Increasing,
    Decreasing,
}

#[derive(Debug)]
pub struct Report(Vec<usize>);

impl Report {
    pub fn is_safe(&self, dampen: bool) -> bool {
        let mut windows = self.0.windows(2);

        // Check if any of the windows doesn't fall within the rules
        if dampen {
            let mut illegals = HashSet::new();
            let mut increasing = HashSet::new();
            let mut decreasing = HashSet::new();

            let first = self.0[0];
            let last = self.0.last().expect("No last value");
            let diff = first.abs_diff(*last);
            let max_diff = (0..diff).step_by(3).size_hint().0;

            let mut biggest_diff = 0;

            windows.enumerate().for_each(|(idx, nums)| {
                let (x, y) = (nums[0], nums[1]);
                let diff = x.abs_diff(y);

                if diff > biggest_diff {
                    biggest_diff = diff;
                }

                if !(diff != 0 && diff <= 3) {
                    illegals.insert(idx);
                }

                match Self::get_direction(x, y) {
                    Direction::Increasing => {
                        increasing.insert(idx);
                    }
                    Direction::Decreasing => {
                        decreasing.insert(idx);
                    }
                }
            });

            println!(
                "({self:?}) max_diff: {max_diff}, biggest_diff: {biggest_diff}, illegals: {illegals:?}, increasing: {increasing:?}, decreasing: {decreasing:?}",
            );

            if illegals.len() > 1 {
                return false;
            }

            if increasing.len() > 1 && decreasing.len() > 1 {
                return false;
            }

            if biggest_diff > max_diff {
                return false;
            }

            true
        } else {
            let (first_x, first_y) = (self.0[0], self.0[1]);
            let first_direction = Self::get_direction(first_x, first_y);
            !windows.any(|nums| {
                let (x, y) = (nums[0], nums[1]);
                let diff = x.abs_diff(y);

                first_direction != Self::get_direction(x, y) || !(diff != 0 && diff <= 3)
            })
        }
    }

    fn get_direction(x: usize, y: usize) -> Direction {
        if x > y {
            Direction::Decreasing
        } else {
            Direction::Increasing
        }
    }
}

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<usize> = s
            .split_ascii_whitespace()
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect();

        if nums.is_empty() {
            return Err(());
        }

        Ok(Report(nums))
    }
}

pub struct Reports(Vec<Report>);

impl Reports {
    pub fn get_amount_of_safe_reports(&self) -> usize {
        self.0.iter().filter(|&a| a.is_safe(false)).count()
    }

    pub fn get_amount_of_dampened_safe_reports(&self) -> usize {
        self.0
            .iter()
            .filter(|&a| {
                let is_safe = a.is_safe(true);
                println!("{a:?} = {is_safe}");
                is_safe
            })
            .count()
    }
}

impl FromStr for Reports {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .filter_map(|line| Report::from_str(line).ok())
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::Reports;
    use std::str::FromStr;

    const EXAMPLE: &str = r#"
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9"#;

    #[test]
    pub fn solution_1() {
        let reports = Reports::from_str(EXAMPLE).expect("failed to parse reports");

        assert_eq!(reports.get_amount_of_safe_reports(), 2);
    }

    #[test]
    pub fn solution_2() {
        let reports = Reports::from_str(EXAMPLE).expect("failed to parse reports");

        assert_eq!(reports.get_amount_of_dampened_safe_reports(), 4);
    }
}
