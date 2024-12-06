use std::collections::HashSet;
use std::iter::Cycle;
use std::slice::Iter;
use std::str::FromStr;

const DIRECTIONS: [(isize, isize); 4] = [
    (-1, 0), // Up
    (0, 1),  // Right
    (1, 0),  // Down
    (0, -1), // Left
];

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Coordinate(usize, usize);

impl Coordinate {
    fn transform(&self, x: isize, y: isize) -> Option<Self> {
        let new_x = (self.0 as isize).checked_add(x)? as usize;
        let new_y = (self.1 as isize).checked_add(y)? as usize;

        Some(Self(new_x, new_y))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Tile {
    Ground,
    Obstacle,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' | '^' => Tile::Ground,
            '#' => Tile::Obstacle,
            _ => unreachable!("Invalid character"),
        }
    }
}

pub enum WalkResult {
    Ok(Coordinate),
    Obstacle,
    OutOfBounds,
}

pub struct Map {
    height: usize,
    width: usize,
    environment: Vec<Vec<Tile>>,
}

impl Map {
    fn walk_from(&self, coordinate: Coordinate, direction: &(isize, isize)) -> WalkResult {
        let Some(coordinate) = coordinate.transform(direction.0, direction.1) else {
            return WalkResult::OutOfBounds;
        };

        if coordinate.0 >= self.height || coordinate.1 >= self.width {
            return WalkResult::OutOfBounds;
        }

        if self.environment[coordinate.0][coordinate.1] == Tile::Obstacle {
            return WalkResult::Obstacle;
        }

        WalkResult::Ok(coordinate)
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Map, ()> {
        let environment: Vec<Vec<Tile>> = s
            .lines()
            .map(|line| line.trim().chars().map(Tile::from).collect::<Vec<Tile>>())
            .collect();

        let height = environment.len();
        let width = environment.first().map(|f| f.len()).expect("No map");

        Ok(Self {
            height,
            width,
            environment,
        })
    }
}

pub struct Lab {
    map: Map,
    start_at: Coordinate,
}

impl Lab {
    pub fn find_guard_route_visits(&self) -> usize {
        let mut visits = HashSet::new();
        let mut cycle = DIRECTIONS.iter().cycle(); // Cycle directions for when we hit obstacles
        let mut guard_pos = self.start_at;

        let Some(next_direction) = cycle.next() else {
            unreachable!("Failed to get next direction!");
        };

        let mut direction = *next_direction;

        loop {
            visits.insert(guard_pos);

            if !self.walk_step(&mut cycle, &mut guard_pos, &mut direction, None) {
                break;
            }
        }

        visits.len()
    }

    fn walk_step(
        &self,
        cycle: &mut Cycle<Iter<(isize, isize)>>,
        guard_pos: &mut Coordinate,
        direction: &mut (isize, isize),
        added_obstacle: Option<Coordinate>,
    ) -> bool {
        match self.map.walk_from(*guard_pos, direction) {
            WalkResult::Ok(coordinate) => {
                // If we simulate an added obstacle, we need to change directions if we hit it instead of giving a new coordinate
                if let Some(obstacle) = added_obstacle {
                    if obstacle == coordinate {
                        let Some(next_direction) = cycle.next() else {
                            unreachable!("Failed to get next direction!");
                        };
                        *direction = *next_direction;
                        return true;
                    }
                }

                *guard_pos = coordinate;
            }
            WalkResult::Obstacle => {
                let Some(next_direction) = cycle.next() else {
                    unreachable!("Failed to get next direction!");
                };
                *direction = *next_direction;
            }
            WalkResult::OutOfBounds => {
                return false;
            }
        }

        true
    }

    pub fn find_route_loops(&self) -> usize {
        let mut looping_obstacles = HashSet::new();
        let mut cycle = DIRECTIONS.iter().cycle();

        let mut guard_pos = self.start_at;

        let Some(next_direction) = cycle.next() else {
            unreachable!("Failed to get next direction!");
        };

        let mut direction = *next_direction;

        loop {
            let next_coord = match self.map.walk_from(guard_pos, &direction) {
                WalkResult::Ok(coord) => coord,
                WalkResult::OutOfBounds => break, // We're done checking the different routes
                WalkResult::Obstacle => {
                    // Get and set next direction and continue looping
                    let Some(next_direction) = cycle.next() else {
                        unreachable!("Failed to get next direction!");
                    };

                    direction = *next_direction;

                    continue;
                }
            };

            let can_set_obstacle =
                !looping_obstacles.contains(&next_coord) && next_coord != self.start_at;

            if can_set_obstacle {
                let mut dir_cycle = DIRECTIONS.iter().cycle(); // Clone this, as we need to be at the same cycle.

                // Get and set next direction and continue looping
                let Some(next_direction) = dir_cycle.next() else {
                    unreachable!("Failed to get next direction!");
                };

                let obstacle = next_coord;
                let mut guard = self.start_at;
                let mut dir = *next_direction;

                let mut visits = HashSet::new();

                // Simulate new route
                loop {
                    // If we try to visit something already visited, with the same direction, we have found a loop
                    if visits.contains(&(guard, dir)) {
                        looping_obstacles.insert(next_coord);
                        break;
                    } else {
                        visits.insert((guard, dir));
                    }

                    if !self.walk_step(&mut dir_cycle, &mut guard, &mut dir, Some(obstacle)) {
                        break;
                    }
                }
            }

            guard_pos = next_coord;
        }

        looping_obstacles.len()
    }
}

impl FromStr for Lab {
    type Err = ();

    fn from_str(s: &str) -> Result<Lab, ()> {
        let s = s.trim();

        let start_at = s
            .lines()
            .enumerate()
            .filter_map(|(i, l)| Some(Coordinate(i, l.trim().match_indices('^').next()?.0)))
            .next()
            .expect("No guard found");

        let map = Map::from_str(s)?;

        Ok(Self { map, start_at })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#..."#;

    #[test]
    fn solution_1() {
        let lab = Lab::from_str(EXAMPLE).expect("Failed to parse lab");
        assert_eq!(lab.find_guard_route_visits(), 41);
    }

    #[test]
    fn solution_2() {
        let lab = Lab::from_str(EXAMPLE).expect("Failed to parse lab");
        assert_eq!(lab.find_route_loops(), 6)
    }
}
