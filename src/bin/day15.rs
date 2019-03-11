use adventofcode2018::*;
use std::collections::HashSet;

fn first(input: &[&str]) -> usize {
    let mut board = parse(input);

    let mut turn = 0;
    println!("TURN {}", turn);
    print_board(&board);
    while run_turn(&mut board) {
        println!("TURN {}", turn);
        print_board(&board);
        turn += 1;
    }
    println!("TURN {}", turn + 1);
    print_board(&board);

    (get_hp(&board, &Kind::Goblin) + get_hp(&board, &Kind::Elf)) * turn
}

type Board = Vec<Vec<Tile>>;
type Coord = (usize, usize);

#[allow(clippy::ptr_arg)]
fn get_hp(board: &Vec<Vec<Tile>>, kind: &Kind) -> usize {
    get_players(board)
        .iter()
        .filter_map(|c| match &board[c.1][c.0] {
            Tile::Npc(k, hp) if k == kind => Some(hp),
            _ => None,
        })
        .sum::<usize>()
}

fn run_turn(board: &mut Board) -> bool {
    let mut players = get_players(&board);

    let mut idx = 0;
    while idx < players.len() {
        let player = players[idx];

        let tgt_kind = match get_kind(&board, player).unwrap() {
            Kind::Goblin => Kind::Elf,
            Kind::Elf => Kind::Goblin,
        };
        if get_hp(&board, &tgt_kind) == 0 {
            return false;
        }

        let mov = |board: &mut Board, next_pos: Coord| {
            let p = board[player.1][player.0].clone();
            board[player.1][player.0] = Tile::Free;
            board[next_pos.1][next_pos.0] = p;
        };

        let mut attak = |board: &mut Board, k: Kind, hp: usize, pos: Coord| {
            if hp > 3 {
                board[pos.1][pos.0] = Tile::Npc(k, hp - 3);
            } else {
                board[pos.1][pos.0] = Tile::Free;
                let dead = players
                    .iter()
                    .enumerate()
                    .find(|(_, &p)| p == pos)
                    .unwrap()
                    .0;
                players.remove(dead);
                if dead < idx {
                    idx -= 1;
                }
            }
        };

        match move_player(board, player) {
            Action::None => {}
            Action::Move(next_pos) => mov(board, next_pos),
            Action::Attack(k, hp, pos) => attak(board, k, hp, pos),
            Action::MoveAttack(next_pos, k, hp, pos) => {
                mov(board, next_pos);
                attak(board, k, hp, pos);
            }
        }

        idx += 1;
    }

    true
}

#[derive(Clone, PartialEq)]
enum Kind {
    Elf,
    Goblin,
}

#[derive(Clone)]
enum Tile {
    Wall,
    Free,
    Npc(Kind, usize),
}

enum Action {
    Move(Coord),
    Attack(Kind, usize, Coord),
    MoveAttack(Coord, Kind, usize, Coord),
    None,
}

#[allow(clippy::ptr_arg)]
fn move_player(board: &Board, player: Coord) -> Action {
    match target_path(board, player) {
        Some(path) => {
            let next_pos = path[0];
            let next = &board[next_pos.1][next_pos.0];

            match next {
                Tile::Free => {
                    let tgt_pos = path[1];
                    let tgt = &board[tgt_pos.1][tgt_pos.0];
                    match tgt {
                        Tile::Npc(k, h) => Action::MoveAttack(next_pos, k.clone(), *h, tgt_pos),
                        Tile::Free => Action::Move(next_pos),
                        Tile::Wall => panic!(),
                    }
                }
                Tile::Npc(k, h) => Action::Attack(k.clone(), *h, next_pos),
                Tile::Wall => panic!(),
            }
        }
        None => Action::None,
    }
}

