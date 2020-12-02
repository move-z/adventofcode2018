use adventofcode2018::*;

use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use std::cmp::Ordering;

fn first(input: &[&str]) -> usize {
    let mut board = parse(input);
    let turn = run_game(&mut board);
    (get_hp(&board, &Kind::Goblin) + get_hp(&board, &Kind::Elf)) * turn
}

fn second(input: &[&str]) -> usize {
    let mut power = 3usize;
    let mut max_fail = power;
    let mut min_success = usize::max_value();
    let mut outcome = 0;

    let count = |board: &Board| board.iter()
        .filter(|(_, t)| matches!(t, Tile::Npc(Kind::Elf, _, _)))
        .count();
    loop {
        let mut board = parse_powerelf(input, power);
        let n_elves = count(&board);

        let turn = run_game(&mut board);
        let success = count(&board) == n_elves;

        if success {
            min_success = power;
            outcome = (get_hp(&board, &Kind::Goblin) + get_hp(&board, &Kind::Elf)) * turn;
        } else {
            max_fail = power;
        }

        if min_success == max_fail + 1 {
            break;
        }

        power = max_fail + (min_success - max_fail) / 2;
    }

    outcome
}

fn run_game(board: &mut Board) -> usize {
    let mut turn = 0;
    while run_turn(board) {
        turn += 1;
    }
    turn
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Debug)]
struct Coord {
    y: isize,
    x: isize,
}

impl Coord {
    fn at(x: isize, y: isize) -> Coord {
        Coord { x, y }
    }
}

type Path = Vec<Coord>;

