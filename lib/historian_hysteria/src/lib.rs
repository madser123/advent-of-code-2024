use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseError;

pub type LocationList = Vec<usize>;

#[derive(Debug)]
pub struct CompareLocations {
    left: LocationList,
    right: LocationList,
}

impl CompareLocations {
    pub fn compare(&self) -> usize {
        self.left.iter().enumerate().fold(0, |mut sum, (idx, num)| {
            sum += num.abs_diff(self.right[idx]);
            sum
        })
    }

    pub fn calculate_similarity(&self) -> usize {
        let mut right_hits = HashMap::new();
        self.right.iter().for_each(|num| {
            right_hits.entry(num).and_modify(|e| *e += 1).or_insert(1);
        });

        self.left
            .iter()
            .map(|num| {
                let x = *right_hits.get(num).unwrap_or(&0);
                num * x
            })
            .sum()
    }
}

impl FromStr for CompareLocations {
    type Err = ();

    fn from_str(input: &str) -> Result<CompareLocations, ()> {
        let (mut left, mut right) = input.lines().fold(
            (LocationList::new(), LocationList::new()),
            |(mut left, mut right), line| {
                let mut split = line.split_ascii_whitespace();

                let Some(l) = split.next().and_then(|l| usize::from_str(l).ok()) else {
                    return (left, right);
                };

                let Some(r) = split.next().and_then(|l| usize::from_str(l).ok()) else {
                    return (left, right);
                };

                left.push(l);
                right.push(r);

                (left, right)
            },
        );

        left.sort();
        right.sort();

        Ok(CompareLocations { left, right })
    }
}

#[cfg(test)]
mod tests {
    use crate::CompareLocations;
    use std::str::FromStr;

    const EXAMPLE_1: &str = r#"
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    "#;

    const EXAMPLE_2: &str = r#"
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    "#;

    #[test]
    fn solution_1() {
        let cmp = CompareLocations::from_str(EXAMPLE_1).unwrap();

        assert_eq!(cmp.compare(), 11)
    }

    #[test]
    fn solution_2() {
        let cmp = CompareLocations::from_str(EXAMPLE_2).unwrap();
        assert_eq!(cmp.calculate_similarity(), 31)
    }
}
