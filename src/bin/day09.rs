extern crate adventofcode2018;
#[macro_use]
extern crate lazy_static;

use std::cell::RefCell;
use std::rc::Rc;

use regex::Regex;

use adventofcode2018::*;

fn first(player_num: usize, max_value: usize) -> usize {
    let mut players = Vec::with_capacity(player_num);
    for _ in 0..player_num {
        players.push(0);
    }

    let mut board = List::new(0);

    for cur in 1..=max_value {
        if cur % 23 == 0 {
            board.reverse(7);
            let take = board.remove();

            let val = players.get_mut(cur % player_num).unwrap();
            *val = *val + take + cur;
        } else {
            board.advance(2);
            board.insert(cur);
        }
    }

    players.iter().max().unwrap().clone()
}

fn second(player_num: usize, max_value: usize) -> usize {
    first(player_num, max_value * 100)
}

struct Entry {
    value: usize,
    next: Option<Rc<RefCell<Entry>>>,
    prev: Option<Rc<RefCell<Entry>>>
}

struct List {
    head: Rc<RefCell<Entry>>
}

impl List {
    fn new(value: usize) -> List {
        let head = Entry { value, next: None, prev: None };
        let rc = Rc::new(RefCell::new(head));
        rc.borrow_mut().next = Some(rc.clone());
        rc.borrow_mut().prev = Some(rc.clone());
        List { head: rc }
    }

    fn insert(&mut self, value: usize) {
        let mut head = Entry { value, next: None, prev: None };
        let next = self.head.clone();
        let prev = next.borrow().prev.clone().unwrap();
        head.next = Some(next);
        head.prev = Some(prev);

        let head = Rc::new(RefCell::new(head));
        let next = self.head.clone();
        let prev = next.borrow().prev.clone().unwrap();
        next.borrow_mut().prev = Some(head.clone());
        prev.borrow_mut().next = Some(head.clone());

        self.head = head;
    }

    fn remove(&mut self) -> usize {
        let head = self.head.clone();

        let next = head.borrow().next.clone().unwrap();
        let prev = head.borrow().prev.clone().unwrap();
        next.borrow_mut().prev = Some(prev.clone());
        prev.borrow_mut().next = Some(next.clone());
        self.head = next.clone();

        let v = head.borrow().value;
        v
    }

    fn advance(&mut self, amount: usize) {
        let head = self.head.clone();
        self.head = head.borrow().next.clone().unwrap();
        if amount > 1 {
            self.advance(amount - 1);
        }
    }

    fn reverse(&mut self, amount: usize) {
        let head = self.head.clone();
        self.head = head.borrow().prev.clone().unwrap();
        if amount > 1 {
            self.reverse(amount - 1);
        }
    }
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

        println!("{}", second(players, max_value));
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
