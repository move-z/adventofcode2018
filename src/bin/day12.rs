use adventofcode2018::*;

use std::collections::HashMap;

fn first(initial: &str, rules: &HashMap<&str, char>) -> isize {
    let mut current = String::from(initial);
    let mut start_idx = 0;

    for _ in 1..=20 {
        if !current.starts_with("....") {
            current.insert_str(0, "....");
            start_idx -= 4;
        }
        if !current.ends_with("....") {
            current.push_str("....");
        }

        current = next_state(current, rules);
    }

    current.char_indices()
        .filter(|(_, c)| *c == '#')
        .map(|(i, _)| {
            i as isize + start_idx
        })
        .sum()
}

fn second(_input: &Vec<&str>) -> u32 {
    unimplemented!()
}

fn next_state(cur: String, rules: &HashMap<&str, char>) -> String {
    let mut new = String::from(&cur[..2]);
    for i in 0..cur.len() - 5 {
        let slice = &cur[i..i + 5];

        let pot = if let Some(rule_res) = rules.get(slice) {
            *rule_res
        } else {
            '.'
        };

        new.push(pot);
    }

    new
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("12");
    let input: Vec<&str> = input.trim().split("\n").collect();

    let initial = &input.get(0).unwrap()[15..];
    let mut rules = HashMap::new();
    &input[2..].iter().for_each(|&s| {
        let mut s = s.split(" => ").into_iter();
        let rule = s.next().unwrap();
        let res = s.next().unwrap().chars().next().unwrap();
        rules.insert(rule, res);
    });

    println!("{}", first(initial, &rules));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut rules = HashMap::new();
        rules.insert("...##", '#');
        rules.insert("..#..", '#');
        rules.insert(".#...", '#');
        rules.insert(".#.#.", '#');
        rules.insert(".#.##", '#');
        rules.insert(".##..", '#');
        rules.insert(".####", '#');
        rules.insert("#.#.#", '#');
        rules.insert("#.###", '#');
        rules.insert("##.#.", '#');
        rules.insert("##.##", '#');
        rules.insert("###..", '#');
        rules.insert("###.#", '#');
        rules.insert("####.", '#');
        assert_eq!(first("#..#.#..##......###...###", &rules), 325);
    }
}
