use adventofcode2018::*;
use itertools::Itertools;

fn first(input: usize) -> String {
    let mut recipes = vec![3, 7];
    let mut indices = (0, 1);

    while recipes.len() < input + 10 {
        indices = next(&mut recipes, indices);
    }

    recipes[input..input + 10].iter().join("")
}

fn second(input: &str) -> usize {
    let target = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();
    let mut recipes = vec![3, 7];
    let mut indices = (0, 1);

    loop {
        indices = next(&mut recipes, indices);
        if recipes.len() > target.len() {
            let startidx = recipes.len() - target.len() - 1;
            if recipes[startidx + 1..] == target[..] {
                return recipes.len() - target.len();
            } else if recipes[startidx..recipes.len() - 1] == target[..] {
                return recipes.len() - target.len() - 1;
            }
        }
    }
}

fn next(recipes: &mut Vec<u8>, indices: (usize, usize)) -> (usize, usize) {
    let mut n = recipes[indices.0] + recipes[indices.1];
    if n > 9 {
        recipes.push(1);
        n -= 10;
    }
    recipes.push(n);

    let next_idx = |idx| (idx + 1 + usize::from(recipes[idx])) % recipes.len();

    (next_idx(indices.0), next_idx(indices.1))
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("14");
    let input = input.trim();

    println!("{}", first(input.parse::<usize>().unwrap()));

    println!("{}", second(input));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test9() {
        assert_eq!(first(9), "5158916779");
    }

    #[test]
    fn test5() {
        assert_eq!(first(5), "0124515891");
    }

    #[test]
    fn test18() {
        assert_eq!(first(18), "9251071085");
    }

    #[test]
    fn test2018() {
        assert_eq!(first(2018), "5941429882");
    }

    #[test]
    fn test51589() {
        assert_eq!(second("51589"), 9);
    }

    #[test]
    fn test01245() {
        assert_eq!(second("01245"), 5);
    }
    #[test]
    fn test92510() {
        assert_eq!(second("92510"), 18);
    }
    #[test]
    fn test59414() {
        assert_eq!(second("59414"), 2018);
    }
}
