use adventofcode2018::*;

use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

fn first(input: &[&str]) -> usize {
    let mut board = Board::parse(input);
    board.drop_water();
    board
        .board
        .iter()
        .filter(|(_, t)| matches!(t, Tile::Water | Tile::Path))
        .count()
}

fn second(input: &[&str]) -> usize {
    let mut board = Board::parse(input);
    board.drop_water();
    board
        .board
        .iter()
        .filter(|(_, t)| matches!(t, Tile::Water))
        .count()
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }

    fn spring() -> Coord {
        Coord::new(500, 0)
    }
}

struct Board {
    board: HashMap<Coord, Tile>,
    floor: usize,
    ceiling: usize,
}

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"(x|y)=(\d+), .=(\d+)..(\d+)").unwrap();
}

impl Board {
    fn parse(input: &[&str]) -> Board {
        let mut board = HashMap::new();
        let mut floor = 0;
        let mut ceiling = 100;
        for &line in input {
            if let Some(cap) = LINE_RE.captures(line) {
                let left = parse_capture(&cap, 1, "left").unwrap();
                let left_idx = parse_capture(&cap, 2, "left_idx").unwrap();
                let right_start = parse_capture(&cap, 3, "right_start").unwrap();
                let right_end = parse_capture(&cap, 4, "right_end").unwrap();

                match left {
                    'x' => {
                        let x = left_idx;
                        for y in right_start..=right_end {
                            board.insert(Coord::new(x, y), Tile::Clay);
                            floor = floor.max(y);
                            ceiling = ceiling.min(y);
                        }
                    }
                    'y' => {
                        let y = left_idx;
                        floor = floor.max(y);
                        ceiling = ceiling.min(y);
                        for x in right_start..=right_end {
                            board.insert(Coord::new(x, y), Tile::Clay);
                        }
                    }
                    _ => panic!("invalid left pattern"),
                }
            }
        }
        floor += 1;
        Board {
            board,
            floor,
            ceiling,
        }
    }

    fn get_tile(&self, pos: &Coord) -> Tile {
        match self.board.get(pos) {
            Some(t) => t.clone(),
            _ => Tile::Sand,
        }
    }

    fn drop_water(&mut self) {
        self.fill_path(&Coord::spring())
    }

    fn fill_path(&mut self, start_pos: &Coord) {
        let mut cursor = Coord::new(start_pos.x, start_pos.y + 1);

        while cursor.y < self.floor && self.get_tile(&cursor) == Tile::Sand {
            if cursor.y >= self.ceiling {
                self.board.insert(cursor, Tile::Path);
            }
            cursor = Coord::new(cursor.x, cursor.y + 1);
        }

        if cursor.y == self.floor {
            return;
        }

        if matches!(self.get_tile(&cursor), Tile::Path) {
            return;
        }

        loop {
            cursor = Coord::new(cursor.x, cursor.y - 1);

            let (left_limit, left_falling) = self.get_left_limit(&cursor);
            let (right_limit, right_falling) = self.get_right_limit(&cursor);

            if left_falling || right_falling {
                for x in left_limit..=right_limit {
                    self.board.insert(Coord::new(x, cursor.y), Tile::Path);
                }

                if left_falling {
                    self.fill_path(&Coord::new(left_limit, cursor.y));
                }
                if right_falling {
                    self.fill_path(&Coord::new(right_limit, cursor.y));
                }

                break;
            }

            for x in left_limit..=right_limit {
                self.board.insert(Coord::new(x, cursor.y), Tile::Water);
            }
        }
    }

    fn get_left_limit(&self, start_pos: &Coord) -> (usize, bool) {
        let mut check = start_pos.x;
        loop {
            if self.get_tile(&Coord::new(check - 1, start_pos.y)) == Tile::Clay {
                return (check, false);
            } else if matches!(
                self.get_tile(&Coord::new(check, start_pos.y + 1)),
                Tile::Sand | Tile::Path
            ) {
                return (check, true);
            } else {
                check -= 1;
            }
        }
    }

    fn get_right_limit(&self, start_pos: &Coord) -> (usize, bool) {
        let mut check = start_pos.x;
        loop {
            if self.get_tile(&Coord::new(check + 1, start_pos.y)) == Tile::Clay {
                return (check, false);
            } else if matches!(
                self.get_tile(&Coord::new(check, start_pos.y + 1)),
                Tile::Sand | Tile::Path
            ) {
                return (check, true);
            } else {
                check += 1;
            }
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Tile {
    Sand,
    Clay,
    Water,
    Path,
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("17");
    let input: Vec<&str> = input.trim().split('\n').collect();

    println!("{}", first(&input));
    println!("{}", second(&input));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Tile::{Clay, Path, Sand, Water};

    lazy_static! {
        static ref VEINS: Vec<&'static str> = vec![
            "x=495, y=2..7",
            "y=7, x=495..501",
            "x=501, y=3..7",
            "x=498, y=2..4",
            "x=506, y=1..2",
            "x=498, y=10..13",
            "x=504, y=10..13",
            "y=13, x=498..504",
        ];
    }

    fn check_board(check: &[&str], board: &Board) {
        let top_margin = 1;
        let left_margin = 494;

        for (row, &line) in check.iter().enumerate() {
            let row = row + top_margin;
            for (col, char) in line.chars().enumerate() {
                let col = col + left_margin;
                let tile = match char {
                    '#' => Clay,
                    '~' => Water,
                    '|' => Path,
                    _ => Sand,
                };
                let coord = Coord::new(col, row);
                assert_eq!(tile, board.get_tile(&coord), "checking x={} y={}", col, row);
            }
        }
    }

    #[test]
    fn test() {
        let check = vec![
            "......|.....#.",
            ".#..#||||...#.",
            ".#..#~~#|.....",
            ".#..#~~#|.....",
            ".#~~~~~#|.....",
            ".#~~~~~#|.....",
            ".#######|.....",
            "........|.....",
            "...|||||||||..",
            "...|#~~~~~#|..",
            "...|#~~~~~#|..",
            "...|#~~~~~#|..",
            "...|#######|..",
        ];

        let mut board = Board::parse(&VEINS);
        board.drop_water();
        check_board(&check, &board);
    }

    #[test]
    fn test1() {
        assert_eq!(57, first(&VEINS));
    }

    #[test]
    fn test2() {
        assert_eq!(29, second(&VEINS));
    }
}
