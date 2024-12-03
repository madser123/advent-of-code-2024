use std::str::FromStr;

/// Failed to parse multiplication
pub struct ParseError;

pub struct Multiplication {
    enabled: bool,
    a: usize,
    b: usize,
}

impl Multiplication {
    pub const fn get_result(&self) -> usize {
        self.a * self.b
    }

    pub const fn get_result_conditional(&self) -> Option<usize> {
        if self.enabled {
            return Some(self.get_result());
        }

        None
    }
}

pub struct Calculations(Vec<Multiplication>);

impl Calculations {
    pub fn sum(&self) -> usize {
        self.0.iter().map(|m| m.get_result()).sum()
    }

    pub fn sum_conditional(&self) -> usize {
        self.0
            .iter()
            .filter_map(|m| m.get_result_conditional())
            .sum()
    }
}

impl FromStr for Calculations {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut do_matches = s.match_indices("do()").map(|(idx, _)| idx);
        let mut do_not_matches = s.match_indices("don't()").map(|(idx, _)| idx);
        let mul_matches = s.match_indices("mul(");

        let mut next_do = do_matches.next().unwrap_or(s.len());
        let mut next_do_not = do_not_matches.next().unwrap_or(s.len());

        let mut enabled = true;

        let calculations = mul_matches
            .filter_map(|(idx, _)| {
                println!("Next do: {next_do} | Next don't: {next_do_not} | {idx}");

                if next_do > next_do_not && next_do_not <= idx {
                    next_do_not = do_not_matches.next().unwrap_or(s.len());
                    enabled = false;
                } else if next_do_not > next_do && next_do <= idx {
                    next_do = do_matches.next().unwrap_or(s.len());
                    enabled = true;
                }

                println!("{idx} {enabled}");

                let start = idx + 4;
                let end = s[start..].find(')')? + start;

                // Parse multiplication
                let mut split = s[start..end].split(',');
                let a = split.next()?.parse().ok()?;
                let b = split.next()?.parse().ok()?;

                Some(Multiplication { enabled, a, b })
            })
            .collect();

        Ok(Self(calculations))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn solution_1() {
        let calcs = Calculations::from_str(EXAMPLE_1).expect("Failed to parse calculations");
        let sum = calcs.sum();

        assert_eq!(sum, 161);
    }

    #[test]
    fn solution_2() {
        let calcs = Calculations::from_str(EXAMPLE_2).expect("Failed to parse calculations");
        let sum = calcs.sum_conditional();

        assert_eq!(sum, 48)
    }
}
