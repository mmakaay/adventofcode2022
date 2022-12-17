//use std::cmp::max;
use crate::Direction::*;
use crate::View::*;

#[derive(Debug)]
#[allow(dead_code)]
enum View {
    UnknownView,
    Free(usize),
    Blocked(usize),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Tree {
    pub height: u8,
    n: View,
    e: View,
    s: View,
    w: View,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Forest {
    cols: usize,
    rows: usize,
    trees: Vec<Vec<Tree>>,
}

#[derive(Debug)]
enum Direction {
    Start,
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
pub struct ForestWalker {
    cols_from: usize,
    cols_to: usize,
    rows_from: usize,
    rows_to: usize,
    i_col: usize,
    i_row: usize,
    direction: Direction,
}

impl ForestWalker {
    fn new(forest: &Forest) -> Self {
        Self {
            cols_from: 0,
            cols_to: forest.cols - 1,
            rows_from: 0,
            rows_to: forest.rows - 1,
            i_col: 0,
            i_row: 0,
            direction: Start,
        }
    }

    fn change_direction(&mut self, direction: Direction) -> bool {
        match self.direction {
            Start => (),
            _ => match direction {
                North => self.rows_to -= 1,
                South => self.rows_from += 1,
                East => self.cols_from += 1,
                West => self.cols_to -= 1,
                Start => (),
            },
        }
        self.direction = direction;
        println!("{self:?}");
        return false;
    }

    fn go_east(&mut self) -> bool {
        if self.i_col < self.cols_to {
            self.i_col += 1;
            return true;
        }
        return false;
    }

    fn go_west(&mut self) -> bool {
        if self.i_col > self.cols_from {
            self.i_col -= 1;
            return true;
        }
        return false;
    }

    fn go_south(&mut self) -> bool {
        if self.i_row < self.rows_to {
            self.i_row += 1;
            return true;
        }
        return false;
    }

    fn go_north(&mut self) -> bool {
        if self.i_row > self.rows_from {
            self.i_row -= 1;
            return true;
        }
        return false;
    }
}

type ForestCoordinate = (usize, usize);

impl Iterator for ForestWalker {
    type Item = ForestCoordinate;

    fn next(&mut self) -> Option<(usize, usize)> {
        // Check if all forest positions have been visited.
        if self.cols_from == self.cols_to && self.rows_from == self.rows_to {
            return None;
        }

        while !match self.direction {
            Start => self.change_direction(East) || true,
            North => self.go_north() || self.change_direction(East),
            East => self.go_east() || self.change_direction(South),
            South => self.go_south() || self.change_direction(West),
            West => self.go_west() || self.change_direction(North),
        } {}
        let coordinate = (self.i_row, self.i_col);
        return Some(coordinate);
    }
}

impl Forest {
    fn new() -> Self {
        Self {
            cols: 0,
            rows: 0,
            trees: vec![],
        }
    }

    pub fn from_input(input: &str) -> Self {
        let mut forest = Self::make_forest(input);
        forest.inspect_forest_layout();
        return forest;
    }

    fn make_forest(input: &str) -> Forest {
        let mut forest = Forest::new();
        let rows = input
            .trim()
            .split("\n")
            .filter(|&line| line != "")
            .collect::<Vec<&str>>();

        forest.cols = rows.get(0).unwrap_or(&"").len();
        forest.rows = rows.len();

        for (id_row, line) in rows.iter().enumerate() {
            let row_of_trees = line
                .as_bytes()
                .iter()
                .enumerate()
                .map(|(id_col, &b)| Tree {
                    height: (b - b'0'),
                    n: if id_row == 0 { Free(0) } else { UnknownView },
                    e: if id_col == forest.cols - 1 {
                        Free(0)
                    } else {
                        UnknownView
                    },
                    s: if id_row == forest.rows - 1 {
                        Free(0)
                    } else {
                        UnknownView
                    },
                    w: if id_col == 0 { Free(0) } else { UnknownView },
                })
                .collect::<Vec<Tree>>();
            forest.trees.push(row_of_trees);
        }
        return forest;
    }

    // fn count_visible_trees(line_of_sight: &[Tree], height: u8) -> usize {
    //     let mut top = 0;
    //     let mut visible = 0;
    //     for tree in line_of_sight {
    //         if tree.height > top {
    //             top = tree.height;
    //             visible += 1;
    //         }
    //         if top >= height {
    //             break;
    //         }
    //     }
    //     return visible;
    // }

    fn inspect_forest_layout(&mut self) {
        let walker = ForestWalker::new(&self);
        for (i_row, i_col) in walker {
            println!("{i_row},{i_col}");
        }
        // let mut i_row = 0;
        // let mut i_col = 0;
        // let left_row
        // let step
        // for (i_row, row) in self.trees.iter().enumerate() {
        //     for (i_col, tree) in row.iter().enumerate() {
        //         if let UnknownView = tree.n {

        //         }
        //     }
        // }
    }

    //     // Check east/west sight lines.
    //     for (i_row, row) in self.rows.iter().enumerate() {
    //         for i in 0..self.width {
    //             let line_of_sight = &row[..i];
    //             let score *= Forest::count_visible_trees(line_of_sight, row[i].height);
    //             let hidden = line_of_sight.iter().any(|other| other.height >= row[i].height);
    //             row[i].scenic_score = score;
    //             if hidden {
    //                 row[i].visibility -= 1;
    //             }

    //             // let east = row[i + 1..]
    //             //     .iter()
    //             //     .take_while(|&other| !(other.height < row[i].height)
    //             //     .count();
    //             let east = 5;
    //             row[i].scenic_score *= east;
    //             if east < row.len() - i - 1 {
    //                 row[i].visibility -= 1;
    //             }
    //             if row[i+1..].iter().any(|other| other.height >= row[i].height) {
    //                 row[i].visibility -= 1;
    //             }
    //         }
    //     }

    //     // Check north/south sight lines.
    //     let mut transposed = vec![vec![0; self.height]; self.width];
    //     for (i, row) in self.rows.iter().enumerate() {
    //         for (j, tree) in row.iter().enumerate() {
    //             transposed[j][i] = tree.height;
    //         }
    //     }
    //     for (i_col, col) in transposed.iter().enumerate() {
    //         for (i, tree) in col.iter().enumerate() {
    //             let north = col[..i]
    //                 .iter()
    //                 .rev()
    //                 .take_while(|&other| other < tree)
    //                 .count();
    //             self.rows[i][i_col].scenic_score *= north;
    //             if north < i {
    //                 self.rows[i][i_col].visibility -= 1;
    //             }
    //             let south = col[i + 1..]
    //                 .iter()
    //                 .take_while(|&other| other < tree)
    //                 .count();
    //             self.rows[i][i_col].scenic_score *= north;
    //             if south < col.len() - i - 1 {
    //                 self.rows[i][i_col].visibility -= 1;
    //             }
    //         }
    //     }
    // }

    // pub fn get_trees_visible_from_the_outside(&self) -> Vec<(usize, usize)> {
    //     let mut visible_trees = vec![];
    //     for (i_row, row) in self.rows.iter().enumerate() {
    //         for (i_col, tree) in row.iter().enumerate() {
    //             if tree.visibility > 0 {
    //                 visible_trees.push((i_row, i_col));
    //             }
    //         }
    //     }
    //     return visible_trees;
    // }

    // pub fn get_highest_scenic_score(&self) -> usize {
    //     let mut highest = 0;
    //     for row in self.rows.iter() {
    //         for tree in row.iter() {
    //             highest = max(highest, tree.scenic_score);
    //         }
    //     }
    //     return highest;
    // }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_empty_forest() {
        let forest = Forest::from_input("");
        assert_eq!(0, forest.cols);
        assert_eq!(0, forest.rows);
        assert_eq!(0, forest.trees.len());
        // assert_eq!(0, forest.get_trees_visible_from_the_outside().len());
        // assert_eq!(0, forest.get_highest_scenic_score());
    }

    #[test]
    fn test_single_tree() {
        let forest = Forest::from_input("5");
        assert_eq!(1, forest.cols);
        assert_eq!(1, forest.rows);
        assert_eq!(1, forest.trees.len());
        assert_eq!(5, forest.trees[0][0].height);
        // assert_eq!(1, forest.get_trees_visible_from_the_outside().len());
        // assert_eq!(0, forest.get_highest_scenic_score());
    }

    #[test]
    fn test_small_forest() {
        let forest = Forest::from_input("12\n34\n");
        assert_eq!(2, forest.cols);
        assert_eq!(2, forest.rows);
        assert_eq!(2, forest.trees.len());
        assert_eq!(2, forest.trees[0].len());
        assert_eq!(1, forest.trees[0][0].height);
        assert_eq!(2, forest.trees[0][1].height);
        assert_eq!(3, forest.trees[1][0].height);
        assert_eq!(4, forest.trees[1][1].height);
        // assert_eq!(4, forest.get_trees_visible_from_the_outside().len());
        // assert_eq!(0, forest.get_highest_scenic_score());
    }

    #[test]
    fn test_a_proper_forest() {
        let forest = Forest::from_input("30373\n25512\n65332\n33549\n35390");
        // assert_eq!(21, forest.get_trees_visible_from_the_outside().len());
        // assert_eq!(8, forest.get_highest_scenic_score());
    }
}
