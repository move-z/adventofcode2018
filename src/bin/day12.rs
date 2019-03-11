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

        current = next_state(&current, rules);
    }

    current
        .char_indices()
        .filter(|(_, c)| *c == '#')
        .map(|(i, _)| i as isize + start_idx)
        .sum()
}

fn second(initial: &str, rules: &HashMap<&str, char>) -> i64 {
    let mut current = String::from(initial);
    let mut start_idx = 0;

    fn count(current: &str, start_idx: isize) -> i64 {
        current
            .char_indices()
            .filter(|(_, c)| *c == '#')
            .map(|(i, _)| i as i64 + start_idx as i64)
            .sum()
    }

    let mut visited = HashMap::new();

    const LIMIT: u64 = 50_000_000_000;
    let mut i = 0u64;
    while i <= LIMIT {
        visited.insert(
            current.trim_matches('.').to_owned(),
            (i, count(&current, start_idx)),
        );

        if !current.starts_with("....") {
            current.insert_str(0, "....");
            start_idx -= 4;
        }
        if !current.ends_with("....") {
            current.push_str("....");
        }

        current = next_state(&current, rules);
        i += 1;

        if let Some((prev_idx, prev_val)) = visited.get(current.trim_matches('.')) {
            let cur_val = count(&current, start_idx);
            let loop_size = i - prev_idx;
            let val_diff = cur_val - prev_val;
            let loops = (LIMIT - i) / loop_size;
            if i + loop_size * loops == LIMIT {
                return cur_val + val_diff * loops as i64;
            }
        }
    }

    count(&current, start_idx)
}

fn next_state(cur: &str, rules: &HashMap<&str, char>) -> String {
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
    let input: Vec<&str> = input.trim().split('\n').collect();

    let initial = &input[0][15..];
    let mut rules = HashMap::new();
    input[2..].iter().for_each(|&s| {
        let mut s = s.split(" => ");
        let rule = s.next().unwrap();
        let res = s.next().unwrap().chars().next().unwrap();
        rules.insert(rule, res);
    });

    println!("{}", first(initial, &rules));

    println!("{}", second(initial, &rules));

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
        assert_eq!(325, first("#..#.#..##......###...###", &rules));
    }
}
