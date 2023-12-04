use itertools::Itertools;

#[derive(Clone, Debug)]
struct Corner {
    x: i16,
    y: i16,
}

#[derive(PartialEq, Debug)]
enum Direction {
    X(i16),
    Y(i16),
}

impl Corner {
    fn new(x: i16, y: i16, board: &[&str]) -> Option<Self> {
        if board[x as usize].chars().nth(y as usize).unwrap() != '+' {
            return None;
        }
        Some(Corner { x, y })
    }

    /// Checks whether two corners have at least one common coordinate
    ///
    fn check_edge_can_exist(&self, another: &Corner) -> Option<Direction> {
        if (self.x != another.x) && (self.y != another.y) {
            return None;
        }

        if self.x == another.x {
            Some(Direction::X(self.x))
        } else {
            Some(Direction::Y(self.y))
        }
    }

    /// Strictly checks whether two corners are connected with an edge
    ///
    fn check_forms_an_edge(&self, another: &Corner, board: &[&str]) -> bool {
        // Check that edge can exists
        let fixed_coord = match self.check_edge_can_exist(another) {
            Some(coord) => coord,
            None => return false,
        };

        let has_an_edge: bool = match fixed_coord {
            Direction::X(coord) => {
                let start = self.y.min(another.y) as usize;
                let end = self.y.max(another.y) as usize;
                board[coord as usize][start..=end]
                    .chars()
                    .all(|char| char == '-' || char == '+')
            }
            Direction::Y(coord) => {
                let start = self.x.min(another.x) as usize;
                let end = self.x.max(another.x) as usize;

                board[start..=end]
                    .iter()
                    .map(|line| line.chars().nth(coord as usize).unwrap())
                    .all(|char| char == '|' || char == '+')
            }
        };

        has_an_edge
    }
}

pub fn count(lines: &[&str]) -> u32 {
    let mut total_rectangles = 0;

    for i in 0..lines.len() {
        let curr_line_corners = lines[i]
            .chars()
            .enumerate()
            .filter(|(_, ch)| *ch == '+')
            .map(|(idx, _)| Corner::new(i as i16, idx as i16, lines).unwrap());

        for corners in curr_line_corners
            .combinations(2)
            .filter(|corners_pair| corners_pair[0].check_forms_an_edge(&corners_pair[1], lines))
        {
            for x in ((corners[0].x + 1) as usize)..lines.len() {
                let corner1 = match Corner::new(x as i16, corners[0].y, lines) {
                    Some(corner) => corner,
                    None => continue,
                };

                let corner2 = match Corner::new(x as i16, corners[1].y, lines) {
                    Some(corner) => corner,
                    None => continue,
                };

                if corner1.check_forms_an_edge(&corner2, lines)
                    && corner1.check_forms_an_edge(&corners[0], lines)
                    && corner2.check_forms_an_edge(&corners[1], lines)
                {
                    total_rectangles += 1;
                }
            }
        }
    }

    total_rectangles
}

#[cfg(test)]
mod tests {
    // Allows to test private functions
    use super::*;

    #[test]
    fn test_check_edge_can_exist_1() {
        assert_eq!(
            Corner { x: 1, y: 2 }.check_edge_can_exist(&Corner { x: 5, y: 2 }),
            Some(Direction::Y(2))
        );
    }

    #[test]
    fn test_check_edge_can_exist_2() {
        assert_eq!(
            Corner { x: 7, y: 5 }.check_edge_can_exist(&Corner { x: 7, y: 3 }),
            Some(Direction::X(7))
        );
    }

    #[test]
    fn test_check_edge_can_exist_3() {
        assert_eq!(
            Corner { x: 8, y: 5 }.check_edge_can_exist(&Corner { x: 7, y: 3 }),
            None
        );
    }

    #[test]
    fn test_check_forms_an_edge_1() {
        let lines = &[
            //             10
            //    1 3 5 7 9|11
            //   0 2 4 6 8 ||12
            "+------+----+", // 0
            "|      |    |", // 1
            "+------+    |", // 2
            "|   |       |", // 3
            "+---+-------+", // 4
        ];

        assert!(!Corner { x: 8, y: 5 }.check_forms_an_edge(&Corner { x: 7, y: 3 }, lines));
    }

    #[test]
    fn test_check_forms_an_edge_2() {
        let lines = &[
            //             10
            //    1 3 5 7 9|11
            //   0 2 4 6 8 ||12
            "+------+----+", // 0
            "|      |    |", // 1
            "+------+    |", // 2
            "|   |       |", // 3
            "+---+-------+", // 4
        ];

        assert!(Corner { x: 0, y: 0 }.check_forms_an_edge(&Corner { x: 0, y: 7 }, lines));
    }

    #[test]
    fn test_check_forms_an_edge_3() {
        let lines = &[
            //             10
            //    1 3 5 7 9|11
            //   0 2 4 6 8 ||12
            "+------+----+", // 0
            "|      |    |", // 1
            "+------+    |", // 2
            "|   |       |", // 3
            "+---+--+----+", // 4
        ];

        assert!(Corner { x: 2, y: 0 }.check_forms_an_edge(&Corner { x: 4, y: 0 }, lines));
        assert!(!Corner { x: 2, y: 3 }.check_forms_an_edge(&Corner { x: 4, y: 3 }, lines));
    }

    #[test]
    fn test_check_forms_an_edge_4() {
        let lines = &[
            //             10
            //    1 3 5 7 9|11
            //   0 2 4 6 8 ||12
            "+------+----+", // 0
            "|      |    |", // 1
            "+------+--+ +", // 2
            "|   |       |", // 3
            "+---+-------+", // 4
        ];

        assert!(Corner { x: 2, y: 7 }.check_forms_an_edge(&Corner { x: 2, y: 10 }, lines));
        assert!(!Corner { x: 2, y: 7 }.check_forms_an_edge(&Corner { x: 2, y: 12 }, lines));
    }
}