#[derive(Copy, Clone)]
enum Tile {
    Wall,
    Npc(Kind, usize, usize),
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Kind {
    Elf,
    Goblin,
}

type Board = HashMap<Coord, Tile>;


fn parse(input: &[&str]) -> Board {
    parse_powerelf(input, 3)
}

fn parse_powerelf(input: &[&str], elf_power: usize) -> Board {
    input.iter()
        .enumerate()
        .flat_map(|(y, s)| {
            s.chars()
                .enumerate()
                .filter_map(move |(x, c)| {
                    let pos: Coord = Coord::at(x as isize, y as isize);
                    match c {
                        '#' => Some((pos, Tile::Wall)),
                        'E' => Some((pos, Tile::Npc(Kind::Elf, elf_power, 200))),
                        'G' => Some((pos, Tile::Npc(Kind::Goblin, 3, 200))),
                        _ => None,
                    }
                })
        })
        .collect()
}

fn target_path(board: &Board, player_coord: &Coord) -> Option<Path> {
    let player_kind = get_kind(board, player_coord)?;

    if adjacent(&board, player_coord, &player_kind) {
        return Some(Vec::new());
    }

    let mut paths: Vec<Path> = vec![vec![*player_coord]];
    let mut visited = HashSet::new();
    visited.insert(*player_coord);

    loop {
        let new_paths: Vec<Path> = paths.iter()
            .flat_map(|path| {
                let reachables = get_reachables(board, path.last().unwrap(), &mut visited);
                reachables.iter().map(|r| {
                    let mut path = path.clone();
                    path.push(*r);
                    path
                }).collect::<Vec<Path>>()
            })
            .collect();

        if new_paths.is_empty() {
            return None
        }

        paths = new_paths;

        let targets: Vec<(&Coord, &Path)> = paths.iter()
            .filter_map(|p| {
                let last = p.last().unwrap();
                if adjacent(&board, last, &player_kind) {
                    Some((last, p))
                } else {
                    None
                }
            })
            .sorted_by_key(|(t, _)| *t)
            .collect();

        if let Some((mut tgt, mut path)) = targets.first() {
            for (t, p) in targets.iter().skip(1) {
                if *t > tgt {
                    break;
                }
                if *p < path {
                    tgt = *t;
                    path = &p;
                }
            }
            return Some(path.clone().into_iter().skip(1).collect());
        }
    }
}

fn get_kind(board: &Board, player_coord: &Coord) -> Option<Kind> {
    board.get(player_coord)
        .and_then(|t| match t {
            Tile::Npc(k, _, _) => Some(*k),
            _ => None
        })
}

fn get_reachables(board: &Board, from: &Coord, visited: &mut HashSet<Coord>) -> Vec<Coord> {
    let mut res = Vec::new();

    let mut check_set = |c: Coord| {
        if board.get(&c).is_none() && !visited.contains(&c) {
            res.push(c);
            visited.insert(c);
        }
    };

    check_set(Coord::at(from.x, from.y - 1));
    check_set(Coord::at(from.x - 1, from.y));
    check_set(Coord::at(from.x + 1, from.y));
    check_set(Coord::at(from.x, from.y + 1));

    res
}

fn adjacent(board: &Board, from: &Coord, player_kind: &Kind) -> bool {
    let is_tgt = |x, y| {
        is_target(board, &Coord::at(x, y), player_kind)
    };

    is_tgt(from.x, from.y - 1)
        || is_tgt(from.x - 1, from.y)
        || is_tgt(from.x + 1, from.y)
        || is_tgt(from.x, from.y + 1)
}

fn is_target(board: &Board, tgt: &Coord, player_kind: &Kind) -> bool {
    board.get(tgt)
        .filter(|&t| {
            matches!(t, Tile::Npc(k, _, _) if *k != *player_kind)
        })
        .is_some()
}

fn mov(board: &mut Board, player: &Coord) -> Option<Coord> {
    if !board.contains_key(player) {
        return None;
    }

    let mut player = player;

    if let Some(path) = target_path(board, player) {
        if let Some(next_pos) = path.first() {
            let p = board.remove(player).unwrap();
            board.insert(*next_pos, p);
            player = next_pos;
        }

        let power = if let Some(Tile::Npc(_, power, _)) = board.get(player) {
            *power
        } else {
            0
        };

        if let Some(target_coord) = maybe_attack(board, player) {
            if let Some(Tile::Npc(_, _, hp)) = board.get_mut(&target_coord) {
                if *hp > power {
                    *hp -= power;
                } else {
                    board.remove(&target_coord);
                    return Some(target_coord);
                }
            }
        }
    }
    None
}

fn maybe_attack(board: &Board, player_coord: &Coord) -> Option<Coord> {
    let player = board.get(player_coord);
    if let Some(Tile::Npc(player_kind, _, _)) = player {
        get_target(board, player_coord, player_kind)
    } else {
        None
    }
}

fn get_target(board: &Board, player_coord: &Coord, player_kind: &Kind) -> Option<Coord> {
    vec![(0, -1), (-1, 0), (1, 0), (0, 1)]
        .iter()
        .filter_map(|(dx, dy)| {
            let tgt_coord = Coord::at(player_coord.x + dx, player_coord.y + dy);
            if is_target(board, &tgt_coord, &player_kind) {
                Some(tgt_coord)
            } else {
                None
            }
        })
        .sorted_by(|t1, t2| {
            if let (Some(Tile::Npc(_, _, hp1)), Some(Tile::Npc(_, _, hp2))) = (board.get(t1), board.get(t2)) {
                hp1.cmp(&hp2)
            } else {
                Ordering::Equal
            }
        })
        .next()
}

fn run_turn(board: &mut Board) -> bool {
    let players = get_players(&board);
    let mut killed = Vec::new();

    for player in players.iter() {
        if killed.contains(player) {
            continue;
        }

        if board.iter().find(|(_, t)| matches!(t, Tile::Npc(Kind::Elf, _, _))).is_none() ||
            board.iter().find(|(_, t)| matches!(t, Tile::Npc(Kind::Goblin, _, _))).is_none() {
            return false
        }

        if let Some(pos) = mov(board, player) {
            killed.push(pos);
        }
    }

    true
}

fn get_players(board: &Board) -> Vec<Coord> {
    board.iter()
        .filter(|(_, t)| matches!(t, Tile::Npc(_, _, _)))
        .map(|(c, _)| *c)
        .sorted()
        .collect()
}

fn get_hp(board: &Board, kind: &Kind) -> usize {
    board.iter()
        .map(|(_, t)| match t {
            Tile::Npc(k, _, hp) if k == kind => hp,
            _ => &0usize,
        })
        .sum()
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("15");
    let input: Vec<&str> = input.trim().split('\n').collect();

    println!("{}", first(&input));

    println!("{}", second(&input));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_path() {
        let board = vec!["#######", "#E..G.#", "#...#.#", "#.G.#G#", "#######"];
        let board = parse(&board);

        assert_eq!(Some(vec![Coord::at(2, 1), Coord::at(3, 1)]),
                   target_path(&board, &Coord::at(1, 1)));
    }

    #[test]
    fn test_path2() {
        let board = vec!["#######", "#.E...#", "#.....#", "#...G.#", "#######"];
        let board = parse(&board);

        assert_eq!(
            Some(vec![Coord::at(3, 1), Coord::at(4, 1), Coord::at(4, 2)]),
            target_path(&board, &Coord::at(2, 1))
        );
    }

    #[test]
    fn test_moves() {
        let board = vec![
            "#########",
            "#G..G..G#",
            "#.......#",
            "#.......#",
            "#G..E..G#",
            "#.......#",
            "#.......#",
            "#G..G..G#",
            "#########",
        ];
        let mut board = parse(&board);

        assert_eq!(true, run_turn(&mut board));
        assert_eq!(
            vec![
                Coord::at(2, 1),
                Coord::at(6, 1),
                Coord::at(4, 2),
                Coord::at(4, 3),
                Coord::at(7, 3),
                Coord::at(2, 4),
                Coord::at(1, 6),
                Coord::at(4, 6),
                Coord::at(7, 6)
            ],
            get_players(&board)
        );

        assert_eq!(true, run_turn(&mut board));
        assert_eq!(
            vec![
                Coord::at(3, 1),
                Coord::at(5, 1),
                Coord::at(4, 2),
                Coord::at(2, 3),
                Coord::at(4, 3),
                Coord::at(6, 3),
                Coord::at(1, 5),
                Coord::at(4, 5),
                Coord::at(7, 5)
            ],
            get_players(&board)
        );

        run_turn(&mut board);
        assert_eq!(
            vec![
                Coord::at(3, 2),
                Coord::at(4, 2),
                Coord::at(5, 2),
                Coord::at(3, 3),
                Coord::at(4, 3),
                Coord::at(5, 3),
                Coord::at(1, 4),
                Coord::at(4, 4),
                Coord::at(7, 5)
            ],
            get_players(&board)
        );
    }

    #[test]
    fn test_turns() {
        let board = vec![
            "#######",
            "#.G...#",
            "#...EG#",
            "#.#.#G#",
            "#..G#E#",
            "#.....#",
            "#######",
        ];
        let mut board = parse(&board);

        println!("TURN 1");
        assert_eq!(true, run_turn(&mut board));
        assert_eq!(
            vec![
                Coord::at(3, 1),
                Coord::at(4, 2),
                Coord::at(5, 2),
                Coord::at(3, 3),
                Coord::at(5, 3),
                Coord::at(5, 4),
            ],
            get_players(&board)
        );

        println!("TURN 2");
        assert_eq!(true, run_turn(&mut board));
        assert_eq!(
            vec![
                Coord::at(4, 1),
                Coord::at(3, 2),
                Coord::at(4, 2),
                Coord::at(5, 2),
                Coord::at(5, 3),
                Coord::at(5, 4),
            ],
            get_players(&board)
        );

        for i in 3..=23 {
            println!("TURN {}", i);
            assert_eq!(true, run_turn(&mut board));
        }
        assert_eq!(
            vec![
                Coord::at(4, 1),
                Coord::at(3, 2),
                Coord::at(5, 2),
                Coord::at(5, 3),
                Coord::at(5, 4),
            ],
            get_players(&board)
        );

        println!("TURN 24");
        assert_eq!(true, run_turn(&mut board));
        assert_eq!(
            vec![
                    Coord::at(3, 1),
                    Coord::at(4, 2),
                    Coord::at(3, 3),
                    Coord::at(5, 3),
                    Coord::at(5, 4),
            ],
            get_players(&board)
        );

        println!("TURN 25");
        assert_eq!(true, run_turn(&mut board));
        assert_eq!(
            vec![
                Coord::at(2, 1),
                Coord::at(3, 2),
                Coord::at(5, 3),
                Coord::at(3, 4),
                Coord::at(5, 4),
            ],
            get_players(&board)
        );

        println!("TURN 26");
        assert_eq!(true, run_turn(&mut board));
        assert_eq!(
            vec![
                Coord::at(1, 1),
                Coord::at(2, 2),
                Coord::at(5, 3),
                Coord::at(5, 4),
                Coord::at(3, 5),
            ],
            get_players(&board)
        );

        println!("TURN 27");
        assert_eq!(true, run_turn(&mut board));
        assert_eq!(
            vec![
                Coord::at(1, 1),
                Coord::at(2, 2),
                Coord::at(5, 3),
                Coord::at(5, 4),
                Coord::at(4, 5),
            ],
            get_players(&board)
        );

        println!("TURN 28");
        assert_eq!(true, run_turn(&mut board));
        assert_eq!(
            vec![
                Coord::at(1, 1),
                Coord::at(2, 2),
                Coord::at(5, 3),
                Coord::at(5, 4),
                Coord::at(5, 5),
            ],
            get_players(&board)
        );

        for i in 29..=47 {
            println!("TURN {}", i);
            assert_eq!(true, run_turn(&mut board));
        }
        assert_eq!(vec![
            Coord::at(1, 1),
            Coord::at(2, 2),
            Coord::at(5, 3),
            Coord::at(5, 5),
        ], get_players(&board));

        assert_eq!(false, run_turn(&mut board));
        assert_eq!(vec![
            Coord::at(1, 1),
            Coord::at(2, 2),
            Coord::at(5, 3),
            Coord::at(5, 5),
        ], get_players(&board));

        assert_eq!(590, get_hp(&board, &Kind::Goblin));
    }

    #[test]
    fn test11() {
        let board = vec![
            "#######",
            "#G..#E#",
            "#E#E.E#",
            "#G.##.#",
            "#...#E#",
            "#...E.#",
            "#######",
        ];
        assert_eq!(36334, first(&board))
    }

    #[test]
    fn test12() {
        let board = vec![
            "#######",
            "#E..EG#",
            "#.#G.E#",
            "#E.##E#",
            "#G..#.#",
            "#..E#.#",
            "#######",
        ];
        assert_eq!(39514, first(&board))
    }

    #[test]
    fn test13() {
        let board = vec![
            "#######",
            "#E.G#.#",
            "#.#G..#",
            "#G.#.G#",
            "#G..#.#",
            "#...E.#",
            "#######",
        ];
        assert_eq!(27755, first(&board))
    }

    #[test]
    fn test14() {
        let board = vec![
            "#######",
            "#.E...#",
            "#.#..G#",
            "#.###.#",
            "#E#G#G#",
            "#...#G#",
            "#######",
        ];
        assert_eq!(28944, first(&board))
    }

    #[test]
    fn test15() {
        let board = vec![
            "#########",
            "#G......#",
            "#.E.#...#",
            "#..##..G#",
            "#...##..#",
            "#...#...#",
            "#.G...G.#",
            "#.....G.#",
            "#########",
        ];
        assert_eq!(18740, first(&board))
    }

    #[test]
    fn test21() {
        let board = vec![
            "#######",
            "#.G...#",
            "#...EG#",
            "#.#.#G#",
            "#..G#E#",
            "#.....#",
            "#######",
        ];
        assert_eq!(4988, second(&board))
    }

    #[test]
    fn test22() {
        let board = vec![
            "#######",
            "#E..EG#",
            "#.#G.E#",
            "#E.##E#",
            "#G..#.#",
            "#..E#.#",
            "#######",
        ];
        assert_eq!(31284, second(&board))
    }

    #[test]
    fn test23() {
        let board = vec![
            "#######",
            "#E.G#.#",
            "#.#G..#",
            "#G.#.G#",
            "#G..#.#",
            "#...E.#",
            "#######",
        ];
        assert_eq!(3478, second(&board))
    }

    #[test]
    fn test24() {
        let board = vec![
            "#######",
            "#.E...#",
            "#.#..G#",
            "#.###.#",
            "#E#G#G#",
            "#...#G#",
            "#######",
        ];
        assert_eq!(6474, second(&board))
    }

    #[test]
    fn test25() {
        let board = vec![
            "#########",
            "#G......#",
            "#.E.#...#",
            "#..##..G#",
            "#...##..#",
            "#...#...#",
            "#.G...G.#",
            "#.....G.#",
            "#########",
        ];
        assert_eq!(1140, second(&board))
    }
}