#[allow(clippy::ptr_arg)]
fn target_path(board: &Board, player: Coord) -> Option<Vec<Coord>> {
    let player_kind = get_kind(board, player)?;

    let t = get_target(board, player, &player_kind);
    if t.is_some() {
        return Some(vec![t.unwrap()]);
    }

    let mut paths = vec![vec![player]];
    let mut visited = HashSet::new();
    visited.insert(player);

    loop {
        let mut reachables = Vec::new();
        for path in &paths {
            for tgt in get_reachables(board, *path.last().unwrap()) {
                if visited.contains(&tgt) {
                    continue;
                }

                let mut new_path = Vec::new();
                for step in path {
                    new_path.push(*step);
                }
                new_path.push(tgt);
                reachables.push(new_path);
            }
        }
        reachables.iter().for_each(|r| {
            visited.insert(*r.last().unwrap());
        });

        if reachables.is_empty() {
            return None;
        }

        let mut adjacent = reachables
            .iter()
            .filter_map(|path| {
                get_target(board, *path.last().unwrap(), &player_kind).map(|t| (path.to_vec(), t))
            })
            .collect::<Vec<(Vec<Coord>, Coord)>>();

        fn coord_sort_key(coord: &Coord) -> usize {
            coord.0 + coord.1 * 1000
        }

        if !adjacent.is_empty() {
            let min_tgt = adjacent
                .iter()
                .min_by_key(|(_, tgt)| coord_sort_key(tgt))
                .unwrap()
                .1;

            let next_step = adjacent
                .iter_mut()
                .filter_map(|(path, tgt)| {
                    if *tgt == min_tgt {
                        path.push(*tgt);
                        Some(path)
                    } else {
                        None
                    }
                })
                .min_by_key(|path| coord_sort_key(&path[path.len() - 2]))
                .unwrap();

            return Some(next_step[1..].to_vec());
        }

        paths = reachables;
    }
}

#[allow(clippy::ptr_arg)]
fn get_kind(board: &Board, player: Coord) -> Option<Kind> {
    if let Tile::Npc(k, _) = &board[player.1][player.0] {
        Some(k.clone())
    } else {
        None
    }
}

#[allow(clippy::ptr_arg)]
fn get_reachables(board: &Board, from: Coord) -> Vec<Coord> {
    let mut res = Vec::new();

    if from.1 > 0 {
        if let Tile::Free = board[from.1 - 1][from.0] {
            res.push((from.0, from.1 - 1));
        }
    }
    if from.0 > 0 {
        if let Tile::Free = board[from.1][from.0 - 1] {
            res.push((from.0 - 1, from.1));
        }
    }
    if let Tile::Free = board[from.1][from.0 + 1] {
        res.push((from.0 + 1, from.1));
    }
    if let Tile::Free = board[from.1 + 1][from.0] {
        res.push((from.0, from.1 + 1));
    }
    res
}

#[allow(clippy::ptr_arg)]
fn get_target(board: &Board, from: Coord, player_k: &Kind) -> Option<Coord> {
    let mut candidate = None;

    let mut check_set = |x: usize, y: usize| {
        match &board[y][x] {
            Tile::Npc(k, hp) if *k != *player_k => {
                match candidate {
                    Some((_, _, old_hp)) if old_hp <= hp => {}
                    _ => candidate = Some((x, y, hp)),
                };
            }
            _ => {}
        };
    };

    if from.1 > 0 {
        check_set(from.0, from.1 - 1);
    }
    if from.0 > 0 {
        check_set(from.0 - 1, from.1);
    }
    check_set(from.0 + 1, from.1);
    check_set(from.0, from.1 + 1);

    candidate.map(|(x, y, _)| (x, y))
}

#[allow(clippy::ptr_arg)]
fn get_players(board: &Board) -> Vec<Coord> {
    board
        .iter()
        .enumerate()
        .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, t)| (t, (x, y))))
        .flat_map(|(t, c)| match t {
            Tile::Npc(_, _) => Some(c),
            _ => None,
        })
        .collect()
}

