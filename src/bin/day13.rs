use adventofcode2018::*;

fn first(input: &[&str]) -> (usize, usize) {
    let board = get_board(input);
    let mut carts = get_carts(input);

    loop {
        {
            carts.sort_by_key(|c| (c.x << 8) + c.y);
        }

        for i in 0..carts.len() {
            let new_c = move_cart(&board, &carts[i]);

            if carts.iter().any(|c| new_c.x == c.x && new_c.y == c.y) {
                return (new_c.x, new_c.y);
            }

            carts[i] = new_c;
        }
    }
}

fn second(input: &[&str]) -> (usize, usize) {
    let board = get_board(input);
    let mut carts = get_carts(input);

    loop {
        {
            carts.sort_by_key(|c| (c.x << 8) + c.y);
        }

        fn coll(curr: &Cart, carts: &mut Vec<Cart>) -> bool {
            let coll = carts.iter().any(|c| curr.x == c.x && curr.y == c.y);

            if coll {
                carts.retain(|c| curr.x != c.x || curr.y != c.y);
            }

            coll
        }

        let mut new_carts = Vec::new();
        while !carts.is_empty() {
            let new_c = move_cart(&board, &carts.remove(0));

            if !coll(&new_c, &mut carts) && !coll(&new_c, &mut new_carts) {
                new_carts.push(new_c);
            }
        }

        if new_carts.len() == 1 {
            let c = &new_carts[0];
            return (c.x, c.y);
        }

        carts = new_carts;
    }
}

#[allow(clippy::ptr_arg)]
fn move_cart(board: &Vec<Vec<Track>>, cart: &Cart) -> Cart {
    let mut cart = cart.clone();

    match cart.dir {
        Direction::Up => cart.y -= 1,
        Direction::Down => cart.y += 1,
        Direction::Left => cart.x -= 1,
        Direction::Right => cart.x += 1,
    }

    cart.dir = match board[cart.y][cart.x] {
        Track::LeanRight => match cart.dir {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        },
        Track::LeanLeft => match cart.dir {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        },
        Track::Crossing => match cart.next_turn {
            Turn::Left => {
                cart.next_turn = Turn::Straight;
                match cart.dir {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                }
            }
            Turn::Right => {
                cart.next_turn = Turn::Left;
                match cart.dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                }
            }
            Turn::Straight => {
                cart.next_turn = Turn::Right;
                cart.dir
            }
        },
        _ => cart.dir,
    };

    cart
}

fn get_board(input: &[&str]) -> Vec<Vec<Track>> {
    input
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '|' => Track::Vertical,
                    'v' => Track::Vertical,
                    '^' => Track::Vertical,
                    '-' => Track::Horizontal,
                    '<' => Track::Horizontal,
                    '>' => Track::Horizontal,
                    '/' => Track::LeanRight,
                    '\\' => Track::LeanLeft,
                    '+' => Track::Crossing,
                    _ => Track::Empty,
                })
                .collect()
        })
        .collect()
}

enum Track {
    Vertical,
    Horizontal,
    LeanRight,
    LeanLeft,
    Crossing,
    Empty,
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Clone)]
struct Cart {
    x: usize,
    y: usize,
    dir: Direction,
    next_turn: Turn,
}

fn get_carts(input: &[&str]) -> Vec<Cart> {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, &s)| {
            s.chars().enumerate().map(move |(x, c)| {
                let d = match c {
                    '^' => Some(Direction::Up),
                    'v' => Some(Direction::Down),
                    '<' => Some(Direction::Left),
                    '>' => Some(Direction::Right),
                    _ => None,
                };
                d.map(|dir| Cart {
                    x,
                    y,
                    dir,
                    next_turn: Turn::Left,
                })
            })
        })
        .filter_map(|c| c)
        .collect()
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("13");
    let input: Vec<&str> = input.trim_end().split('\n').collect();

    println!("{:?}", first(&input));

    println!("{:?}", second(&input));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            first(&vec![
                "/->-\\        ",
                "|   |  /----\\",
                "| /-+--+-\\  |",
                "| | |  | v  |",
                "\\-+-/  \\-+--/",
                "  \\------/   "
            ]),
            (7, 3)
        );
    }
}
