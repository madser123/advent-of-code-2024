use std::collections::HashMap;
use std::str::FromStr;

type OrderingRules = HashMap<usize, Vec<usize>>;

#[derive(Debug)]
pub struct Update {
    pages: Vec<usize>,
}

impl Update {
    pub fn middle_value(&self) -> usize {
        let middle = self.pages.len() / 2;
        self.pages[middle]
    }

    pub fn is_invalid(&self, rules: &OrderingRules) -> bool {
        !self.is_valid(rules)
    }

    pub fn is_valid(&self, rules: &OrderingRules) -> bool {
        let mut stack: Vec<usize> = Vec::with_capacity(self.pages.len());

        self.pages.iter().all(|page| {
            if let Some(must_come_before) = rules.get(page) {
                if must_come_before.iter().any(|num| stack.contains(num)) {
                    return false;
                }
            }

            stack.push(*page);

            true
        })
    }

    pub fn correct(&self, rules: &OrderingRules) -> Self {
        let mut stack: Vec<usize> = Vec::with_capacity(self.pages.len());

        self.pages.iter().for_each(|page| {
            if let Some(must_come_before) = rules.get(page) {
                if let Some(before_pos) = must_come_before
                    .iter()
                    .filter_map(|num| stack.iter().position(|x| x == num))
                    .min()
                {
                    stack.insert(before_pos, *page);
                } else {
                    stack.push(*page);
                }
            }
        });

        let new = Self { pages: stack };

        new
    }
}

impl FromStr for Update {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pages = s
            .split(',')
            .map(|s| s.parse().expect("Failed to parse ordering number"))
            .collect();
        Ok(Self { pages })
    }
}

#[derive(Debug)]
pub struct PrintQueue {
    rules: OrderingRules,
    updates: Vec<Update>,
}

impl PrintQueue {
    pub fn incorrect_updates_sum(&self) -> usize {
        self.updates
            .iter()
            .filter(|&u| u.is_invalid(&self.rules))
            .map(|u| u.correct(&self.rules).middle_value())
            .sum()
    }

    pub fn correct_updates_sum(&self) -> usize {
        self.updates
            .iter()
            .filter(|&u| u.is_valid(&self.rules))
            .map(|u| u.middle_value())
            .sum()
    }
}

impl FromStr for PrintQueue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = OrderingRules::new();
        let mut updates = Vec::new();

        for line in s.lines().map(|l| l.trim()) {
            if line.contains('|') {
                let mut split = line
                    .split('|')
                    .map(|num| num.parse().expect("Failed to parse ordering number"));
                let before = split.next().expect("No before number in rule");
                let after = split.next().expect("No after number in rule");
                let rule = rules.entry(before).or_default();
                rule.push(after);
            } else if line.contains(",") {
                updates.push(Update::from_str(line)?);
            }
        }

        Ok(Self { rules, updates })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47"#;

    #[test]
    fn solution_1() {
        let queue = PrintQueue::from_str(EXAMPLE).expect("Failed to parse printer queue");
        assert_eq!(queue.correct_updates_sum(), 143);
    }

    #[test]
    fn solution_2() {
        let queue = PrintQueue::from_str(EXAMPLE).expect("Failed to parse printer queue");
        assert_eq!(queue.incorrect_updates_sum(), 123);
    }
}
