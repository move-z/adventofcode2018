extern crate adventofcode2018;
#[macro_use]
extern crate lazy_static;

use regex::Regex;

use adventofcode2018::*;

fn first(player_num: usize, max_value: usize) -> usize {
    let mut players = Vec::with_capacity(4);
    for _ in 0..player_num {
        players.push(0);
    }

    let mut board = vec![0];

    let mut cur = 0;
    let mut cur_idx = 0;

    while cur < max_value {
        cur += 1;
        if cur % 23 == 0 {
            let take_idx = (board.len() + cur_idx - 7) % board.len();
            let take = board.remove(take_idx);

            let val = players.get_mut(cur % player_num).unwrap();
            *val = *val + take + cur;

            cur_idx = take_idx;
        } else {
            cur_idx = (cur_idx + 2) % board.len();
            board.insert(cur_idx, cur);
        }
    }

    players.iter().max().unwrap().clone()
}

fn second(player_num: usize, max_value: usize) -> usize {
    unimplemented!()
}

lazy_static! {
     static ref RE: Regex = Regex::new(r"^(\d+) players; last marble is worth (\d+) points$").unwrap();
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("09");
    let input = input.trim();
    if let Some(cap) = RE.captures(&input) {
        let players = parse_capture::<usize>(&cap, 1, "players").unwrap();
        let max_value = parse_capture::<usize>(&cap, 2, "max_value").unwrap();

        println!("{}", first(players, max_value));
    }

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1_9() {
        assert_eq!(first(9, 25), 32);
    }

    #[test]
    fn test1_10() {
        assert_eq!(first(10, 1618), 8317);
    }

    #[test]
    fn test1_13() {
        assert_eq!(first(13, 7999), 146373);
    }

    #[test]
    fn test1_17() {
        assert_eq!(first(17, 1104), 2764);
    }

    #[test]
    fn test1_21() {
        assert_eq!(first(21, 6111), 54718);
    }

    #[test]
    fn test1_30() {
        assert_eq!(first(30, 5807), 37305);
    }
}