fn parse(input: &[&str]) -> Board {
    input
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '#' => Tile::Wall,
                    'E' => Tile::Npc(Kind::Elf, 200),
                    'G' => Tile::Npc(Kind::Goblin, 200),
                    _ => Tile::Free,
                })
                .collect()
        })
        .collect()
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("15");
    let input: Vec<&str> = input.trim().split('\n').collect();

    println!("{}", first(&input));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_path() {
        let board = vec!["#######", "#E..G.#", "#...#.#", "#.G.#G#", "#######"];
        let board = parse(&board);

        assert_eq!(Some(vec![(2, 1), (3, 1), (4, 1)]), target_path(&board, (1, 1)));
    }

    #[test]
    fn test_path2() {
        let board = vec!["#######", "#.E...#", "#.....#", "#...G.#", "#######"];
        let board = parse(&board);

        assert_eq!(
            Some(vec![(3, 1), (4, 1), (4, 2), (4, 3)]),
            target_path(&board, (2, 1))
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
                (2, 1),
                (6, 1),
                (4, 2),
                (4, 3),
                (7, 3),
                (2, 4),
                (1, 6),
                (4, 6),
                (7, 6)
            ],
            get_players(&board)
        );

        assert_eq!(true, run_turn(&mut board));
        assert_eq!(
            vec![
                (3, 1),
                (5, 1),
                (4, 2),
                (2, 3),
                (4, 3),
                (6, 3),
                (1, 5),
                (4, 5),
                (7, 5)
            ],
            get_players(&board)
        );

        run_turn(&mut board);
        assert_eq!(
            vec![
                (3, 2),
                (4, 2),
                (5, 2),
                (3, 3),
                (4, 3),
                (5, 3),
                (1, 4),
                (4, 4),
                (7, 5)
            ],
            get_players(&board)
        );
    }

    #[test]
    fn test_turns() {
        let board = vec![
            "#######", "#.G...#", "#...EG#", "#.#.#G#", "#..G#E#", "#.....#", "#######",
        ];
        let mut board = parse(&board);

        println!("TURN 1");
        assert_eq!(true, run_turn(&mut board));
        assert_eq!(
            vec![(3, 1), (4, 2), (5, 2), (3, 3), (5, 3), (5, 4),],
            get_players(&board)
        );

        println!("TURN 2");
        assert_eq!(true, run_turn(&mut board));
        assert_eq!(
            vec![(4, 1), (3, 2), (4, 2), (5, 2), (5, 3), (5, 4),],
            get_players(&board)
        );

        for i in 3..=23 {
            println!("TURN {}", i);
            assert_eq!(true, run_turn(&mut board));
        }
        assert_eq!(
            vec![(4, 1), (3, 2), (5, 2), (5, 3), (5, 4),],
            get_players(&board)
        );

        println!("TURN 24");
        assert_eq!(true, run_turn(&mut board));
        assert_eq!(
            vec![(3, 1), (4, 2), (3, 3), (5, 3), (5, 4),],
            get_players(&board)
        );

        println!("TURN 25");
        assert_eq!(true, run_turn(&mut board));
        assert_eq!(
            vec![(2, 1), (3, 2), (5, 3), (3, 4), (5, 4),],
            get_players(&board)
        );

        println!("TURN 26");
        assert_eq!(true, run_turn(&mut board));
        assert_eq!(
            vec![(1, 1), (2, 2), (5, 3), (5, 4), (3, 5),],
            get_players(&board)
        );

        println!("TURN 27");
        assert_eq!(true, run_turn(&mut board));
        assert_eq!(
            vec![(1, 1), (2, 2), (5, 3), (5, 4), (4, 5),],
            get_players(&board)
        );

        println!("TURN 28");
        assert_eq!(true, run_turn(&mut board));
        assert_eq!(
            vec![(1, 1), (2, 2), (5, 3), (5, 4), (5, 5),],
            get_players(&board)
        );

        for i in 29..=47 {
            println!("TURN {}", i);
            assert_eq!(true, run_turn(&mut board));
        }
        assert_eq!(vec![(1, 1), (2, 2), (5, 3), (5, 5),], get_players(&board));

        assert_eq!(false, run_turn(&mut board));
        assert_eq!(vec![(1, 1), (2, 2), (5, 3), (5, 5),], get_players(&board));

        assert_eq!(590, get_hp(&board, &Kind::Goblin));
    }

    #[test]
    fn test1() {
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
    fn test2() {
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
    fn test3() {
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
    fn test4() {
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
    fn test5() {
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
}

fn print_board(board: &[Vec<Tile>]) {
    let mut npcs = Vec::new();

    for row in board {
        for tile in row {
            let c = match tile {
                Tile::Free => '.',
                Tile::Wall => '#',
                Tile::Npc(Kind::Elf, h) => {
                    npcs.push(('E', h));
                    'E'
                },
                Tile::Npc(Kind::Goblin, h) => {
                    npcs.push(('G', h));
                    'G'
                },
            };
            print!("{}", c);
        }
        println!();
    }

    for npc in npcs {
        println!("{}({})", npc.0, npc.1);
    }
}
