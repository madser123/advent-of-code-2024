use std::str::FromStr;

const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),   // Above
    (1, 1),   // Upper right
    (0, 1),   // Right
    (-1, 1),  // Lower right
    (-1, 0),  // Below
    (-1, -1), // Lower left
    (0, -1),  // Left
    (1, -1),  // Upper left
];

const DIAGONALS: [(isize, isize); 4] = [
    (1, -1),  // Upper left
    (-1, 1),  // Lower right
    (-1, -1), // Lower left
    (1, 1),   // Upper right
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate(pub usize, pub usize);

impl Coordinate {
    pub fn transform(&self, x: isize, y: isize) -> Option<Self> {
        let new_x = (self.0 as isize).checked_add(x)? as usize;
        let new_y = (self.1 as isize).checked_add(y)? as usize;

        Some(Self(new_x, new_y))
    }
}

pub struct LetterMatrix(Vec<String>);

impl LetterMatrix {
    pub fn find_coordinates_for(&self, letter: char) -> Vec<Coordinate> {
        self.0
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut acc, (row, string)| {
                string
                    .match_indices(letter)
                    .for_each(|(idx, _)| acc.push(Coordinate(row, idx)));

                acc
            })
    }

    pub fn get_coord(&self, coord: Coordinate) -> Option<char> {
        self.0.get(coord.0).and_then(|row| row.chars().nth(coord.1))
    }
}

impl FromStr for LetterMatrix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matrix = s
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect();

        Ok(Self(matrix))
    }
}

pub struct WordSearch {
    matrix: LetterMatrix,
}

impl WordSearch {
    pub fn new(words: &str) -> Self {
        let matrix = LetterMatrix::from_str(words).expect("Failed to parse matrix");

        Self { matrix }
    }

    pub fn whole_word_sum(&self) -> usize {
        let start_coordinates = self.matrix.find_coordinates_for('X');

        let mut result = 0;

        let remaining = "AS";

        // Check each coordinate for first letter
        for coord in start_coordinates {
            for (trans_x, trans_y) in &DIRECTIONS {
                let Some(mut current_coord) = coord.transform(*trans_x, *trans_y) else {
                    continue;
                };
                let mut current_char = 'M';
                let mut next_char_idx = 0;

                while let Some(found) = self.matrix.get_coord(current_coord) {
                    if found == current_char {
                        let Some(next) = remaining.chars().nth(next_char_idx) else {
                            result += 1;
                            break;
                        };

                        let Some(next_coord) = current_coord.transform(*trans_x, *trans_y) else {
                            break;
                        };

                        current_coord = next_coord;
                        next_char_idx += 1;
                        current_char = next;
                    } else {
                        break;
                    }
                }
            }
        }

        result
    }

    pub fn xmas_sum(&self) -> usize {
        let start_coordinates = self.matrix.find_coordinates_for('A');
        let allowed_patterns = ["MSSM", "SMMS", "MSMS", "SMSM"]; // Allowed patterns when checking diagonals in order

        let mut result = 0;

        for coord in start_coordinates {
            let mut pattern = String::new();
            for (trans_x, trans_y) in &DIAGONALS {
                let Some(current_coord) = coord.transform(*trans_x, *trans_y) else {
                    break;
                };

                let Some(found) = self.matrix.get_coord(current_coord) else {
                    break;
                };

                pattern.push(found);
            }

            if allowed_patterns.iter().any(|pat| pattern.contains(pat)) {
                result += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX"#;

    #[test]
    fn solution_1() {
        let word_search = WordSearch::new(EXAMPLE);

        assert_eq!(word_search.whole_word_sum(), 18);
    }

    #[test]
    fn solution_2() {
        let word_search = WordSearch::new(EXAMPLE);

        assert_eq!(word_search.xmas_sum(), 9);
    }
}