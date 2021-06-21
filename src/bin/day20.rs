use adventofcode2018::*;
use std::collections::HashSet;
use std::str::Chars;

fn first(input: &str) -> usize {
    let routes = Route::parse(input);
    longest_path(&routes)
}

fn longest_path(route: &Route) -> usize {
    route
        .0
        .iter()
        .map(|e| match e {
            RouteElement::Directions(s) => s.len(),
            RouteElement::Choice(c) => c.iter().map(|r| longest_path(r)).max().unwrap_or(0),
        })
        .sum()
}

fn second(input: &str) -> usize {
    let routes = Route::parse(input);
    let mut rooms = HashSet::new();
    n_rooms(&routes, 999, 0, 0, &mut rooms).0
}

fn n_rooms(
    route: &Route,
    skip_n: usize,
    start_x: i32,
    start_y: i32,
    seen_rooms: &mut HashSet<(i32, i32)>,
) -> (usize, usize) {
    let mut to_skip = skip_n;
    let mut rooms = 0;
    let mut x = start_x;
    let mut y = start_y;

    for r in &route.0 {
        match r {
            RouteElement::Directions(s) => {
                let mut block_rooms = 0;
                for dir in s {
                    match dir {
                        Direction::N => y += 1,
                        Direction::W => x -= 1,
                        Direction::S => y -= 1,
                        Direction::E => x += 1,
                    }
                    if seen_rooms.insert((x, y)) {
                        block_rooms += 1;
                    }
                }
                if to_skip > block_rooms {
                    to_skip -= block_rooms;
                } else {
                    rooms += block_rooms - to_skip;
                    to_skip = 0;
                }
            }
            RouteElement::Choice(c) => {
                let paths_rooms: Vec<(usize, usize)> = c
                    .iter()
                    .map(|r| n_rooms(r, to_skip, x, y, seen_rooms))
                    .collect();
                let skipped = paths_rooms.iter().map(|p| p.1).min();
                let paths_rooms = paths_rooms.iter().map(|p| p.0).sum::<usize>();
                if let Some(skipped) = skipped {
                    if to_skip > skipped {
                        to_skip -= skipped;
                    } else {
                        to_skip = 0;
                    }
                }
                rooms += paths_rooms;
            }
        }
    }

    (rooms, skip_n - to_skip)
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    N,
    W,
    S,
    E,
}

#[derive(Debug)]
enum RouteElement {
    Directions(Vec<Direction>),
    Choice(Vec<Route>),
}

#[derive(Debug)]
struct Route(Vec<RouteElement>);

impl Route {
    fn parse(input: &str) -> Route {
        let input = &input[1..input.len() - 1];
        let mut input = input.chars();
        Route::parse_paths(&mut input)
    }

    fn parse_paths(input: &mut Chars) -> Route {
        Route::parse_route(input).0
    }

    fn parse_route(input: &mut dyn Iterator<Item = char>) -> (Route, char) {
        let mut res = Vec::new();

        loop {
            let (curr, separator) = Route::parse_elements(input);
            let mut separator = separator;
            if !curr.is_empty() {
                res.push(RouteElement::Directions(curr));
            }

            if separator == '(' {
                let mut subroutes = Route::parse_subroutes(input);

                if subroutes
                    .last()
                    .and_then(|r| if r.0.is_empty() { Some(()) } else { None })
                    .is_some()
                {
                    subroutes.pop();

                    for r in &mut subroutes {
                        let r = &mut r.0;
                        for dirs in r {
                            if let RouteElement::Directions(dirs) = dirs {
                                dirs.truncate(dirs.len() / 2);
                            }
                        }
                    }

                    let (next, new_sep) = Route::parse_route(input);
                    subroutes.push(next);
                    separator = new_sep;
                }

                res.push(RouteElement::Choice(subroutes));
            }

            if separator != '(' {
                return (Route(res), separator);
            }
        }
    }

    fn parse_subroutes(input: &mut dyn Iterator<Item = char>) -> Vec<Route> {
        let mut cur_choices: Vec<Route> = Vec::new();
        loop {
            let (inner, separator) = Route::parse_route(input);
            cur_choices.push(inner);

            if separator == ')' || separator == '$' {
                return cur_choices;
            }
        }
    }

    fn parse_elements(input: &mut dyn Iterator<Item = char>) -> (Vec<Direction>, char) {
        let mut res = Vec::new();

        for c in input {
            match c {
                'N' => res.push(Direction::N),
                'W' => res.push(Direction::W),
                'S' => res.push(Direction::S),
                'E' => res.push(Direction::E),
                _ => return (res, c),
            }
        }

        (res, '$')
    }
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("20");
    let input: &str = input.trim();

    println!("{}", first(&input));
    println!("{}", second(&input));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test11() {
        let input = "^WNE$";
        let max = first(input);
        assert_eq!(max, 3);
    }

    #[test]
    fn test12() {
        let input = "^ENWWW(NEEE|SSE(EE|N))$";
        let max = first(input);
        assert_eq!(max, 10);
    }

    #[test]
    fn test13() {
        let input = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
        let max = first(input);
        assert_eq!(max, 18);
    }

    #[test]
    fn test14() {
        let input = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
        let max = first(input);
        assert_eq!(max, 23);
    }

    #[test]
    fn test15() {
        let input = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
        let max = first(input);
        assert_eq!(max, 31);
    }
}
